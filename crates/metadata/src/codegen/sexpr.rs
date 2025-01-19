use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::ext::{EnumGen, FieldGen, IdentGen, NodeGen, TypeGen},
    ir::{ApplyTo, ChildrenOf, Enum, Node, Stat, Type},
};

pub trait SexprGen {
    fn gen_sexpr_source_codes(&self, opcode_mod: &TokenStream) -> TokenStream;
}

impl SexprGen for Node {
    fn gen_sexpr_source_codes(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut fns = vec![];

        fns.push(self.gen_sexpr_default_fn(opcode_mod));
        fns.push(self.gen_sexpr_init_fn(opcode_mod));
        fns.push(self.gen_sexpr_init_fn2(opcode_mod));
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

        let mut generics = vec![];
        let mut where_clauses = vec![];

        let sexpr_mod = quote! {};

        let mut fields = vec![];

        for field in self.fields.iter() {
            if !field.is_option() {
                let param_type = format!("P{}", generic_index).parse().unwrap();
                let param = if non_options > 1 {
                    format!("value.{}", generic_index).parse().unwrap()
                } else {
                    quote! { value }
                };

                let from_expr = field.ty().gen_from_expr(&sexpr_mod, &param);

                fields.push(field.gen_init_expr(opcode_mod, from_expr));

                where_clauses.push(field.ty().gen_from_where_clause(
                    opcode_mod,
                    &sexpr_mod,
                    &param_type,
                ));
                generics.push(param_type);
                generic_index += 1;
            } else {
                fields.push(field.gen_init_none());
            }
        }

        let body = self.gen_body_expr(fields);

        let (impl_generics, generics) = if generics.len() > 1 {
            (
                quote! {
                    <#(#generics),*>
                },
                quote! {
                    (#(#generics),*)
                },
            )
        } else {
            (
                quote! {
                    <#(#generics),*>
                },
                quote! {
                    #(#generics),*
                },
            )
        };

        quote! {
            impl #impl_generics From<#generics> for #ident where #(#where_clauses),* {
                fn from(value: #generics) -> Self {
                    Self # body
                }
            }
        }
    }

    fn gen_sexpr_init_fn2(&self, opcode_mod: &TokenStream) -> TokenStream {
        let non_options = self.fields.iter().fold(0usize, |non_options, field| {
            if field.is_option() {
                non_options
            } else {
                non_options + 1
            }
        });

        if non_options > 0 {
            return quote! {};
        }

        let ident = self.gen_ident();

        let ident = quote! {#opcode_mod #ident};

        let mut generic_index = 0usize;

        let mut generics = vec![];
        let mut where_clauses = vec![];

        let sexpr_mod = quote! {};

        let mut fields = vec![];

        for field in self.fields.iter() {
            if field.is_init_field() {
                let param_type = format!("P{}", generic_index).parse().unwrap();
                let param = if let Some(ident) = field.gen_ident() {
                    ident
                } else {
                    format!("p{}", generic_index).parse().unwrap()
                };

                let from_expr = field.ty().gen_from_expr(&sexpr_mod, &param);

                fields.push(field.gen_init_expr(opcode_mod, from_expr));

                where_clauses.push(field.ty().gen_from_where_clause(
                    opcode_mod,
                    &sexpr_mod,
                    &param_type,
                ));
                generics.push((param_type, param));
                generic_index += 1;
            } else {
                fields.push(field.gen_init_none());
            }
        }

        let body = self.gen_body_expr(fields);

        let impl_generics = generics.iter().map(|(ty, _)| ty).collect::<Vec<_>>();

        let params = generics
            .iter()
            .map(|(ty, param)| quote! { #param: #ty})
            .collect::<Vec<_>>();

        quote! {
            impl #ident {
                pub fn new<#(#impl_generics),*>(#(#params),*) -> Self where #(#where_clauses),* {
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
    fn gen_sexpr_source_codes(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut token_streams = vec![];

        token_streams.push(self.gen_sexpr_init_fns(opcode_mod));

        quote! {
            #(#token_streams)*
        }
    }
}

impl Enum {
    fn gen_sexpr_init_fns(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut fns = vec![];

        let sexpr_mod = quote! {};

        for node in &self.fields {
            if node.fields.is_empty() {
                continue;
            }

            let mut generics = vec![];
            let mut where_clauses = vec![];

            let mut fields = vec![];

            for (index, field) in node.fields.iter().enumerate() {
                let param_type = format!("P{}", index).parse().unwrap();
                let param = if let Some(ident) = field.gen_ident() {
                    ident
                } else {
                    format!("p{}", index).parse().unwrap()
                };

                let from_expr = field.ty().gen_from_expr(&sexpr_mod, &param);

                fields.push(field.gen_init_expr(opcode_mod, from_expr));

                where_clauses.push(field.ty().gen_from_where_clause(
                    opcode_mod,
                    &sexpr_mod,
                    &param_type,
                ));

                generics.push((param_type, param));
            }

            let ident = node.ident.field_ident();

            let params = generics
                .iter()
                .map(|(ty, param)| quote! { #param: #ty})
                .collect::<Vec<_>>();

            let impl_generics = generics.iter().map(|(ty, _)| ty).collect::<Vec<_>>();

            let body = node.gen_body_expr(fields);

            let field_ident = node.gen_ident();

            fns.push(quote! {
                pub fn #ident<#(#impl_generics),*> (#(#params),*) -> Self where #(#where_clauses),* {
                    Self::#field_ident #body
                }
            });
        }

        let ident = self.gen_ident();

        quote! {
            impl #opcode_mod #ident {
                #(#fns)*
            }
        }
    }
}

impl SexprGen for ApplyTo {
    fn gen_sexpr_source_codes(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut apply_to_pairs = vec![];
        for from in &self.from {
            let from = from.type_ident();

            for to in &self.to {
                apply_to_pairs.push((from.clone(), to.type_ident()));
            }
        }

        let fns = apply_to_pairs
            .iter()
            .map(|(from, to)| {
                quote! {
                    impl ApplyTo<#opcode_mod #to> for #opcode_mod #from {}
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #(#fns)*
        }
    }
}

impl SexprGen for ChildrenOf {
    fn gen_sexpr_source_codes(&self, opcode_mod: &TokenStream) -> TokenStream {
        let mut child_of_pairs = vec![];
        for from in &self.from {
            let from = from.type_ident();

            for to in &self.to {
                child_of_pairs.push((from.clone(), to.type_ident()));
            }
        }

        let fns = child_of_pairs
            .iter()
            .map(|(from, to)| {
                quote! {
                    impl ContentOf<#opcode_mod #to> for #opcode_mod #from {}
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #(#fns)*
        }
    }
}

/// A generator that create `sexpr` mod for `mlang`.
pub struct SexprModGen {
    /// Sexpr support tuple max length.
    tuple_max_len: usize,
    /// Stat mod path prefix.
    opcode_mod: TokenStream,
    /// collection of data types.
    data_types: Vec<TokenStream>,
    /// collection of attr types
    attr_types: Vec<TokenStream>,
    /// collection of el types
    el_types: Vec<TokenStream>,
    /// collection of leaf node types.
    leaf_types: Vec<TokenStream>,
    /// collection of list types.
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
            opcode_mod: String::from(opcode_mod).parse().unwrap(),
            data_types: Default::default(),
            attr_types: Default::default(),
            el_types: Default::default(),
            leaf_types: Default::default(),
            map_collect_types: Default::default(),
        }
    }
    /// Generate sexpr mod
    pub fn gen(mut self, stats: &[Stat]) -> TokenStream {
        let mut token_streams = vec![];

        for stat in stats {
            match stat {
                Stat::Element(node) => {
                    token_streams.push(node.gen_sexpr_source_codes(&self.opcode_mod));
                    self.el_types.push(node.gen_ident());
                    self.collect_map_collect_types(node);
                }
                Stat::Leaf(node) => {
                    token_streams.push(node.gen_sexpr_source_codes(&self.opcode_mod));
                    self.leaf_types.push(node.gen_ident());
                    self.collect_map_collect_types(node);
                }
                Stat::Attr(node) => {
                    token_streams.push(node.gen_sexpr_source_codes(&self.opcode_mod));
                    self.attr_types.push(node.gen_ident());
                    self.collect_map_collect_types(node);
                }
                Stat::Data(node) => {
                    token_streams.push(node.gen_sexpr_source_codes(&self.opcode_mod));
                    self.data_types.push(node.gen_ident());
                    self.collect_map_collect_types(node);
                }
                Stat::Enum(node) => {
                    token_streams.push(node.gen_sexpr_source_codes(&self.opcode_mod));
                    self.data_types.push(node.gen_ident());

                    for field in &node.fields {
                        self.collect_map_collect_types(field);
                    }
                }

                Stat::ApplyTo(node) => {
                    token_streams.push(node.gen_sexpr_source_codes(&self.opcode_mod));
                }
                Stat::ChildrenOf(node) => {
                    token_streams.push(node.gen_sexpr_source_codes(&self.opcode_mod));
                }
                _ => {}
            }
        }

        token_streams.push(self.gen_common_codes());

        token_streams.append(&mut self.gen_sexpr_graphics_impl());
        token_streams.append(&mut self.gen_sexpr_apply_fn());
        token_streams.append(&mut self.gen_sexpr_children_fn());
        token_streams.append(&mut self.gen_map_collect_impls());
        token_streams.append(&mut &mut self.gen_tuple_apply_to_impls());
        token_streams.append(&mut &mut self.gen_tuple_graphic_impls());

        quote! {
            #(#token_streams)*
        }
    }

    fn collect_map_collect_types(&mut self, node: &Node) {
        for field in node.fields.iter() {
            match field.ty() {
                Type::ListOf(component, _) => {
                    self.map_collect_types.insert(
                        component.gen_type_definition(&quote! {}).to_string(),
                        *component.clone(),
                    );
                }
                _ => {}
            }
        }
    }

    fn gen_tuple_apply_to_impls(&self) -> Vec<TokenStream> {
        let mut impls = vec![];

        for len in 2..self.tuple_max_len {
            let mut param_types = vec![];
            let mut where_clauses = vec![];

            for i in 0..len {
                let param_type: TokenStream = format!("P{}", i).parse().unwrap();

                where_clauses.push(quote! {
                    #param_type: ApplyTo<Target>
                });

                param_types.push(param_type);
            }

            impls.push(quote! {
                impl<Target, #(#param_types),*> ApplyTo<Target> for (#(#param_types),*)
                where
                #(#where_clauses),*
                {}
            });
        }

        impls
    }

    fn gen_tuple_graphic_impls(&self) -> Vec<TokenStream> {
        let mut impls = vec![];

        for len in 2..self.tuple_max_len {
            let mut param_types = vec![];
            let mut where_clauses = vec![];
            let mut fields = vec![];

            for i in 0..len {
                let param_type: TokenStream = format!("P{}", i).parse().unwrap();

                where_clauses.push(quote! {
                    #param_type: Graphics
                });

                let param: TokenStream = format!("self.{}", i).parse().unwrap();

                fields.push(quote! {#param.build(builder)});

                param_types.push(param_type);
            }

            impls.push(quote! {
                impl<#(#param_types),*> Graphics for (#(#param_types),*)
                where #(#where_clauses),*
                {
                    fn build(self,builder: &mut BuildContext) {
                        #(#fields);*
                    }
                }
            });
        }

        impls
    }

    fn gen_map_collect_impls(&self) -> Vec<TokenStream> {
        let opcode_mod = &self.opcode_mod;

        let skip = quote! { #opcode_mod Point }.to_string();

        let sexpr_mod = quote! {};
        let mut impls = vec![];
        for (_, ty) in &self.map_collect_types {
            let ty_ident = ty.gen_type_definition(&self.opcode_mod);

            for len in 1..self.tuple_max_len {
                let mut param_types = vec![];
                let mut where_clauses = vec![];
                let mut fields = vec![];

                if ty_ident.to_string() == skip && len == 2 {
                    continue;
                }

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

                if len == 1 {
                    impls.push(quote! {
                        impl<#(#param_types),*> MapCollect<#ty_ident> for #(#param_types),*
                        where #(#where_clauses),*
                        {
                            fn map_collect(self) -> Vec<#ty_ident> {
                                vec![#(#fields),*]
                            }
                        }
                    });
                } else {
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
            }
        }

        impls
    }

    fn gen_common_codes(&self) -> TokenStream {
        let opcode_mod = &self.opcode_mod;
        quote! {
            /// An attribute that can used as appliable attribute for one element must implement this trait.
            pub trait ApplyTo<E> {}

            /// An element that can used as container element's child must implement this trait.
            pub trait ContentOf<E> {}

            /// s-expr combinator must implement this trait.
            pub trait Graphics {
                /// Generate `Stat`s for specific surface.
                fn build(self, builder: &mut BuildContext);
            }

            /// build context used by [`Graphics`] trait.
            #[derive(Debug, Default)]
            pub struct BuildContext(Vec<#opcode_mod Opcode>);

            impl From<BuildContext> for Vec<#opcode_mod Opcode> {
                fn from(value: BuildContext) -> Self {
                    value.0
                }
            }

            impl AsRef<[#opcode_mod Opcode]> for BuildContext {
                fn as_ref(&self) -> &[#opcode_mod Opcode] {
                    &self.0
                }
            }

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

                /// Build a [`Graphics`] and return result ase a [`Source`].
                #[cfg(feature = "surface")]
                #[cfg_attr(docsrs, doc(cfg(feature = "surface")))]
                pub fn create_source(grapchics: impl Graphics) -> crate::surface::Source<'static> {
                    let mut builder = Self::default();
                    grapchics.build(&mut builder);

                    builder.0.into()
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

    fn gen_sexpr_graphics_impl(&self) -> Vec<TokenStream> {
        let opcode_mod = &self.opcode_mod;

        self.el_types
            .iter()
            .map(|ident| {
                quote! {
                    impl Graphics for #opcode_mod #ident {
                        fn build(self, builder: &mut BuildContext) {
                            builder.push(#opcode_mod Element::from(self));
                        }
                    }
                }
            })
            .chain(self.leaf_types.iter().map(|ident| {
                quote! {
                    impl Graphics for #opcode_mod #ident {
                        fn build(self, builder: &mut BuildContext) {
                            builder.push(#opcode_mod Leaf::from(self));
                        }
                    }
                }
            }))
            .chain(self.attr_types.iter().map(|ident| {
                quote! {
                    impl Graphics for #opcode_mod #ident {
                        fn build(self, builder: &mut BuildContext) {
                            builder.push(#opcode_mod Attr::from(self));
                        }
                    }
                }
            }))
            .collect()
    }

    fn gen_sexpr_apply_fn(&self) -> Vec<TokenStream> {
        let opcode_mod = &self.opcode_mod;

        self.el_types
            .iter()
            .map(|ident| {
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
            })
            .chain(self.leaf_types.iter().map(|ident| {
                quote! {
                    impl #opcode_mod #ident {
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
        let opcode_mod = &self.opcode_mod;
        self.el_types
            .iter()
            .map(|ident| {
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
            })
            .collect()
    }
}
