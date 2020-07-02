use crate::attr::{Container, Provides};
use crate::ctxt::Ctxt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Error, ItemStruct};

pub(crate) fn provides_attr(
    provides: Provides,
    mut input: ItemStruct,
    debug_feature: bool,
) -> TokenStream {
    let cx = Ctxt::new();
    let container = Container::from_attr_and_item(&cx, provides, &mut input, true);
    if let Err(e) = cx.check() {
        return to_compile_errors(e);
    }
    let container = container.unwrap();

    let has_generics = !input.generics.params.is_empty();
    let generic_params = input.generics.params.clone();
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
        .clone()
        .map(|w| {
            let t: Vec<_> = generic_params.iter().collect();
            quote! { #w #(, #t: Send + Sync + 'static )* }
        })
        .unwrap_or_default();
    if container.providers.is_empty() {
        return quote! { #input };
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
    let input_ident = input.ident.clone();

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

            let dependencies_impl = if debug_feature {
                vec![quote! {
                    impl #generics #coi::Dependencies for #provider #generics #where_clause {
                        fn dependencies(&self) -> &'static[&'static str] {
                            &[
                                #( #keys, )*
                            ]
                        }
                    }
                }]
            } else {
                vec![]
            };

            quote! {
                #input
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
                }

                #( #dependencies_impl )*
            }
        })
        .collect();

    quote! {
        #( #provider_impls )*
        #( #constructed_provides )*
    }
}

pub(crate) fn provides_derive(input: DeriveInput, debug_feature: bool) -> TokenStream {
    let cx = Ctxt::new();
    let container = Container::from_ast(&cx, &input, false);
    if let Err(e) = cx.check() {
        return to_compile_errors(e);
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

    let coi = container.coi_path();
    let dependencies_impl = if debug_feature {
        vec![{
            quote! {
                impl #generics #coi::Dependencies for #provider #generics #where_clause {
                    fn dependencies(&self) -> &'static[&'static str] {
                        &[]
                    }
                }
            }
        }]
    } else {
        vec![]
    };

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
                }

                #( #dependencies_impl )*
            }
        })
        .collect();
    quote! {
        #( #expanded )*
    }
}

fn to_compile_errors(errors: Vec<Error>) -> TokenStream {
    let compile_errors = errors.iter().map(Error::to_compile_error);
    quote!(#(#compile_errors)*)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use syn::parse::{Parse, Parser};

    #[test]
    fn provides_attr_simple_dependency_no_debug() {
        let attr = TokenStream::from(quote! {
            provides pub dyn Pub + Send + Sync with NewStruct::new(dependency)
        });
        let attr = Provides::parse.parse2(attr).unwrap();
        let item = TokenStream::from(quote! {
            struct NewStruct {
                #[coi(inject)]
                dependency: Arc<dyn Dependency + Send + Sync>,
            }
        });
        let item = ItemStruct::parse.parse2(item).unwrap();
        let ts = provides_attr(attr, item, false);
        assert_eq!(ts.to_string(), TokenStream::from(quote! {
            struct NewStruct {
                dependency: Arc<dyn Dependency + Send + Sync>,
            }

            pub struct NewStructProvider;

            impl coi::Provide for NewStructProvider {
                type Output = dyn Pub + Send + Sync;

                fn provide(&self, container: &coi::Container, ) -> coi::Result<::std::sync::Arc<Self::Output>> {
                    let dependency = container.resolve::<dyn Dependency + Send + Sync>("dependency")?;
                    Ok(::std::sync::Arc::new(NewStruct::new(dependency)) as ::std::sync::Arc<dyn Pub + Send + Sync>)
                }
            }
        }).to_string());
    }

    #[test]
    fn provides_attr_simple_dependency_with_debug() {
        let attr = TokenStream::from(quote! {
            provides pub dyn Pub + Send + Sync with NewStruct::new(dependency)
        });
        let attr = Provides::parse.parse2(attr).unwrap();
        let item = TokenStream::from(quote! {
            struct NewStruct {
                #[coi(inject)]
                dependency: Arc<dyn Dependency + Send + Sync>,
            }
        });
        let item = ItemStruct::parse.parse2(item).unwrap();
        let ts = provides_attr(attr, item, true);
        assert_eq!(ts.to_string(), TokenStream::from(quote! {
            struct NewStruct {
                dependency: Arc<dyn Dependency + Send + Sync>,
            }

            pub struct NewStructProvider;

            impl coi::Provide for NewStructProvider {
                type Output = dyn Pub + Send + Sync;

                fn provide(&self, container: &coi::Container, ) -> coi::Result<::std::sync::Arc<Self::Output>> {
                    let dependency = container.resolve::<dyn Dependency + Send + Sync>("dependency")?;
                    Ok(::std::sync::Arc::new(NewStruct::new(dependency)) as ::std::sync::Arc<dyn Pub + Send + Sync>)
                }
            }

            impl coi::Dependencies for NewStructProvider {
                fn dependencies(&self) -> &'static[&'static str] {
                    &["dependency",]
                }
            }
        }).to_string());
    }

    #[test]
    fn provides_derive_simple_dependency_no_debug() {
        let input = TokenStream::from(quote! {
            #[coi(provides pub dyn Pub + Send + Sync with NewStruct::new(self.data))]
            struct NewStructProvider {
                data: u32,
            }
        });
        let input = DeriveInput::parse.parse2(input).unwrap();
        let ts = provides_derive(input, false);
        assert_eq!(ts.to_string(), TokenStream::from(quote! {
            impl coi::Provide for NewStructProvider {
                type Output = dyn Pub + Send + Sync;

                fn provide (&self, _: &coi::Container, ) -> coi::Result<::std::sync::Arc<Self::Output>> {
                    Ok(::std::sync::Arc::new(NewStruct::new(self.data)) as ::std::sync::Arc<dyn Pub + Send + Sync>)
                }
            }
        }).to_string());
    }

    #[test]
    fn provides_derive_simple_dependency_with_debug() {
        let input = TokenStream::from(quote! {
            #[coi(provides pub dyn Pub + Send + Sync with NewStruct::new(self.data))]
            struct NewStructProvider {
                data: u32,
            }
        });
        let input = DeriveInput::parse.parse2(input).unwrap();
        let ts = provides_derive(input, true);
        assert_eq!(ts.to_string(), TokenStream::from(quote! {
            impl coi::Provide for NewStructProvider {
                type Output = dyn Pub + Send + Sync;

                fn provide (&self, _: &coi::Container, ) -> coi::Result<::std::sync::Arc<Self::Output>> {
                    Ok(::std::sync::Arc::new(NewStruct::new(self.data)) as ::std::sync::Arc<dyn Pub + Send + Sync>)
                }
            }

            impl coi::Dependencies for NewStructProvider {
                fn dependencies(&self) -> &'static[&'static str] {
                    &[]
                }
            }
        }).to_string());
    }
}
