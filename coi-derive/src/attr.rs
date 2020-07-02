use crate::ctxt::Ctxt;
use crate::symbol::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    self,
    parse::{Parse, ParseStream},
    parse_quote, Data, DataEnum, DataUnion, DeriveInput, Error, Expr, Fields, Ident, ItemStruct,
    Meta::{List, NameValue, Path as MetaPath},
    NestedMeta::{Lit, Meta},
    Path, Token, Type, Visibility,
};

struct Attr<'c, T> {
    cx: &'c Ctxt,
    name: Symbol,
    tokens: TokenStream,
    value: Option<T>,
}

impl<'c, T> Attr<'c, T> {
    fn none(cx: &'c Ctxt, name: Symbol) -> Self {
        Self {
            cx,
            name,
            tokens: TokenStream::new(),
            value: None,
        }
    }

    fn set<A: ToTokens>(&mut self, obj: A, value: T) {
        let tokens = obj.into_token_stream();

        if self.value.is_some() {
            self.cx.push(syn::Error::new_spanned(
                tokens,
                format!("duplicate coi attribute `{}`", self.name),
            ));
        } else {
            self.tokens = tokens;
            self.value = Some(value);
        }
    }

    fn get(self) -> Option<T> {
        self.value
    }
}

pub struct Container {
    coi_path: Option<syn::Path>,
    pub providers: Vec<Provides>,
    pub injected: Vec<InjectableField>,
}

macro_rules! collect_injected_fields {
    (@retain $field:ident, &) => {};
    (@retain $field:ident, &mut) => {
        $field.attrs.retain(|attr| attr.path != COI);
    };
    ($cx:ident, $injected:ident, $fields:expr, $is_provides:expr, $($ref_kind:tt)+) => {
        match $($ref_kind)+ $fields {
            Fields::Named(named_fields) => {
                for field in $($ref_kind)+ named_fields.named {
                    for meta_item in field
                        .attrs
                        .iter()
                        .flat_map(|attr| {
                            if !$is_provides {
                                $cx.push(Error::new_spanned(
                                    attr,
                                    "coi field attribute inject only allowed when deriving Inject",
                                ));
                            }

                            get_coi_meta_items($cx, attr)
                        })
                        .flatten()
                    {
                        match &meta_item {
                            // Parse `#[coi(inject)]`
                            Meta(MetaPath(word)) if word == INJECT => {
                                let ident = field.ident.as_ref().cloned().unwrap();
                                let ty = field.ty.clone();
                                $injected.push(parse_quote::parse::<InjectableField>(
                                    quote! {#ident: #ty},
                                ));
                            }
                            // Parse `#[coi(inject = "...")]`
                            Meta(NameValue(m)) if m.path == INJECT => {
                                let ident = if let Some(ident) =
                                    get_ident_from_lit($cx, INJECT, INJECT, &m.lit)
                                {
                                    ident
                                } else {
                                    continue;
                                };
                                let ty = field.ty.clone();
                                $injected.push(parse_quote::parse::<InjectableField>(
                                    quote! {#ident: #ty},
                                ));
                            }
                            Meta(meta_item) => {
                                let path = meta_item
                                    .path()
                                    .into_token_stream()
                                    .to_string()
                                    .replace(' ', "");
                                $cx.push(Error::new_spanned(
                                    meta_item.path(),
                                    format!("unknown coi field attribute `{}`", path),
                                ));
                            }
                            Lit(lit) => $cx.push(Error::new_spanned(
                                lit,
                                "unexpected literal in coi field attribute",
                            )),
                        }
                    }

                    collect_injected_fields!(@retain field, $($ref_kind)+);
                }
            }
            Fields::Unnamed(unnamed_fields) => {
                for field in $($ref_kind)+ unnamed_fields.unnamed {
                    for meta_item in field
                        .attrs
                        .iter()
                        .flat_map(|attr| get_coi_meta_items($cx, attr))
                        .flatten()
                    {
                        match &meta_item {
                                // Parse `#[coi(inject = "...")]`
                                Meta(NameValue(m)) if m.path == INJECT => {
                                    let ident = if let Some(ident) =
                                        get_ident_from_lit($cx, INJECT, INJECT, &m.lit)
                                    {
                                        ident
                                    } else {
                                        continue;
                                    };
                                    let ty = field.ty.clone();
                                    $injected.push(parse_quote::parse::<InjectableField>(quote!{#ident: #ty}));
                                },
                                // Reject `#[coi(inject)]`
                                Meta(MetaPath(word)) if word == INJECT => {
                                    $cx.push(Error::new_spanned(word, "unnamed fields require a named injection, `#[coi(inject = \"...\")]`"))
                                },
                                Meta(meta_item)  => {
                                    let path = meta_item
                                        .path()
                                        .into_token_stream()
                                        .to_string()
                                        .replace(' ', "");
                                    $cx.push(Error::new_spanned(
                                        meta_item.path(),
                                        format!("unknown coi field attribute `{}`", path),
                                    ));
                                },
                                Lit(lit) => $cx.push(Error::new_spanned(lit, "unexpected literal in coi field attribute"))
                            }
                    }

                    collect_injected_fields!(@retain field, $($ref_kind)+);
                }
            }
            Fields::Unit => {}
        };
    }
}

impl Container {
    pub fn from_attr_and_item(
        cx: &Ctxt,
        provides: Provides,
        item: &mut ItemStruct,
        is_provides: bool,
    ) -> Option<Self> {
        let coi_path = Attr::none(cx, CRATE);
        let mut injected = vec![];
        collect_injected_fields!(cx, injected, item.fields, is_provides, &mut);

        Some(Container {
            coi_path: coi_path.get(),
            providers: vec![provides],
            injected,
        })
    }

    pub fn from_ast(cx: &Ctxt, item: &DeriveInput, is_provides: bool) -> Option<Self> {
        let mut coi_path = Attr::none(cx, CRATE);
        let mut providers = vec![];

        let coi_attrs: Vec<_> = item
            .attrs
            .iter()
            .filter_map(|attr| get_coi_attrs(cx, attr).map(|a| (a, attr)))
            .collect();

        let has_multiple_unnamed_providers = coi_attrs.iter().fold(0, |acc, (coi_attr, _)| {
            if let ContainerAttr::Provides(Provides { name: None, .. }) = coi_attr {
                acc + 1
            } else {
                acc
            }
        }) > 1;
        for (coi_attr, attr) in coi_attrs {
            match coi_attr {
                ContainerAttr::Provides(p) => {
                    if has_multiple_unnamed_providers && p.name.is_none() {
                        cx.push(Error::new_spanned(attr, "expected `#[coi::coi(provides <type> as <unique name> with <expr>)]` when multiple provides field attributes are supplied"))
                    }

                    providers.push(p);
                }
                ContainerAttr::Crate(c) => coi_path.set(attr, c.path),
            }
        }

        let data_struct = match &item.data {
            Data::Struct(data_struct) => data_struct,
            Data::Enum(DataEnum { enum_token, .. }) => {
                cx.push(Error::new(enum_token.span, "expected struct item"));
                return None;
            }
            Data::Union(DataUnion { union_token, .. }) => {
                cx.push(Error::new(union_token.span, "expected struct item"));
                return None;
            }
        };

        let mut injected = vec![];
        collect_injected_fields!(cx, injected, data_struct.fields, is_provides, &);

        Some(Container {
            coi_path: coi_path.get(),
            providers,
            injected,
        })
    }

    pub fn coi_path(&self) -> Path {
        self.coi_path
            .as_ref()
            .cloned()
            .unwrap_or_else(|| COI.as_ident().into())
    }
}

pub fn get_ident_from_lit(
    cx: &Ctxt,
    attr_name: Symbol,
    meta_item_name: Symbol,
    lit: &syn::Lit,
) -> Option<Ident> {
    if let syn::Lit::Str(lit) = lit {
        lit.parse().map_err(|e| cx.push(e)).ok()
    } else {
        cx.push(Error::new_spanned(
            lit,
            format!(
                "expected coi {} attribute to be a string: `{} = \"...\"`",
                attr_name, meta_item_name
            ),
        ));
        None
    }
}

fn get_coi_attrs(cx: &Ctxt, attr: &syn::Attribute) -> Option<ContainerAttr> {
    if attr.path != COI {
        return None;
    }

    attr.parse_args()
        .map_err(|e| {
            cx.push(e);
        })
        .ok()
}

fn get_coi_meta_items(cx: &Ctxt, attr: &syn::Attribute) -> Result<Vec<syn::NestedMeta>, ()> {
    if attr.path != COI {
        return Ok(Vec::new());
    }

    match attr.parse_meta() {
        Ok(List(meta)) => Ok(meta.nested.into_iter().collect()),
        Ok(other) => {
            cx.push(Error::new_spanned(other, "expected #[coi(...)]"));
            Err(())
        }
        Err(err) => {
            cx.push(err);
            Err(())
        }
    }
}

#[allow(clippy::large_enum_variant)]
enum ContainerAttr {
    Provides(Provides),
    Crate(Crate),
}

impl Parse for ContainerAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(|_| PROVIDES.as_ident()) {
            <Provides as Parse>::parse(input).map(ContainerAttr::Provides)
        } else if lookahead.peek(|_| CRATE.as_ident()) {
            <Crate as Parse>::parse(input).map(ContainerAttr::Crate)
        } else {
            Err(Error::new(
                input.span(),
                "expected one of `crate` or `provides`",
            ))
        }
    }
}

pub struct Crate {
    pub path: Path,
}

impl Parse for Crate {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().and_then(|ident: Ident| {
            if ident.eq("crate") {
                Ok(())
            } else {
                Err(Error::new(ident.span(), "expected `crate`"))
            }
        })?;
        let _eq: Token![=] = input.parse()?;
        let path = input.parse()?;
        Ok(Crate { path })
    }
}

pub struct Provides {
    pub vis: Visibility,
    pub ty: Type,
    pub with: Expr,
    pub name: Option<Ident>,
}

impl Provides {
    pub fn name_or(&self, item: &Ident) -> Ident {
        self.name
            .as_ref()
            .cloned()
            .unwrap_or(format_ident!("{}Provider", item))
    }
}

impl Parse for Provides {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().and_then(|ident: Ident| {
            if ident == PROVIDES {
                Ok(())
            } else {
                Err(Error::new(ident.span(), "expected `provides`"))
            }
        })?;

        let vis = input.parse()?;
        let ty = input.parse()?;

        let lookahead = input.lookahead1();
        let name = if lookahead.peek(Token![as]) {
            let _as: Token![as] = input.parse().unwrap();
            Some(input.parse()?)
        } else {
            None
        };

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
        Ok(Provides {
            vis,
            ty,
            with,
            name,
        })
    }
}

pub struct InjectableField {
    pub name: Ident,
    pub ty: Type,
}

impl Parse for InjectableField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _colon_separator: Token![:] = input.parse()?;
        let arc: Ident = input.parse()?;
        if arc != ARC {
            return Err(Error::new_spanned(
                arc,
                "coi field attribute inject expects `Arc<...>` type",
            ));
        }

        let _left_angle: Token![<] = input.parse()?;
        let ty = input.parse()?;
        let _right_angle: Token![>] = input.parse()?;
        Ok(InjectableField { name, ty })
    }
}
