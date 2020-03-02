//! Coi-derive simplifies implementing the traits provided in the [coi] crate.
//!
//! [coi]: https://docs.rs/coi

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Error};

mod attr;
mod ctxt;
mod symbol;

use crate::attr::Container;
use crate::ctxt::Ctxt;

/// Generates an impl for `Inject` and also generates a "Provider" struct with its own
/// `Provide` impl.
///
/// This derive proc macro impls `Inject` on the struct it modifies, and also processes #[coi(...)]
/// attributes:
/// - `#[coi(provides ...)]` - It takes the form
/// ```rust,ignore
/// #[coi(provides <vis> <ty> with <expr>)]
/// ```
/// or the form
/// ```rust,ignore
/// #[coi(provides <vis> <ty> as <name> with <expr>)]
/// ```
/// The latter form *must* be used when generating multiple providers for a single type. This might
/// be useful if you have multiple trait implementations for one struct and want to provide separate
/// unique instances for each trait in the container. That use case might be more common with mocks
/// in unit tests rather than in production code.
/// 
/// It generates a provider struct with visibility `<vis>`
/// that impls `Provide` with an output type of `Arc<<ty>>`. It will construct `<ty>` with `<expr>`,
/// and all params to `<expr>` must match the struct fields marked with `#[coi(inject)]` (see the
/// next bullet item). `<vis>` must match the visibility of `<ty>` or you will get code that might
/// not compile. If `<name>` is not provided, the struct name will be used and `Provider` will be
/// appended to it.
/// - `#[coi(inject)]` - All fields marked `#[coi(inject)]` are resolved in the `provide` fn
/// described above.
/// Given a field `<field_name>: <field_ty>`, this attribute will cause the following resolution to
/// be generated:
/// ```rust,ignore
/// let <field_name> = Container::resolve::<<field_ty>>(container, "<field_name>");
/// ```
/// Because of this, it's important that the field name *must* match the string that's used to
/// register the provider in the `ContainerBuilder`.
///
/// ## Examples
///
/// Private trait and no dependencies
/// ```rust
/// use coi::Inject;
/// # use coi_derive::Inject;
/// trait Priv: Inject {}
///
/// #[derive(Inject)]
/// #[coi(provides dyn Priv with SimpleStruct)]
/// # pub
/// struct SimpleStruct;
///
/// impl Priv for SimpleStruct {}
/// ```
///
/// Public trait and dependency
/// ```rust
/// use coi::Inject;
/// # use coi_derive::Inject;
/// use std::sync::Arc;
/// pub trait Pub: Inject {}
/// pub trait Dependency: Inject {}
///
/// #[derive(Inject)]
/// #[coi(provides pub dyn Pub with NewStruct::new(dependency))]
/// # pub
/// struct NewStruct {
///     #[coi(inject)]
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
/// # use coi_derive::Inject;
///
/// #[derive(Inject)]
/// #[coi(provides pub InjectableStruct with InjectableStruct)]
/// # pub
/// struct InjectableStruct;
/// ```
///
/// Unnamed fields
/// ```rust
/// use coi::Inject;
/// # use coi_derive::Inject;
/// use std::sync::Arc;
///
/// #[derive(Inject)]
/// #[coi(provides Dep1 with Dep1)]
/// struct Dep1;
///
/// #[derive(Inject)]
/// #[coi(provides Impl1 with Impl1(dep1))]
/// struct Impl1(#[coi(inject = "dep1")] Arc<Dep1>);
/// ```
///
/// Generics
/// ```rust
/// use coi::{container, Inject};
/// # use coi_derive::Inject;
///
/// #[derive(Inject)]
/// #[coi(provides Impl1<T> with Impl1::<T>::new())]
/// struct Impl1<T>(T)
/// where
///     T: Default;
///
/// impl<T> Impl1<T>
/// where
///     T: Default,
/// {
///     fn new() -> Self {
///         Self(Default::default())
///     }
/// }
///
/// fn build_container() {
///   // Take note that these providers have to be constructed
///   // with explicit types.
///   let impl1_provider = Impl1Provider::<bool>::new();
///   let container = container! {
///       impl1 => impl1_provider,
///   };
///   let _bool_impl = container
///       .resolve::<Impl1<bool>>("impl1")
///       .expect("Should exist");
/// }
///
/// # build_container();
/// ```
///
/// If you need some form of constructor fn that takes arguments that are not injected, then you
/// might be able to use the [`coi::Provide`] derive. If that doesn't fit your use case, you'll
/// need to manually implement `Provide`.
/// 
/// [`coi::Provide`]: derive.Provide.html
#[proc_macro_derive(Inject, attributes(coi))]
pub fn inject_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let cx = Ctxt::new();
    let container = Container::from_ast(&cx, &input, true);
    if let Err(e) = cx.check() {
        return to_compile_errors(e).into();
    }
    let container = container.unwrap();

    let has_generics = !input.generics.params.is_empty();
    let generic_params = input.generics.params;
    let generics = if has_generics {
        quote! {
            <#generic_params>
        }
    } else {
        quote! {}
    };

    let coi = container.coi_path();
    let where_clause = input
        .generics
        .where_clause
        .map(|w| {
            let t: Vec<_> = generic_params.iter().collect();
            quote! { #w #(, #t: Send + Sync + 'static )* }
        })
        .unwrap_or_default();
    if container.providers.is_empty() {
        let ident = input.ident;
        return quote! {
            impl #generics #coi::Inject for #ident #generics #where_clause {}
        }
        .into();
    }

    let container_ident = format_ident!(
        "{}",
        if container.injected.is_empty() {
            "_"
        } else {
            "container"
        }
    );
    let (resolve, keys): (Vec<_>, Vec<_>) = container
        .injected
        .into_iter()
        .map(|field| {
            let ident = field.name;
            let ty = field.ty;
            let key = format!("{}", ident);
            (
                quote! {
                    let #ident = #container_ident.resolve::<#ty>(#key)?;
                },
                key,
            )
        })
        .unzip();
    let input_ident = input.ident;

    let dependencies_fn = if cfg!(feature = "debug") {
        vec![quote! {
            fn dependencies(&self) -> Vec<&'static str> {
                vec![
                    #( #keys, )*
                ]
            }
        }]
    } else {
        vec![]
    };

    let provider_fields = if has_generics {
        let tys: Vec<_> = generic_params.iter().cloned().collect();
        quote! {
            (
                #( ::std::marker::PhantomData<#tys> )*
            )
        }
    } else {
        quote! {}
    };

    let phantom_data: Vec<_> = generic_params
        .iter()
        .map(|_| quote! {::std::marker::PhantomData})
        .collect();

    let provider_impls = if !phantom_data.is_empty() {
        container
            .providers
            .iter()
            .map(|p| {
                let provider = p.name_or(&input_ident);
                let vis = &p.vis;
                quote! {
                    impl #generics #provider #generics #where_clause {
                        #vis fn new() -> Self {
                            Self(#( #phantom_data )*)
                        }
                    }
                }
            })
            .collect()
    } else {
        vec![]
    };

    let constructed_provides: Vec<_> = container
        .providers
        .into_iter()
        .map(|p| {
            let provider = p.name_or(&input_ident);
            let vis = p.vis;
            let ty = p.ty;
            let provides_with = p.with;

            quote! {
                #vis struct #provider #generics #provider_fields #where_clause;

                impl #generics #coi::Provide for #provider #generics #where_clause {
                    type Output = #ty;

                    fn provide(
                        &self,
                        #container_ident: &#coi::Container,
                    ) -> #coi::Result<::std::sync::Arc<Self::Output>> {
                        #( #resolve )*
                        Ok(::std::sync::Arc::new(#provides_with) as ::std::sync::Arc<#ty>)
                    }

                    #( #dependencies_fn )*
                }
            }
        })
        .collect();

    let expanded = quote! {
        impl #generics #coi::Inject for #input_ident #generics #where_clause {}

        #( #provider_impls )*
        #( #constructed_provides )*
    };
    TokenStream::from(expanded)
}

/// Generates an impl for `Provide` and also generates a "Provider" struct with its own
/// `Provide` impl.
///
/// This derive proc macro impls `Provide` on the struct it modifies, and also processes #[coi(...)]
/// attributes:
/// - `#[coi(provides ...)]` - It takes the form
/// ```rust,ignore
/// #[coi(provides <vis> <ty> with <expr>)]
/// ```
/// 
/// Multiple `provides` attributes are not allowed since this is for a specific `Provide` impl and
/// not for the resolved type.
/// 
/// It generates a provider struct with visibility `<vis>`
/// that impls `Provide` with an output type of `Arc<<ty>>`. It will construct `<ty>` with `<expr>`,
/// and all params to `<expr>` must match the struct fields marked with `#[coi(inject)]` (see the
/// next bullet item). `<vis>` must match the visibility of `<ty>` or you will get code that might
/// not compile. If `<name>` is not provided, the struct name will be used and `Provider` will be
/// appended to it.
///
/// ## Examples
///
/// Private trait and no dependencies
/// ```rust
/// use coi::{Inject, Provide};
/// # use coi_derive::{Inject, Provide};
/// trait Priv: Inject {}
/// 
/// #[derive(Inject)]
/// # pub
/// struct SimpleStruct {
///     data: u32
/// }
/// 
/// impl SimpleStruct {
///     fn new(data: u32) -> Self {
///         Self { data }
///     }
/// }
///
/// impl Priv for SimpleStruct {}
///
/// #[derive(Provide)]
/// #[coi(provides dyn Priv with SimpleStruct::new(self.data))]
/// struct SimpleStructProvider {
///     data: u32,
/// }
/// 
/// impl SimpleStructProvider {
///     fn new(data: u32) -> Self {
///         Self { data: 42 }
///     }
/// }
/// ```
#[proc_macro_derive(Provide, attributes(coi))]
pub fn provide_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let cx = Ctxt::new();
    let container = Container::from_ast(&cx, &input, false);
    if let Err(e) = cx.check() {
        return to_compile_errors(e).into();
    }
    let container = container.unwrap();

    let provider = input.ident.clone();
    let has_generics = !input.generics.params.is_empty();
    let generic_params = input.generics.params;
    let generics = if has_generics {
        quote! {
            <#generic_params>
        }
    } else {
        quote! {}
    };
    let where_clause = input
        .generics
        .where_clause
        .map(|w| {
            let t: Vec<_> = generic_params.iter().collect();
            quote! { #w #(, #t: Send + Sync + 'static )* }
        })
        .unwrap_or_default();

    let dependencies_fn = if cfg!(feature = "debug") {
        vec![{
            quote! {
                fn dependencies(
                    &self
                ) -> Vec<&'static str> {
                    vec![]
                }
            }
        }]
    } else {
        vec![]
    };

    let coi = container.coi_path();
    let expanded: Vec<_> = container
        .providers
        .into_iter()
        .map(|p| {
            let ty = p.ty;
            let provides_with = p.with;
            quote! {
                impl #generics #coi::Provide for #provider #generics #where_clause {
                    type Output = #ty;

                    fn provide(
                        &self,
                        _: &#coi::Container,
                    ) -> #coi::Result<::std::sync::Arc<Self::Output>> {
                        Ok(::std::sync::Arc::new(#provides_with) as ::std::sync::Arc<#ty>)
                    }

                    #( #dependencies_fn )*
                }
            }
        })
        .collect();
    TokenStream::from(quote! {
        #( #expanded )*
    })
}

fn to_compile_errors(errors: Vec<Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
