use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Field, GenericArgument, Ident, ItemStruct, PathSegment, Type};

pub(super) fn derive_stream_api(item: &ItemStruct) -> TokenStream {
    let ItemStruct {
        attrs: _,
        vis: _,
        struct_token: _,
        ident,
        generics,
        fields,
        semi_token: _,
    } = item;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut apis = vec![];

    match fields {
        syn::Fields::Named(fields_named) => {
            for field in &fields_named.named {
                DeriveFiled::new(field.clone()).derive(&mut apis);
            }

            quote! {
                impl #impl_generics #ident #ty_generics #where_clause {
                    #(#apis)*
                }
            }
        }
        syn::Fields::Unnamed(_) => quote! {},
        syn::Fields::Unit => quote! {},
    }
}

#[derive(PartialEq, Debug)]
enum DeriveType {
    Vec,
    Variable,
    Option,
    Unknown(String),
}

struct DeriveFiled {
    ident: Ident,
    root_type: Type,
    type_stack: Vec<DeriveType>,
}

impl DeriveFiled {
    fn new(field: Field) -> Self {
        DeriveFiled {
            ident: field.ident.expect("Unsupport tuple structure."),
            root_type: field.ty,
            type_stack: Default::default(),
        }
    }

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

    fn parse_field_type(&mut self) {
        let mut current_type = &self.root_type;

        loop {
            match current_type {
                Type::Path(path) => {
                    if path.path.segments.len() != 1 {
                        self.type_stack.push(DeriveType::Unknown(
                            current_type.to_token_stream().to_string(),
                        ));

                        break;
                    }

                    let seg = path.path.segments.first().unwrap();

                    match seg.ident.to_string().as_str() {
                        "Option" => {
                            // only parse top level `Option` type.
                            if self.type_stack.is_empty() {
                                self.type_stack.push(DeriveType::Option);

                                current_type = Self::parse_generic_type(seg);

                                continue;
                            } else {
                                self.type_stack.push(DeriveType::Unknown(
                                    current_type.to_token_stream().to_string(),
                                ));

                                break;
                            }
                        }
                        "Vec" => {
                            // only parse: Vec<T>, Variable<Vec<T>>, Option<Vec<T>>, Option<Variable<Vec<T>>>.
                            if self.type_stack.is_empty()
                                || (self.type_stack.len() == 1
                                    && *self.type_stack.first().unwrap() == DeriveType::Option)
                                || (self.type_stack.len() == 1
                                    && *self.type_stack.first().unwrap() == DeriveType::Variable)
                                || (self.type_stack.len() == 2
                                    && self.type_stack[0] == DeriveType::Option
                                    && self.type_stack[1] == DeriveType::Variable)
                            {
                                self.type_stack.push(DeriveType::Vec);

                                current_type = Self::parse_generic_type(seg);

                                continue;
                            } else {
                                self.type_stack.push(DeriveType::Unknown(
                                    current_type.to_token_stream().to_string(),
                                ));

                                break;
                            }
                        }
                        "Variable" => {
                            // only parse Variable<T>, Vec<Variable<T>> or Option<Variable<T>>,
                            if self.type_stack.is_empty()
                                || (self.type_stack.len() == 1
                                    && *self.type_stack.first().unwrap() == DeriveType::Option)
                            {
                                self.type_stack.push(DeriveType::Variable);

                                current_type = Self::parse_generic_type(seg);

                                continue;
                            } else {
                                self.type_stack.push(DeriveType::Unknown(
                                    current_type.to_token_stream().to_string(),
                                ));

                                break;
                            }
                        }
                        _ => {
                            self.type_stack.push(DeriveType::Unknown(
                                current_type.to_token_stream().to_string(),
                            ));

                            break;
                        }
                    }
                }
                _ => {
                    self.type_stack.push(DeriveType::Unknown(
                        current_type.to_token_stream().to_string(),
                    ));

                    break;
                }
            }
        }
    }

    fn fn_animated(&self) -> Option<Ident> {
        self.type_stack
            .contains(&DeriveType::Variable)
            .then(|| format_ident!("{}_animated", self.ident))
    }

    fn derive(&mut self, apis: &mut Vec<proc_macro2::TokenStream>) {
        self.parse_field_type();

        let fn_name = &self.ident;

        let fn_animated = self.fn_animated();

        let variable_reference = quote! {
            crate::opcode::variable::Variable::Reference {
                path: crate::opcode::variable::Path::Named(value.to_owned()),
                target: crate::opcode::variable::Target::Register,
            }
        };

        match self.type_stack.len() {
            // only type T
            1 => match self.type_stack.first().unwrap() {
                DeriveType::Unknown(stream) => {
                    let ty: TokenStream = stream.parse().unwrap();
                    apis.push(quote! {
                        pub fn #fn_name<T>(mut self, value: T) -> Self
                        where
                            #ty: From<T>,
                        {
                            self.#fn_name = value.into();
                            self
                        }
                    });
                }
                _ => {
                    panic!("inner error, type_stack(1)")
                }
            },
            // May be Option<T>, Variable<T> or Vec<T>
            2 => {
                let ty: TokenStream = match &self.type_stack[1] {
                    DeriveType::Unknown(stream) => stream.parse().unwrap(),
                    _ => {
                        panic!("inner error, type_stack(2),T");
                    }
                };

                match &self.type_stack[0] {
                    DeriveType::Vec => {
                        apis.push(quote! {
                            pub fn #fn_name<T>(mut self, value: T) -> Self
                            where
                                T: crate::sexpr::MapCollect<#ty>,
                            {
                                self.#fn_name = value.map_collect();
                                self
                            }
                        });
                    }
                    DeriveType::Variable => {
                        apis.push(quote! {
                            pub fn #fn_name<T>(mut self, value: T) -> Self
                            where
                                #ty: From<T>,
                            {
                                self.#fn_name = crate::opcode::variable::Variable::Constant(value.into());
                                self
                            }

                            pub fn #fn_animated<T>(mut self, value: T) -> Self
                            where
                                T: ToOwned<Owned = String>,
                            {
                                self.#fn_name = #variable_reference;
                                self
                            }
                        });
                    }
                    DeriveType::Option => {
                        apis.push(quote! {
                            pub fn #fn_name<T>(mut self, value: T) -> Self
                            where
                                #ty: From<T>,
                            {
                                self.#fn_name = Some(value.into());
                                self
                            }
                        });
                    }
                    _ => panic!("inner error, type_stack(2)"),
                }
            }
            // May be Option<Vec<T>>, Variable<Vec<T>> or Option<Variable<T>>
            3 => {
                let ty: TokenStream = match &self.type_stack[2] {
                    DeriveType::Unknown(stream) => stream.parse().unwrap(),
                    _ => {
                        panic!("inner error, type_stack(3),T");
                    }
                };

                match &self.type_stack[0] {
                    DeriveType::Option => match &self.type_stack[1] {
                        DeriveType::Vec => {
                            apis.push(quote! {
                                pub fn #fn_name<T>(mut self, value: T) -> Self
                                where
                                    T: crate::sexpr::MapCollect<#ty>,
                                {
                                    self.#fn_name = Some(value.map_collect());
                                    self
                                }
                            });
                        }
                        DeriveType::Variable => {
                            apis.push(quote! {
                                pub fn #fn_name<T>(mut self, value: T) -> Self
                                where
                                    #ty: From<T>,
                                {
                                    self.#fn_name = Some(crate::opcode::variable::Variable::Constant(value.into()));
                                    self
                                }
                                
                                pub fn #fn_animated<T>(mut self, value: T) -> Self
                                where
                                    T: ToOwned<Owned = String>,
                                {
                                    self.#fn_name = Some(#variable_reference);
                                    self
                                }
                            });
                        }
                        _ => {
                            panic!("inner error, type_stack(3), 1");
                        }
                    },
                    // Variable<Vec<T>>>
                    DeriveType::Variable => {
                        match &self.type_stack[1] {
                            DeriveType::Vec => {},
                            _ => {
                                panic!("inner error, type_stack(3),T");
                            }
                        }

                        let ty: TokenStream = match &self.type_stack[2] {
                            DeriveType::Unknown(stream) => stream.parse().unwrap(),
                            _ => {
                                panic!("inner error, type_stack(3),T");
                            }
                        };

                        apis.push(quote! {
                            pub fn #fn_name<T>(mut self, value: T) -> Self
                            where
                                T: crate::sexpr::MapCollect<#ty>,
                            {
                                self.#fn_name = crate::opcode::variable::Variable::Constant(value.map_collect());
                                self
                            }

                            pub fn #fn_animated<T>(mut self, value: T) -> Self
                            where
                                T: ToOwned<Owned = String>,
                            {
                                self.#fn_name = #variable_reference;
                                self
                            }
                        });
                    }
                    _ => {
                        panic!("inner error, type_stack(3), 0");
                    }
                }
            }
            // only Option<Variable<Vec<T>>>
            4 => {
                match &self.type_stack[0] {
                    DeriveType::Option => {},
                    _ => {
                        panic!("inner error, type_stack(4),0");
                    }
                }

                match &self.type_stack[1] {
                    DeriveType::Variable => {},
                    _ => {
                        panic!("inner error, type_stack(4),1");
                    }
                }

                match &self.type_stack[2] {
                    DeriveType::Vec => {},
                    _ => {
                        panic!("inner error, type_stack(4),2");
                    }
                }

                let ty: TokenStream = match &self.type_stack[3] {
                    DeriveType::Unknown(stream) => stream.parse().unwrap(),
                    _ => {
                        panic!("inner error, type_stack(3),T");
                    }
                };

                apis.push(quote! {
                    pub fn #fn_name<T>(mut self, value: T) -> Self
                    where
                        T: crate::sexpr::MapCollect<#ty>,
                    {
                        self.#fn_name = Some(crate::opcode::variable::Variable::Constant(value.map_collect()));
                        self
                    }

                    pub fn #fn_animated<T>(mut self, value: T) -> Self
                    where
                        T: ToOwned<Owned = String>,
                    {
                        self.#fn_name = Some(#variable_reference);
                        self
                    }
                });
            }
            _ => {
                panic!("inner error")
            }
        }
    }
}
