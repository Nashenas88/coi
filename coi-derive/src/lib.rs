//! Coi-derive simplifies implementing the traits provided in the [coi] crate.
//!
//! [coi]: https://docs.rs/coi

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, Data, DeriveInput, Error, Expr, Fields, Result, Token, Type,
    Visibility,
};

struct Provides {
    vis: Visibility,
    ty: Type,
    with: Expr,
}

impl Parse for Provides {
    fn parse(input: ParseStream) -> Result<Self> {
        let vis = input.parse()?;
        let ty = input.parse()?;
        input.parse().and_then(|ident: Ident| {
            if ident.eq("with") {
                Ok(())
            } else {
                Err(Error::new(ident.span(), "expected `with`"))
            }
        })?;
        // FIXME(pfaria) we need to limit the kinds of exprs allowed here. Quite a few will
        // fail to compile
        let with = input.parse()?;
        Ok(Provides { vis, ty, with })
    }
}

struct InjectableField {
    name: Ident,
    ty: Type,
}

impl Parse for InjectableField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let _colon_separator: Token![:] = input.parse()?;
        let arc: Ident = input.parse()?;
        if !arc.eq("Arc") {
            return Err(Error::new_spanned(arc, "expected `Arc<...>`"));
        }
        let _left_angle: Token![<] = input.parse()?;
        let ty = input.parse()?;
        let _right_angle: Token![>] = input.parse()?;
        Ok(InjectableField { name, ty })
    }
}

/// Generates an impl for `Inject` and also generates a "Provider" struct with its own
/// `Provide` impl.
///
/// This derive proc macro impls `Inject` on the struct it modifies, and also processes two
/// attributes:
/// - `#[provides]` - Only one of these is allowed per `#[derive(Inject)]`. It takes the form
/// ```rust,ignore
/// #[provides(<vis> <ty> with <expr>)]
/// ```
/// It generates a provider struct with visibility `<vis>`
/// that impls `Provide` with an output type of `Arc<<ty>>`. It will construct `<ty>` with `<expr>`,
/// and all params to `<expr>` must match the struct fields marked with `#[inject]` (see the next
/// bullet item). `<vis>` must match the visibility of `<ty>` or you will get code that might not
/// compile.
/// - `#[inject]` - All fields marked `#[inject]` are resolved in the `provide` fn described above.
/// Given a field `<field_name>: <field_ty>`, this attribute will cause the following resolution to
/// be generated:
/// ```rust,ignore
/// let <field_name> = Container::resolve::<<field_ty>>(conainer, "<field_name>");
/// ```
/// Because of this, it's important that the field name MUST match the string that's used to
/// register the provider in the `ContainerBuilder`.
///
/// ## Examples
///
/// Private trait and no dependencies
/// ```rust
/// use coi::Inject;
/// use coi_derive::Inject;
/// trait Priv: Inject {}
///
/// #[derive(Inject)]
/// #[provides(dyn Priv with SimpleStruct)]
/// # pub
/// struct SimpleStruct;
///
/// impl Priv for SimpleStruct {}
/// ```
///
/// Public trait and dependency
/// ```rust
/// use coi::Inject;
/// use coi_derive::Inject;
/// use std::sync::Arc;
/// pub trait Pub: Inject {}
/// pub trait Dependency: Inject {}
///
/// #[derive(Inject)]
/// #[provides(pub dyn Pub with NewStruct::new(dependency))]
/// # pub
/// struct NewStruct {
///     #[inject]
///     dependency: Arc<dyn Dependency>,
/// }
///
/// impl NewStruct {
///     fn new(dependency: Arc<dyn Dependency>) -> Self {
///         Self {
///             dependency
///         }
///     }
/// }
///
/// impl Pub for NewStruct {}
/// ```
///
/// Struct injection
/// ```rust
/// use coi::Inject;
/// use coi_derive::Inject;
/// #[derive(Inject)]
/// #[provides(pub InjectableStruct with InjectableStruct)]
/// # pub
/// struct InjectableStruct;
/// ```
///
/// If you need some form of constructor fn that takes arguments that are not injected, then you
/// need to manually implement the `Provide` trait, and this derive will not be usable.
#[proc_macro_derive(Inject, attributes(provides, inject))]
pub fn inject_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data_struct = match input.data {
        Data::Struct(data_struct) => data_struct,
        _ => {
            return Error::new_spanned(input, "#[derive(Inject)] only supports structs")
                .to_compile_error()
                .into()
        }
    };
    let provider = format_ident!("{}Provider", input.ident);
    let attr = match input.attrs.into_iter().find(|attr| {
        attr.path
            .segments
            .first()
            .map(|p| p.ident.eq("provides"))
            .unwrap_or(false)
    }) {
        None => {
            return Error::new_spanned(
                input.ident,
                "#[derive(Inject)] requires a `provides` attribute",
            )
            .to_compile_error()
            .into()
        }
        Some(attr) => attr,
    };

    let args: Vec<_> = match data_struct.fields {
        Fields::Named(named_fields) => {
            let injectable_fields: Vec<_> = named_fields
                .named
                .into_iter()
                .filter(|field| field.attrs.iter().any(|attr| attr.path.is_ident("inject")))
                .map(|field| -> Result<InjectableField> {
                    let field_name = field.ident.unwrap();
                    let field_ty = field.ty;
                    Ok(parse_quote! {
                        #field_name: #field_ty
                    })
                })
                .collect();

            if injectable_fields.iter().any(|f| f.is_err()) {
                return injectable_fields
                    .into_iter()
                    .fold(Ok(()), |acc, f| match f {
                        Ok(_) => acc,
                        Err(e) => match acc {
                            Ok(()) => Err(e),
                            Err(mut e2) => {
                                e2.combine(e);
                                Err(e2)
                            }
                        },
                    })
                    .unwrap_err()
                    .to_compile_error()
                    .into();
            }

            injectable_fields.into_iter().map(Result::unwrap).collect()
        }
        // FIXME(pfaria) add support for unnamed fields by allowing the name to be
        // specified as part of the attribute params
        Fields::Unnamed(_) => {
            return Error::new_spanned(data_struct.fields, "unnamed fields cannot be injected")
                .to_compile_error()
                .into()
        }
        Fields::Unit => vec![],
    };
    let container = format_ident!("{}", if args.is_empty() { "_" } else { "container" });

    let (resolve, keys): (Vec<_>, Vec<_>) = args
        .into_iter()
        .map(|field| {
            let ident = field.name;
            let ty = field.ty;
            let key = format!("{}", ident);
            (
                quote! {
                    let #ident = #container.resolve::<#ty>(#key)?;
                },
                key,
            )
        })
        .unzip();

    let attr2 = attr.clone();
    let mut token_iter = attr2.tokens.into_iter();
    let provides = match token_iter.next() {
        Some(TokenTree::Group(group)) => {
            let token_stream = TokenStream::from(group.stream());
            parse_macro_input!(token_stream as Provides)
        }
        Some(s) => {
            return Error::new_spanned(s, "expected `(ty with expr)`")
                .to_compile_error()
                .into()
        }
        _ => {
            return Error::new_spanned(attr, "expected `(ty with expr)`")
                .to_compile_error()
                .into()
        }
    };
    let vis = provides.vis;
    let ty = provides.ty;
    let provides_with = provides.with;

    if let Some(s) = token_iter.next() {
        return Error::new_spanned(
            s,
            "Only expected the format #[provides `Interface` with `provider`]",
        )
        .to_compile_error()
        .into();
    }

    let input_ident = input.ident;

    let dependencies_fn = if cfg!(feature = "debug") {
        vec![{
            quote! {
                fn dependencies(
                    &self
                ) -> Vec<&'static str> {
                    vec![
                        #( #keys, )*
                    ]
                }
            }
        }]
    } else {
        vec![]
    };

    let expanded = quote! {
        impl Inject for #input_ident {}

        #vis struct #provider;

        impl ::coi::Provide for #provider {
            type Output = #ty;

            fn provide(
                &self,
                #container: &::coi::Container,
            ) -> ::coi::Result<::std::sync::Arc<Self::Output>> {
                #( #resolve )*
                Ok(::std::sync::Arc::new(#provides_with) as ::std::sync::Arc<#ty>)
            }

            #( #dependencies_fn )*
        }
    };
    TokenStream::from(expanded)
}
