use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn gen_sexpr_mod(
    el_fields: &[TokenStream],
    leaf_fields: &[TokenStream],
    apply_to: &[(TokenStream, TokenStream)],
    child_of: &[(TokenStream, TokenStream)],
) -> TokenStream {
    let graphics_impl = gen_sexpr_graphics_impl(el_fields, leaf_fields);
    let apply_impl = gen_sexpr_apply_fn(el_fields, leaf_fields);
    let children_impl = gen_sexpr_children_fn(el_fields);
    let apply_to_impl = gen_sexpr_apply_to_impl(apply_to);
    let child_of_impl = gen_sexpr_child_of_impl(child_of);

    quote! {
        use super::opcode::{Opcode};

        /// An attribute that can used as appliable attribute for one element must implement this trait.
        pub trait ApplyTo<E> {}

        /// An element that can used as container element's child must implement this trait.
        pub trait ContentOf<E> {}

        /// s-expr combinator must implement this trait.
        pub trait Graphics {
            /// Generate `opcode`s for specific surface.
            fn build(self, builder: &mut BuildContext);
        }

        /// build context used by [`Graphics`] trait.
        #[derive(Debug, Default)]
        pub struct BuildContext(Vec<Opcode>);

        impl From<BuildContext> for Vec<Opcode> {
            fn from(value: BuildContext) -> Self {
                value.0
            }
        }

        impl AsRef<[Opcode]> for BuildContext {
            fn as_ref(&self) -> &[Opcode] {
                &self.0
            }
        }

        impl BuildContext {
            /// Push a new `opcode`
            pub fn push<O>(&mut self, opcode: O)
            where
                Opcode: From<O>,
            {
                self.0.push(opcode.into());
            }

            /// Push a `Pop` opcode.
            pub fn pop(&mut self) {
                self.0.push(Opcode::Pop);
            }
        }

        /// A wrapper [`Graphics`] returns by shape's apply function.
        pub struct ApplyLeaf<Attrs, Node> {
            pub attrs: Attrs,
            pub node: Node,
        }

        impl<Attrs, Node> Graphics for ApplyLeaf<Attrs, Node>
        where
            Attrs: ApplyTo<Node> + Graphics,
            Node: Graphics,
        {
            fn build(self, builder: &mut BuildContext) {
                self.attrs.build(builder);
                self.node.build(builder);
            }
        }

        /// A wrapper [`Graphics`] returns by shape's apply function.
        pub struct ApplyElement<Attrs, Node> {
            pub attrs: Attrs,
            pub node: Node,
        }

        impl<Attrs, Node> Graphics for ApplyElement<Attrs, Node>
        where
            Attrs: ApplyTo<Node> + Graphics,
            Node: Graphics,
        {
            fn build(self, builder: &mut BuildContext) {
                self.attrs.build(builder);
                self.node.build(builder);
            }
        }

        impl<Attrs, Node> ApplyElement<Attrs, Node>
        where
            Attrs: ApplyTo<Node> + Graphics,
            Node: Graphics,
        {
            /// Apply the children node.
            pub fn children<Children>(
                self,
                children: Children,
            ) -> ApplyElementChildren<Attrs, Node, Children> {
                ApplyElementChildren {
                    attrs: self.attrs,
                    node: self.node,
                    children,
                }
            }
        }

        /// A wrapper [`Graphics`] returns by [`ApplyElement::children`] or container's `children` function.
        pub struct ApplyElementChildren<Attrs, Node, Children> {
            pub attrs: Attrs,
            pub node: Node,
            pub children: Children,
        }

        impl<Attrs, Node, Children> Graphics for ApplyElementChildren<Attrs, Node, Children>
        where
            Attrs: ApplyTo<Node> + Graphics,
            Node: Graphics,
            Children: Graphics,
        {
            fn build(self, builder: &mut BuildContext) {
                self.attrs.build(builder);
                self.node.build(builder);
                self.children.build(builder);
                builder.pop();
            }
        }

        /// A wrapper [`Graphics`] returns by calling container's children function.
        pub struct ElementChildren<Node, Children> {
            pub node: Node,
            pub children: Children,
        }

        impl<Node, Children> ElementChildren<Node, Children>
        where
            Node: Graphics,
        {
            /// Apply the children node.
            pub fn apply<Attrs>(self, attrs: Attrs) -> ApplyElementChildren<Attrs, Node, Children>
            where
                Attrs: ApplyTo<Node>,
            {
                ApplyElementChildren {
                    attrs,
                    node: self.node,
                    children: self.children,
                }
            }
        }

        impl<Node, Children> Graphics for ElementChildren<Node, Children>
        where
            Node: Graphics,
            Children: Graphics,
        {
            fn build(self, builder: &mut BuildContext) {
                self.node.build(builder);
                self.children.build(builder);
                builder.pop();
            }
        }

        /// Map item via iterator and collect them into vec.
        #[allow(unused)]
        pub trait MapCollect<Item> {
            fn map_collect(self) -> Vec<Item>;
        }

        impl<F, T> MapCollect<T> for F
        where
            T: From<F>,
        {
            fn map_collect(self) -> Vec<T> {
                vec![self.into()]
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct Number(
            /// The wrapped [`f32`] value.
            pub f32,
        );

        impl From<i32> for Number {
            fn from(value: i32) -> Self {
                Self(value as f32)
            }
        }

        impl From<f32> for Number {
            fn from(value: f32) -> Self {
                Self(value as f32)
            }
        }

        #(#graphics_impl)*
        #(#apply_impl)*
        #(#children_impl)*
        #(#apply_to_impl)*
        #(#child_of_impl)*
    }
}

fn gen_sexpr_graphics_impl(
    el_fields: &[TokenStream],
    leaf_fields: &[TokenStream],
) -> Vec<TokenStream> {
    el_fields
        .iter()
        .map(|ident| {
            quote! {
                impl Graphics for super::opcode::#ident {
                    fn build(self, builder: &mut BuildContext) {
                        builder.push(super::opcode::Element::from(self));
                    }
                }
            }
        })
        .chain(leaf_fields.iter().map(|ident| {
            quote! {
                impl Graphics for super::opcode::#ident {
                    fn build(self, builder: &mut BuildContext) {
                        builder.push(super::opcode::Leaf::from(self));
                    }
                }
            }
        }))
        .collect()
}

fn gen_sexpr_apply_fn(el_fields: &[TokenStream], leaf_fields: &[TokenStream]) -> Vec<TokenStream> {
    el_fields
        .iter()
        .map(|ident| {
            quote! {
                impl super::opcode::#ident {
                    pub fn apply<A>(self, attrs: A) -> ApplyElement<A,Self>
                    where
                      A: ApplyTo<Self>,
                    {
                        ApplyElement { attrs,node: self }
                    }
                }
            }
        })
        .chain(leaf_fields.iter().map(|ident| {
            quote! {
                impl super::opcode::#ident {
                    pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A,Self>
                    where
                        A: ApplyTo<Self>,
                    {
                        ApplyLeaf { attrs,node: self }
                    }
                }
            }
        }))
        .collect()
}

fn gen_sexpr_children_fn(el_fields: &[TokenStream]) -> Vec<TokenStream> {
    el_fields
        .iter()
        .map(|ident| {
            quote! {
                impl super::opcode::#ident {
                    pub fn children<C>(self, children: C) -> ElementChildren<Self,C>
                    where
                        C: ContentOf<Self>,
                    {
                        ElementChildren { node: self, children }
                    }
                }
            }
        })
        .collect()
}

fn gen_sexpr_apply_to_impl(apply_to: &[(TokenStream, TokenStream)]) -> Vec<TokenStream> {
    apply_to
        .iter()
        .map(|(from, to)| {
            quote! {
                impl ApplyTo<super::opcode::#to> for super::opcode::#from {}
            }
        })
        .collect()
}

fn gen_sexpr_child_of_impl(child_of: &[(TokenStream, TokenStream)]) -> Vec<TokenStream> {
    child_of
        .iter()
        .map(|(from, to)| {
            quote! {
                impl ContentOf<super::opcode::#to> for super::opcode::#from {}
            }
        })
        .collect()
}
