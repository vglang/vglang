use quote::{quote, ToTokens};
use syn::{parse_macro_input, GenericArgument, ItemEnum, PathSegment, Type};

fn parse_generic_type(seg: &PathSegment) -> &Type {
    match &seg.arguments {
        syn::PathArguments::AngleBracketed(args) => {
            match args.args.first().expect("s-expr derive inner error.") {
                GenericArgument::Type(t) => {
                    return t;
                }
                _ => {
                    panic!("s-expr derive inner error.")
                }
            }
        }
        _ => panic!("s-expr derive inner error."),
    }
}

pub fn derive_data(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let item = parse_macro_input!(item as ItemEnum);

    let ItemEnum {
        attrs: _,
        vis: _,
        enum_token: _,
        ident,
        generics,
        brace_token: _,
        variants,
    } = parse_macro_input!(item as ItemEnum);

    assert_eq!(
        generics.params.len(),
        0,
        "unsupport enum with generic params"
    );

    // let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut impls = vec![];

    for variant in variants {
        if variant.fields.is_empty() {
            continue;
        }

        assert_eq!(variant.fields.len(), 1, "unspport filed({})", variant.ident);

        let filed = variant.fields.iter().next().unwrap();

        let type_path = match &filed.ty {
            syn::Type::Path(type_path) => type_path,
            _ => unimplemented!("unspport filed({})", variant.ident),
        };

        let name = variant.ident;

        let seg = type_path.path.segments.first().unwrap();

        match seg.ident.to_string().as_str() {
            "Box" => {
                let field_type = parse_generic_type(seg).to_token_stream();

                impls.push(quote! {
                    impl From<#field_type> for #ident {
                        fn from(value: #field_type) -> Self {
                            Self::#name(Box::new(value))
                        }
                    }

                    impl<'a> TryFrom<&'a #ident> for &'a #field_type {
                        type Error = ();

                        fn try_from(value: &'a #ident) -> Result<Self,Self::Error> {
                            match value {
                                #ident::#name(v) => Ok(v),
                                _ => Err(())
                            }
                        }
                    }
                });
            }
            "Vec" => {
                let field_type = parse_generic_type(seg).to_token_stream();

                impls.push(quote! {
                    impl From<Vec<#field_type>> for #ident {
                        fn from(value: Vec<#field_type>) -> Self {
                            Self::#name(value)
                        }
                    }

                    impl<'a> TryFrom<&'a #ident> for &'a [#field_type] {
                        type Error = ();

                        fn try_from(value: &'a #ident) -> Result<Self,Self::Error> {
                            match value {
                                #ident::#name(v) => Ok(v),
                                _ => Err(())
                            }
                        }
                    }
                });
            }
            _ => {
                let field_type = type_path.to_token_stream();

                impls.push(quote! {
                    impl From<#field_type> for #ident {
                        fn from(value: #field_type) -> Self {
                            Self::#name(value)
                        }
                    }

                    impl<'a> TryFrom<&'a #ident> for &'a #field_type {
                        type Error = ();

                        fn try_from(value: &'a #ident) -> Result<Self,Self::Error> {
                            match value {
                                #ident::#name(v) => Ok(v),
                                _ => Err(())
                            }
                        }
                    }
                });
            }
        }
    }

    quote! {
        #(#impls)*
    }
    .into()
}
