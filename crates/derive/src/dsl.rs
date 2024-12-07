use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, GenericArgument, ItemStruct, Type};

pub fn derive_api(item: TokenStream) -> TokenStream {
    let ItemStruct {
        attrs: _,
        vis: _,
        struct_token: _,
        ident,
        generics,
        fields,
        semi_token: _,
    } = parse_macro_input!(item as ItemStruct);

    let mut apis = vec![];

    for field in fields {
        match field.ty {
            syn::Type::Path(type_path) => {
                for seg in type_path.path.segments {
                    if seg.ident == "Animatable" {
                        match seg.arguments {
                            syn::PathArguments::AngleBracketed(args) => {
                                assert_eq!(
                                    args.args.len(),
                                    1,
                                    "Animatable only apply one generic argument."
                                );

                                let content_type = args.args.first().unwrap();

                                let fn_name =
                                    field.ident.as_ref().expect("Can't drive tuple struct.");

                                let animatable_fn_name = format_ident!("{}_animated", fn_name);

                                let is_vec = match content_type {
                                    GenericArgument::Type(t) => is_vec(t),
                                    _ => {
                                        panic!("Unsupport animatable argument");
                                    }
                                };

                                if let Some(content_type) = is_vec {
                                    apis.push(quote! {
                                        /// Set attribute `#fn_name` with constant value.
                                        pub fn #fn_name<V>(mut self, v: V) -> Self 
                                        where 
                                            V: crate::MapCollect<Item=#content_type>,
                                        {
                                            self.#fn_name = Animatable::Constant(v.map_collect());
                                            self
                                        }
                                    
                                        /// Set attribute `#fn_name` with animatable register name.
                                        pub fn #animatable_fn_name<V>(mut self, v: V) -> Self where V: Into<String> {
                                            self.#fn_name = Animatable::Animated(v.into());
                                            self
                                        }
                                    });
                                } else {
                                    apis.push(quote! {
                                        /// Set attribute `#fn_name` with constant value.
                                        pub fn #fn_name<V>(mut self, v: V) -> Self where V: Into<#content_type> {
                                            self.#fn_name = Animatable::Constant(v.into());
                                            self
                                        }

                                        /// Set attribute `#fn_name` with animatable register name.
                                        pub fn #animatable_fn_name<V>(mut self, v: V) -> Self where V: Into<String> {
                                            self.#fn_name = Animatable::Animated(v.into());
                                            self
                                        }
                                    });
                                }

                                // println!("animatable field: {}", content_type.to_token_stream());
                            }
                            _ => {
                                panic!("unsupport.");
                            }
                        }
                    } else {
                        let fn_name = field.ident.as_ref().expect("Can't drive tuple struct.");

                        let content_type = seg.to_token_stream();

                        apis.push(quote! {
                            /// Set attribute `#fn_name` with constant value.
                            pub fn #fn_name<V>(mut self, v: V) -> Self where V: Into<#content_type> {
                                self.#fn_name = v.into();
                                self
                            }
                        });
                    }
                }
            }

            _ => {
                panic!("Unsupport field: {}", field.ty.to_token_stream());
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {

            /// Create default attrs for this element.
            pub fn attrs() -> Self where Self: Default {
                Self::default()
            }

            #(#apis)*
        }
    }
    .into()
}

fn is_vec(t: &Type) -> Option<&GenericArgument> {
    if let syn::Type::Path(path) = t {
        if let Some(seg) = path.path.segments.first() {
            if seg.ident == "Vec" {
                match &seg.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        assert_eq!(
                            args.args.len(),
                            1,
                            "Animatable only apply one generic argument."
                        );

                        return args.args.first();
                    }

                    _ => {
                        panic!("Unsupport vec argument");
                    }
                }
            }
        }
    }

    None
}
