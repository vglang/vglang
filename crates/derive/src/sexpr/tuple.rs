use quote::{format_ident, quote};

pub fn derive_tuple_traits(max_length: usize) -> proc_macro::TokenStream {
    assert!(max_length >= 2, "length must greater than or equal to 2");
    let mut impls = vec![];

    for length in 2..max_length {
        let mut params = vec![];

        let mut point_params = vec![];
        let mut point_where_params = vec![];
        let mut point_into_params = vec![];

        for i in 0..length {
            params.push(format_ident!("P{}", i));

            let x = format_ident!("X{}", i);
            let y = format_ident!("Y{}", i);

            point_params.push(quote! {
                #x,#y
            });

            point_where_params.push(quote! {
                crate::opcode::data::Number: From<#x> + From<#y>
            });

            point_into_params.push(quote! {
                (#x,#y).into()
            });
        }

        impls.push(quote! {
            impl<Target,#(#params),*> crate::sexpr::ApplyTo<Target> for (#(#params),*)
            where
                Target: crate::sexpr::Element,
                #(#params: crate::sexpr::ApplyTo<Target>),*
            {

            }

            impl<#(#params),*> crate::sexpr::Graphics for (#(#params),*)
            where
                #(#params: crate::sexpr::Graphics),*
            {
                fn build(self, builder: &mut crate::sexpr::BuildContext) {
                    let (#(#params),*) = self;

                    #(#params.build(builder);)*
                }
            }


            impl<#(#params),*> crate::sexpr::MapCollect<crate::opcode::data::Length> for (#(#params),*)
            where
                #(crate::opcode::data::Length: From<#params>),*
            {
                fn map_collect(self) -> Vec<crate::opcode::data::Length> {
                    let (#(#params),*) = self;

                    vec![#(#params.into()),*]
                }
            }

            impl<#(#params),*> crate::sexpr::MapCollect<crate::opcode::data::FontFamily> for (#(#params),*)
            where
                #(crate::opcode::data::FontFamily: From<#params>),*
            {
                fn map_collect(self) -> Vec<crate::opcode::data::FontFamily> {
                    let (#(#params),*) = self;

                    vec![#(#params.into()),*]
                }
            }

            impl<#(#params),*> crate::sexpr::MapCollect<crate::opcode::data::Angle> for (#(#params),*)
            where
                #(crate::opcode::data::Angle: From<#params>),*
            {
                fn map_collect(self) -> Vec<crate::opcode::data::Angle> {
                    let (#(#params),*) = self;

                    vec![#(#params.into()),*]
                }
            }

            impl<#(#params),*> crate::sexpr::MapCollect<crate::opcode::data::PathEvent> for (#(#params),*)
            where
                #(crate::opcode::data::PathEvent: From<#params>),*
            {
                fn map_collect(self) -> Vec<crate::opcode::data::PathEvent> {
                    let (#(#params),*) = self;

                    vec![#(#params.into()),*]
                }
            }

            impl<#(#params),*> crate::sexpr::MapCollect<crate::opcode::data::Transform> for (#(#params),*)
            where
                #(crate::opcode::data::Transform: From<#params>),*
            {
                fn map_collect(self) -> Vec<crate::opcode::data::Transform> {
                    let (#(#params),*) = self;

                    vec![#(#params.into()),*]
                }
            }

            impl<#(#point_params),*> crate::sexpr::MapCollect<crate::opcode::data::Point> for (#(#point_params),*)
            where
                #(#point_where_params),*
            {
                fn map_collect(self) -> Vec<crate::opcode::data::Point> {
                    let (#(#point_params),*) = self;

                    vec![#(#point_into_params),*]
                }
            }
        });
    }

    quote! {
        #(#impls)*
    }
    .into()
}
