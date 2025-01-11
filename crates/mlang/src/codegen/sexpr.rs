use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::ext::{EnumGen, FieldGen, IdentGen, NodeGen, TypeGen},
    opcode::{ApplyTo, ChildrenOf, Enum, Node, Opcode},
};

pub trait SexprGen {
    fn gen_sexpr_fns(&self, opcode_mod: &TokenStream) -> TokenStream;
}

impl SexprGen for Node {
    fn gen_sexpr_fns(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut fns = vec![];

        fns.push(self.gen_sexpr_default_fn(opcode_mod));
        fns.push(self.gen_sexpr_init_fn(opcode_mod));
        fns.push(self.gen_sexpr_from_one_init_fn(opcode_mod));
        fns.push(self.gen_sexpr_chain_build_fn(opcode_mod));

        quote! {
            #(#fns)*
        }
    }
}

impl Node {
    fn gen_sexpr_init_fn(&self, opcode_mod: &TokenStream) -> TokenStream {
        let non_options = self.fields.iter().fold(0usize, |non_options, field| {
            if field.is_option() {
                non_options
            } else {
                non_options + 1
            }
        });

        if non_options == 0 {
            return quote! {};
        }

        let ident = self.gen_ident();

        let ident = quote! {#opcode_mod #ident};

        let mut generic_index = 0usize;

        let mut impl_generics = vec![];
        let mut where_clauses = vec![];

        let sexpr_mod = quote! {};

        let mut fields = vec![];

        for field in self.fields.iter() {
            if !field.is_option() {
                let param = format!("P{}", generic_index).parse().unwrap();

                let from_expr = if non_options > 1 {
                    field.ty().gen_from_expr(
                        &sexpr_mod,
                        &format!("value.{}", generic_index).parse().unwrap(),
                    )
                } else {
                    field.ty().gen_from_expr(&sexpr_mod, &quote! { value })
                };

                fields.push(field.gen_init_expr(opcode_mod, from_expr));

                where_clauses.push(
                    field
                        .ty()
                        .gen_from_where_clause(opcode_mod, &sexpr_mod, &param),
                );
                impl_generics.push(param);
                generic_index += 1;
            } else {
                fields.push(field.gen_init_none());
            }
        }

        let type_generics = if non_options > 1 {
            quote! { (#(#impl_generics),*) }
        } else {
            quote! { #(#impl_generics),* }
        };

        let body = self.gen_body_expr(fields);

        quote! {
            impl<#(#impl_generics),*> From<#type_generics> for #ident where #(#where_clauses),* {
                fn from(value: #type_generics) -> Self {
                    Self # body
                }
            }
        }
    }

    fn gen_sexpr_chain_build_fn(&self, opcode_mod: &TokenStream) -> TokenStream {
        if self.is_tuple() {
            return quote! {};
        }

        let sexpr_mod = quote! {};

        let ident = self.gen_ident();

        let mut fns = vec![];

        for field in self.fields.iter() {
            let param = quote! {T};

            let from_expr = field.ty().gen_from_expr(&sexpr_mod, &quote! { value });

            let where_clause = field
                .ty()
                .gen_from_where_clause(opcode_mod, &sexpr_mod, &param);

            let fn_name: TokenStream = field.ident().unwrap().field_ident();

            let assign = field.gen_assign_expr(opcode_mod, from_expr);

            fns.push(quote! {
                impl #opcode_mod #ident {
                    pub fn #fn_name<T>(mut self, value: T) -> Self where #where_clause {
                        self.#fn_name = #assign;
                        self
                    }
                }
            });
        }

        quote! {
            #(#fns)*
        }
    }

    fn gen_sexpr_from_one_init_fn(&self, opcode_mod: &TokenStream) -> TokenStream {
        if self.is_tuple() {
            return quote! {};
        }

        let no_options = self.fields.iter().fold(0usize, |non_option, field| {
            if field.is_option() {
                non_option
            } else {
                non_option + 1
            }
        });

        if no_options > 0 {
            return quote! {};
        }

        let sexpr_mod = quote! {};

        let ident = self.gen_ident();

        let mut fns = vec![];

        for field in self.fields.iter() {
            let param = quote! {T};

            let from_expr = field.ty().gen_from_expr(&sexpr_mod, &quote! { value });

            let where_clause = field
                .ty()
                .gen_from_where_clause(opcode_mod, &sexpr_mod, &param);

            let fn_name: TokenStream = field.ident().unwrap().field_ident_with_prefix("from_");

            let body = self.gen_body_expr(vec![
                field.gen_init_expr(opcode_mod, from_expr),
                quote! {..Default::default()},
            ]);

            fns.push(quote! {
                impl #opcode_mod #ident {
                    pub fn #fn_name<T>(value: T) -> Self where #where_clause {
                        Self #body
                    }
                }
            });
        }

        quote! {
            #(#fns)*
        }
    }
    fn gen_sexpr_default_fn(&self, opcode_mod: &TokenStream) -> TokenStream {
        let no_options = self.fields.iter().fold(0usize, |non_option, field| {
            if field.is_option() {
                non_option
            } else {
                non_option + 1
            }
        });

        if no_options > 0 {
            return quote! {};
        }

        let fields = self
            .fields
            .iter()
            .map(|field| {
                if let Some(ident) = field.gen_ident() {
                    quote! { #ident: None}
                } else {
                    quote! { None }
                }
            })
            .collect::<Vec<_>>();

        let ident = self.gen_ident();

        let ident = quote! {#opcode_mod #ident};

        let body = self.gen_body_expr(fields);

        quote! {
            impl Default for #ident {
                fn default() -> Self {
                    Self #body
                }
            }
        }
    }
}

impl SexprGen for Enum {
    fn gen_sexpr_fns(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
}

impl SexprGen for ApplyTo {
    fn gen_sexpr_fns(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
}

impl SexprGen for ChildrenOf {
    fn gen_sexpr_fns(&self, _: &TokenStream) -> TokenStream {
        quote! {}
    }
}

/// A generator that create `sexpr` mod for `mlang`.
pub struct SexprModGen {
    opcode_mod: TokenStream,
    /// collection of data types.
    data_types: Vec<TokenStream>,
    /// collection of attr types
    attr_types: Vec<TokenStream>,
    /// collection of el types
    el_types: Vec<TokenStream>,
    /// collection of leaf node types.
    leaf_types: Vec<TokenStream>,
    /// apply to pair list.
    apply_to_pairs: Vec<(TokenStream, TokenStream)>,
    /// child of pair list.
    child_of_pairs: Vec<(TokenStream, TokenStream)>,
}

impl SexprModGen {
    /// Create new sexpr mode generator
    pub fn new<S>(opcode_mod: S) -> Self
    where
        String: From<S>,
    {
        Self {
            opcode_mod: String::from(opcode_mod).parse().unwrap(),
            data_types: Default::default(),
            attr_types: Default::default(),
            el_types: Default::default(),
            leaf_types: Default::default(),
            apply_to_pairs: Default::default(),
            child_of_pairs: Default::default(),
        }
    }
    /// Generate sexpr mod
    pub fn gen(mut self, opcodes: &[Opcode]) -> TokenStream {
        let mut token_streams = vec![];

        for opcode in opcodes {
            match opcode {
                Opcode::Element(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                    self.el_types.push(node.gen_ident());
                }
                Opcode::Leaf(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                    self.leaf_types.push(node.gen_ident());
                }
                Opcode::Attr(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                    self.attr_types.push(node.gen_ident());
                }
                Opcode::Data(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                    self.data_types.push(node.gen_ident());
                }
                Opcode::Enum(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));
                    self.data_types.push(node.gen_ident());
                }

                Opcode::ApplyTo(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));

                    for from in &node.from {
                        let from = from.type_ident();

                        for to in &node.to {
                            self.apply_to_pairs.push((from.clone(), to.type_ident()));
                        }
                    }
                }
                Opcode::ChildrenOf(node) => {
                    token_streams.push(node.gen_sexpr_fns(&self.opcode_mod));

                    for from in &node.from {
                        let from = from.type_ident();

                        for to in &node.to {
                            self.child_of_pairs.push((from.clone(), to.type_ident()));
                        }
                    }
                }
                _ => {}
            }
        }

        token_streams.push(self.gen_common_codes());

        quote! {
            #(#token_streams)*
        }
    }

    fn gen_common_codes(&self) -> TokenStream {
        let graphics_impl = self.gen_sexpr_graphics_impl();
        let apply_impl = self.gen_sexpr_apply_fn();
        let children_impl = self.gen_sexpr_children_fn();
        let apply_to_impl = self.gen_sexpr_apply_to_impl();
        let child_of_impl = self.gen_sexpr_child_of_impl();

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

    fn gen_sexpr_graphics_impl(&self) -> Vec<TokenStream> {
        self.el_types
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
            .chain(self.leaf_types.iter().map(|ident| {
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

    fn gen_sexpr_apply_fn(&self) -> Vec<TokenStream> {
        self.el_types
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
            .chain(self.leaf_types.iter().map(|ident| {
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

    fn gen_sexpr_children_fn(&self) -> Vec<TokenStream> {
        self.el_types
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

    fn gen_sexpr_apply_to_impl(&self) -> Vec<TokenStream> {
        self.apply_to_pairs
            .iter()
            .map(|(from, to)| {
                quote! {
                    impl ApplyTo<super::opcode::#to> for super::opcode::#from {}
                }
            })
            .collect()
    }

    fn gen_sexpr_child_of_impl(&self) -> Vec<TokenStream> {
        self.child_of_pairs
            .iter()
            .map(|(from, to)| {
                quote! {
                    impl ContentOf<super::opcode::#to> for super::opcode::#from {}
                }
            })
            .collect()
    }
}
