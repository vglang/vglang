use std::collections::HashMap;

use heck::{ToLowerCamelCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;

use crate::compiler::{
    codegen::ext::{EnumGen, FieldGen, IdentGen, NodeGen, TypeGen},
    ir::{ApplyTo, ChildrenOf, Enum, Node, Stat, Type},
};

trait SexprElGen {
    fn gen_el_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream;
}

trait SexprLeafGen {
    fn gen_leaf_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream;
}

trait SexprAttrGen {
    fn gen_attr_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream;
}

trait SexprDataGen {
    fn gen_data_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream;
}

trait SexprRestraintGen {
    fn gen_restraint_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream;
}

impl SexprElGen for Node {
    fn gen_el_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut stats = vec![];
        stats.push(self.gen_from_fn(opcode_mod));
        stats.push(self.gen_default_fn(opcode_mod));
        stats.push(self.gen_chain_init_fns(opcode_mod));
        stats.push(self.gen_graphic_trait_impl(opcode_mod, 1));
        stats.push(self.gen_apply_fn(opcode_mod));
        stats.push(self.gen_children_fn(opcode_mod));

        quote! {
            #(#stats)*
        }
    }
}

impl SexprLeafGen for Node {
    fn gen_leaf_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut stats = vec![];
        stats.push(self.gen_from_fn(opcode_mod));
        stats.push(self.gen_default_fn(opcode_mod));
        stats.push(self.gen_chain_init_fns(opcode_mod));
        stats.push(self.gen_graphic_trait_impl(opcode_mod, 0));
        stats.push(self.gen_apply_fn(opcode_mod));

        quote! {
            #(#stats)*
        }
    }
}

impl SexprAttrGen for Node {
    fn gen_attr_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut stats = vec![];
        stats.push(self.gen_from_fn(opcode_mod));
        stats.push(self.gen_default_fn(opcode_mod));
        stats.push(self.gen_chain_init_fns(opcode_mod));
        stats.push(self.gen_graphic_trait_impl(opcode_mod, 2));

        quote! {
            #(#stats)*
        }
    }
}

impl SexprRestraintGen for ApplyTo {
    fn gen_restraint_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut stats = vec![];

        for from in &self.from {
            let from = from.type_ident();
            for to in &self.to {
                let to = to.type_ident();
                stats.push(quote! {
                    impl ApplyTo<#opcode_mod #to> for #opcode_mod #from {}
                });
            }
        }

        quote! {
            #(#stats)*
        }
    }
}

impl SexprRestraintGen for ChildrenOf {
    fn gen_restraint_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut stats = vec![];

        for from in &self.from {
            let from = from.type_ident();
            for to in &self.to {
                let to = to.type_ident();
                stats.push(quote! {
                    impl ContentOf<#opcode_mod #to> for #opcode_mod #from {}
                });
            }
        }

        quote! {
            #(#stats)*
        }
    }
}

impl Node {
    fn gen_from_fn(&self, opcode_mod: &TokenStream) -> TokenStream {
        if self.init_skip() {
            return quote! {};
        }

        let init_fields = self
            .fields
            .iter()
            .filter(|field| !field.is_option() || field.is_init_field())
            .count();

        if init_fields == 0 {
            return quote! {};
        }

        let mut stats = vec![];
        let mut tuple_index = 0;
        let mut generics = vec![];
        let mut where_clauses = vec![];
        let sexpr_mod = quote! {};

        for field in self.fields.iter() {
            if field.is_option() && !field.is_init_field() {
                if let Some(ident) = field.gen_ident() {
                    stats.push(quote! {
                        #ident: None
                    });
                } else {
                    stats.push(quote! { None });
                }

                continue;
            }

            let value = if init_fields == 1 {
                quote! { value }
            } else {
                format!("value.{}", tuple_index).parse().unwrap()
            };

            let from_expr = field.ty().gen_from_expr(&sexpr_mod, &value);

            let assign_expr = field.gen_assign_expr(opcode_mod, from_expr);

            if let Some(ident) = field.gen_ident() {
                stats.push(quote! {
                    #ident: #assign_expr
                });
                let ident = field.ident().unwrap().type_ident();

                generics.push(ident);
            } else {
                stats.push(quote! { #assign_expr });
                generics.push(format!("P{}", tuple_index).parse().unwrap());
            }

            where_clauses.push(field.ty().gen_from_where_clause(
                opcode_mod,
                &sexpr_mod,
                generics.last().unwrap(),
            ));

            tuple_index += 1;
        }

        let impl_generics = quote! { #(#generics),* };

        let type_generics = if init_fields == 1 {
            quote! { #(#generics),* }
        } else {
            quote! { (#(#generics),*) }
        };

        let ident = self.gen_ident();

        let body = self.gen_body_expr(stats);

        quote! {
            impl<#impl_generics> From<#type_generics> for #opcode_mod #ident where #(#where_clauses),* {
                fn from(value: #type_generics) -> Self {
                    Self # body
                }
            }
        }
    }

    fn gen_default_fn(&self, opcode_mod: &TokenStream) -> TokenStream {
        let init_fields = self
            .fields
            .iter()
            .filter(|field| !field.is_option())
            .count();

        if init_fields != 0 {
            return quote! {};
        }

        let mut init_stats = vec![];

        for field in self.fields.iter() {
            if let Some(ident) = field.gen_ident() {
                init_stats.push(quote! { #ident: None });
            } else {
                init_stats.push(quote! { None });
            }
        }

        let body = self.gen_body_expr(init_stats);

        let ident = self.gen_ident();

        quote! {
            impl Default for #opcode_mod #ident {
                fn default() -> Self {
                    Self #body
                }
            }
        }
    }

    fn gen_chain_init_fns(&self, opcode_mod: &TokenStream) -> TokenStream {
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

    fn gen_graphic_trait_impl(&self, opcode_mod: &TokenStream, ty: usize) -> TokenStream {
        let ident = self.gen_ident();

        match ty {
            0 => {
                quote! {
                    impl Graphics for #opcode_mod #ident {
                        fn build(self, builder: &mut BuildContext) {
                            builder.push(#opcode_mod Leaf::from(self));
                        }
                    }
                }
            }
            1 => {
                quote! {
                    impl Graphics for #opcode_mod #ident {
                        fn build(self, builder: &mut BuildContext) {
                            builder.push(#opcode_mod Element::from(self));
                            builder.pop();
                        }
                    }

                    impl ElementGraphics for #opcode_mod #ident {
                        fn build_element(self, builder: &mut BuildContext) {
                            builder.push(#opcode_mod Element::from(self));
                        }
                    }
                }
            }
            2 => {
                quote! {
                    impl Graphics for #opcode_mod #ident {
                        fn build(self, builder: &mut BuildContext) {
                            builder.push(#opcode_mod Attr::from(self));
                        }
                    }
                }
            }
            _ => panic!("unknown node ty {}", ty),
        }
    }

    fn gen_children_fn(&self, opcode_mod: &TokenStream) -> TokenStream {
        let ident = self.gen_ident();
        quote! {
            impl #opcode_mod #ident {
                pub fn children<C>(self, children: C) -> ElementChildren<Self,C>
                where
                    C: ContentOf<Self>,
                {
                    ElementChildren { node: self, children }
                }
            }
        }
    }

    fn gen_apply_fn(&self, opcode_mod: &TokenStream) -> TokenStream {
        let ident = self.gen_ident();

        quote! {
            impl #opcode_mod #ident {
                pub fn apply<A>(self, attrs: A) -> ApplyElement<A,Self>
                where
                  A: ApplyTo<Self>,
                {
                    ApplyElement { attrs,node: self }
                }
            }
        }
    }
}

impl SexprDataGen for Node {
    fn gen_data_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut stats = vec![];
        stats.push(self.gen_from_fn(opcode_mod));
        stats.push(self.gen_default_fn(opcode_mod));

        quote! {
            #(#stats)*
        }
    }
}

impl SexprDataGen for Enum {
    fn gen_data_sexpr_src(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut impls = vec![];

        let enum_ident = self.gen_ident();

        for node in self.fields.iter() {
            if node.fields.is_empty() {
                continue;
            }

            let ident = format!(
                "S{}{}",
                self.ident.1.to_lower_camel_case(),
                node.ident.1.to_upper_camel_case()
            )
            .parse::<TokenStream>()
            .unwrap();

            let field_ident = node.ident.field_ident();

            impls.push(quote! {
                pub trait #ident {
                    fn #field_ident(self) -> #opcode_mod #enum_ident;
                }
            });

            if node.init_skip() {
                continue;
            }

            let init_fields = node
                .fields
                .iter()
                .filter(|field| !field.is_option() || field.is_init_field())
                .count();

            if init_fields == 0 {
                continue;
            }

            let mut stats = vec![];
            let mut tuple_index = 0;
            let mut generics = vec![];
            let mut where_clauses = vec![];
            let sexpr_mod = quote! {};

            for field in node.fields.iter() {
                if field.is_option() && !field.is_init_field() {
                    if let Some(ident) = field.gen_ident() {
                        stats.push(quote! {
                            #ident: None
                        });
                    } else {
                        stats.push(quote! { None });
                    }

                    continue;
                }

                let value = if init_fields == 1 {
                    quote! { self }
                } else {
                    format!("self.{}", tuple_index).parse().unwrap()
                };

                let from_expr = field.ty().gen_from_expr(&sexpr_mod, &value);

                let assign_expr = field.gen_assign_expr(opcode_mod, from_expr);

                if let Some(ident) = field.gen_ident() {
                    stats.push(quote! {
                        #ident: #assign_expr
                    });
                    let ident = field.ident().unwrap().type_ident();

                    generics.push(ident);
                } else {
                    stats.push(quote! { #assign_expr });
                    generics.push(format!("P{}", tuple_index).parse().unwrap());
                }

                where_clauses.push(field.ty().gen_from_where_clause(
                    opcode_mod,
                    &sexpr_mod,
                    generics.last().unwrap(),
                ));

                tuple_index += 1;
            }

            let impl_generics = quote! { #(#generics),* };

            let type_generics = if init_fields == 1 {
                quote! { #(#generics),* }
            } else {
                quote! { (#(#generics),*) }
            };

            let node_ident = node.gen_ident();

            let body = node.gen_body_expr(stats);

            impls.push(quote! {
                impl<#impl_generics> #ident for #type_generics where #(#where_clauses),* {
                    fn #field_ident(self) -> #opcode_mod #enum_ident {
                        #opcode_mod #enum_ident::#node_ident # body
                    }
                }
            });
        }

        quote! { #(#impls)* }
    }
}

/// A generator that create `sexpr` mod for `mlang`.
#[allow(unused)]
pub struct SexprModGen {
    /// Sexpr support tuple max length.
    tuple_max_len: usize,
    /// Stat mod path prefix.
    opcode_mod: TokenStream,
    /// ident collection to generate MapCollect.
    map_collect_types: HashMap<String, Type>,
}

impl SexprModGen {
    /// Create new sexpr mode generator
    pub fn new<S>(opcode_mod: S, tuple_max_len: usize) -> Self
    where
        String: From<S>,
    {
        assert!(tuple_max_len > 1, "tuple length must greater than 1");
        Self {
            tuple_max_len,
            map_collect_types: HashMap::new(),
            opcode_mod: String::from(opcode_mod).parse().unwrap(),
        }
    }
    /// Generate sexpr mod
    pub fn gen(mut self, stats: &[Stat]) -> TokenStream {
        let mut token_streams = vec![self.gen_common_codes()];

        for stat in stats {
            match stat {
                Stat::Element(node) => {
                    token_streams.push(node.gen_el_sexpr_src(&self.opcode_mod));
                    self.collect_map_collect_types(node);
                }
                Stat::Leaf(node) => {
                    token_streams.push(node.gen_leaf_sexpr_src(&self.opcode_mod));
                    self.collect_map_collect_types(node);
                }
                Stat::Attr(node) => {
                    token_streams.push(node.gen_attr_sexpr_src(&self.opcode_mod));
                    self.collect_map_collect_types(node);
                }
                Stat::Data(node) => {
                    token_streams.push(node.gen_data_sexpr_src(&self.opcode_mod));
                    self.collect_map_collect_types(node);
                }
                Stat::Enum(node) => {
                    token_streams.push(node.gen_data_sexpr_src(&self.opcode_mod));
                    for field in &node.fields {
                        self.collect_map_collect_types(field);
                    }
                }
                Stat::ApplyTo(node) => {
                    token_streams.push(node.gen_restraint_sexpr_src(&self.opcode_mod));
                }
                Stat::ChildrenOf(node) => {
                    token_streams.push(node.gen_restraint_sexpr_src(&self.opcode_mod));
                }
                _ => {}
            }
        }

        token_streams.push(self.gen_tuple_impls());
        token_streams.push(self.gen_point_map_collect());

        quote! {
            #(#token_streams)*
        }
    }

    fn gen_point_map_collect(&self) -> TokenStream {
        let opcode_mod = &self.opcode_mod;
        let mut impls = vec![];

        for len in 1..self.tuple_max_len {
            let mut generics = vec![];
            let mut where_clauses = vec![];
            let mut stats = vec![];

            for i in 0..len {
                let i = i * 2;
                let ty = format!("P{}", i).parse::<TokenStream>().unwrap();

                where_clauses.push(quote! {
                    Number: From<#ty>
                });

                generics.push(ty);

                let ty = format!("P{}", i + 1).parse::<TokenStream>().unwrap();

                where_clauses.push(quote! {
                    Number: From<#ty>
                });

                generics.push(ty);

                stats.push(
                    format!(
                        "{} Point(Number::from(self.{}).0,Number::from(self.{}).0)",
                        opcode_mod,
                        i,
                        i + 1
                    )
                    .parse::<TokenStream>()
                    .unwrap(),
                );
            }

            impls.push(quote! {
                impl<#(#generics),*> MapCollect<#opcode_mod Point> for (#(#generics),*)
                where
                    #(#where_clauses),*
                {
                    fn map_collect(self) -> Vec<#opcode_mod Point> {
                        vec![#(#stats),*]
                    }
                }
            });
        }

        quote! {
            #(#impls)*
        }
    }

    fn collect_map_collect_types(&mut self, node: &Node) {
        for field in node.fields.iter() {
            match field.ty() {
                Type::ListOf(component, _) => {
                    let ident = component.gen_type_definition(&quote! {}).to_string();
                    match ident.as_str() {
                        "Point" => {}
                        _ => {
                            self.map_collect_types.insert(ident, *component.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn gen_common_codes(&self) -> TokenStream {
        let opcode_mod = &self.opcode_mod;
        quote! {
            /// build context used by [`Graphics`] trait.
            #[derive(Debug, Default)]
            pub struct BuildContext(pub Vec<#opcode_mod Opcode>);

            impl BuildContext {
                /// Push a new `Stat`
                pub fn push<O>(&mut self, opcode: O)
                where
                    #opcode_mod Opcode: From<O>,
                {
                    self.0.push(opcode.into());
                }

                /// Push a `Pop` opcode.
                pub fn pop(&mut self) {
                    self.0.push(#opcode_mod Opcode::Pop);
                }
            }

            /// An attribute that can used as appliable attribute for one element must implement this trait.
            pub trait ApplyTo<E> {}

            /// An element that can used as container element's child must implement this trait.
            pub trait ContentOf<E> {}

            /// s-expr combinator must implement this trait.
            pub trait Graphics {
                /// Generate `Stat`s for specific surface.
                fn build(self, builder: &mut BuildContext);
            }

            pub trait ElementGraphics: Graphics {
                /// Generate `Stat`s for specific surface.
                fn build_element(self, builder: &mut BuildContext);
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

            impl<Attrs, Node, E> ContentOf<E> for ApplyElement<Attrs, Node>
            where
                Node: ContentOf<E>,
            {
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

            impl<Attrs, Node, Children, E> ContentOf<E> for ApplyElementChildren<Attrs, Node, Children>
            where
                Node: ContentOf<E>,
            {
            }

            impl<Attrs, Node, Children> Graphics for ApplyElementChildren<Attrs, Node, Children>
            where
                Attrs: ApplyTo<Node> + Graphics,
                Node: ElementGraphics,
                Children: Graphics,
            {
                fn build(self, builder: &mut BuildContext) {
                    self.attrs.build(builder);
                    self.node.build_element(builder);
                    self.children.build(builder);
                    builder.pop();
                }
            }

            /// A wrapper [`Graphics`] returns by calling container's children function.
            pub struct ElementChildren<Node, Children> {
                pub node: Node,
                pub children: Children,
            }

            impl<Node, Children, E> ContentOf<E> for ElementChildren<Node, Children>
            where
                Node: ContentOf<E>,
            {
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
                Node: ElementGraphics,
                Children: Graphics,
            {
                fn build(self, builder: &mut BuildContext) {
                    self.node.build_element(builder);
                    self.children.build(builder);
                    builder.pop();
                }
            }

            /// Map item via iterator and collect them into vec.
            #[allow(unused)]
            pub trait MapCollect<Item> {
                fn map_collect(self) -> Vec<Item>;
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
        }
    }

    fn gen_tuple_impls(&self) -> TokenStream {
        let mut stats = vec![];
        for i in 2..self.tuple_max_len {
            stats.push(self.gen_tuple_graphics_impl(i));
            stats.push(self.gen_tuple_apply_to_impl(i));
            stats.push(self.gen_tuple_content_of_impl(i));
            stats.push(self.gen_tuple_map_collect_impl(i));
        }

        let sexpr_mod = quote! {};

        for (_, ty) in &self.map_collect_types {
            let ty_ident = ty.gen_type_definition(&self.opcode_mod);

            let param_type: TokenStream = quote! { P };

            let where_clause = ty.gen_from_where_clause(&self.opcode_mod, &sexpr_mod, &param_type);

            let param = quote! {self};

            let field = ty.gen_from_expr(&sexpr_mod, &param);

            stats.push(quote! {
                impl<#param_type> MapCollect<#ty_ident> for #param_type
                where #where_clause
                {
                    fn map_collect(self) -> Vec<#ty_ident> {
                        vec![#field]
                    }
                }
            });
        }

        quote! {
            #(#stats)*
        }
    }

    fn gen_tuple_map_collect_impl(&self, len: usize) -> TokenStream {
        let mut impls = vec![];
        let sexpr_mod = quote! {};

        for (_, ty) in &self.map_collect_types {
            let ty_ident = ty.gen_type_definition(&self.opcode_mod);
            let mut param_types = vec![];
            let mut where_clauses = vec![];
            let mut fields = vec![];

            for i in 0..len {
                let param_type: TokenStream = format!("P{}", i).parse().unwrap();

                where_clauses.push(ty.gen_from_where_clause(
                    &self.opcode_mod,
                    &sexpr_mod,
                    &param_type,
                ));

                let param = if len == 1 {
                    quote! {self}
                } else {
                    format!("self.{}", i).parse().unwrap()
                };

                fields.push(ty.gen_from_expr(&sexpr_mod, &param));

                param_types.push(param_type);
            }

            impls.push(quote! {
                impl<#(#param_types),*> MapCollect<#ty_ident> for (#(#param_types),*)
                where #(#where_clauses),*
                {
                    fn map_collect(self) -> Vec<#ty_ident> {
                        vec![#(#fields),*]
                    }
                }
            });
        }

        quote! { #(#impls)* }
    }

    fn gen_tuple_apply_to_impl(&self, len: usize) -> TokenStream {
        let mut generics = vec![];

        for i in 0..len {
            generics.push(format!("P{}", i).parse::<TokenStream>().unwrap());
        }

        quote! {
            impl<E, #(#generics),*> ApplyTo<E> for (#(#generics),*)
            where
                #(#generics: ApplyTo<E>),*
            {
            }
        }
    }

    fn gen_tuple_content_of_impl(&self, len: usize) -> TokenStream {
        let mut generics = vec![];

        for i in 0..len {
            generics.push(format!("P{}", i).parse::<TokenStream>().unwrap());
        }

        quote! {
            impl<E, #(#generics),*> ContentOf<E> for (#(#generics),*)
            where
                #(#generics: ContentOf<E>),*
            {
            }
        }
    }

    fn gen_tuple_graphics_impl(&self, len: usize) -> TokenStream {
        let mut generics = vec![];
        let mut stats = vec![];

        for i in 0..len {
            generics.push(format!("P{}", i).parse::<TokenStream>().unwrap());

            let ident: TokenStream = format!("{}", i).parse().unwrap();

            stats.push(quote! {
                self.#ident.build(builder)
            });
        }

        quote! {
            impl<#(#generics),*> Graphics for (#(#generics),*)
            where
                #(#generics: Graphics),*
            {
                fn build(self, builder: &mut BuildContext) {
                    #(#stats);*
                }
            }
        }
    }
}
