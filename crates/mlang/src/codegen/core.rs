use std::collections::HashMap;

use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{CodeGen, EnumCodeGen, NodeCodeGen};

/// `mlang` core codes generator.
#[derive(Default)]
pub struct CoreGen {
    mixin_gens: HashMap<String, CoreNodeGen>,
    node_gens: Vec<CoreNodeGen>,
    enum_gens: Vec<CoreEnumGen>,
    attr_fileds: Vec<TokenStream>,
    el_fileds: Vec<TokenStream>,
    leaf_fields: Vec<TokenStream>,
    data_fields: Vec<TokenStream>,
    child_of: Vec<(TokenStream, TokenStream)>,
    apply_to: Vec<(TokenStream, TokenStream)>,
}

impl CoreGen {
    fn gen_variable_mod(&self) -> TokenStream {
        quote! {
            /// The path used by [`Variable`] is used to point to [`Target`].
            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Path {
                /// Reference by name.
                Named(String),
                /// Reference by optimized position.
                Index(usize),
            }

            impl From<String> for Path {
                fn from(value: String) -> Self {
                    Self::Named(value)
                }
            }

            impl From<&str> for Path {
                fn from(value: &str) -> Self {
                    Self::Named(value.to_owned())
                }
            }

            impl From<usize> for Path {
                fn from(value: usize) -> Self {
                    Self::Index(value)
                }
            }

            /// The type of variable pointed to by [`Path`].
            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Target {
                /// Target is animation register.
                Register,
                /// Target is `item` variable returns by `foreach` iterator.
                ForeachItem,
                /// Target is `index` variable returns by `foreach` iterator.
                ForeachIndex,
                /// Target is `index` variable returns by `for range` iterator.
                Range,
            }

            /// Variable used by property fields.
            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Variable<T>
            where
                super::opcode::Data: From<T>,
            {
                /// A literal constant value.
                Constant(T),

                Reference {
                    path: Path,
                    target: Target,
                },
            }

            impl<T> From<T> for Variable<T>
            where
                super::opcode::Data: From<T>,
            {
                fn from(value: T) -> Self {
                    Self::Constant(value)
                }
            }

            impl<T> Default for Variable<T>
            where
                T: Default,
                super::opcode::Data: From<T>,
            {
                fn default() -> Self {
                    Self::Constant(T::default())
                }
            }

            impl<P, T> From<(P, Target)> for Variable<T>
            where
                super::opcode::Data: From<T>,
                Path: From<P>,
            {
                fn from(value: (P, Target)) -> Self {
                    Self::Reference {
                        path: value.0.into(),
                        target: value.1,
                    }
                }
            }

        }
    }

    fn gen_data_definition(&self) -> TokenStream {
        let mut fields = vec![];
        let mut froms = vec![];

        let builtin_types = vec![
            ("bool", "bool"),
            ("string", "String"),
            ("byte", "i8"),
            ("ubyte", "u8"),
            ("short", "i16"),
            ("ushort", "u16"),
            ("int", "i32"),
            ("uint", "u32"),
            ("long", "i64"),
            ("ulong", "u64"),
            ("float", "f32"),
            ("double", "f64"),
        ];

        for (ident, ty) in builtin_types {
            let name = ident.to_upper_camel_case();
            let ty: TokenStream = ty.parse().unwrap();

            let ident: TokenStream = name.parse().unwrap();
            let list_ident = format_ident!("ListOf{}", name);

            fields.push(quote! {
                #ident(#ty),#list_ident(Box<Vec<#ty>>)
            });

            froms.push(quote! {
                impl From<#ty> for Data {
                    fn from(value: #ty) -> Self {
                        Data::#ident(value)
                    }
                }

                impl From<Vec<#ty>> for Data {
                    fn from(value: Vec<#ty>) -> Self {
                        Data::#list_ident(Box::new(value))
                    }
                }
            });
        }

        for ident in &self.data_fields {
            let list_ident = format_ident!("ListOf{}", ident.to_string());

            fields.push(quote! {
                #ident(Box<#ident>),#list_ident(Box<Vec<#ident>>)
            });

            froms.push(quote! {
                impl From<#ident> for Data {
                    fn from(value: #ident) -> Self {
                        Data::#ident(Box::new(value))
                    }
                }

                impl From<Vec<#ident>> for Data {
                    fn from(value: Vec<#ident>) -> Self {
                        Data::#list_ident(Box::new(value))
                    }
                }
            });
        }

        quote! {
            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Data {
                #(#fields),*
            }

            #(#froms)*
        }
    }

    fn gen_el_definition(&self) -> TokenStream {
        let mut fields = vec![];
        let mut froms = vec![];

        for el in &self.el_fileds {
            fields.push(quote! {
                #el(Box<#el>)
            });

            froms.push(quote! {
                impl From<#el> for Element {
                    fn from(value: #el) -> Self {
                        Self::#el(Box::new(value))
                    }
                }
            });
        }

        quote! {

            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Element {
                #(#fields),*
            }

            #(#froms)*
        }
    }

    fn gen_leaf_definition(&self) -> TokenStream {
        let mut fields = vec![];
        let mut froms = vec![];

        for el in &self.leaf_fields {
            fields.push(quote! {
                #el(Box<#el>)
            });

            froms.push(quote! {
                impl From<#el> for Leaf {
                    fn from(value: #el) -> Self {
                        Self::#el(Box::new(value))
                    }
                }
            });
        }

        quote! {

            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Leaf {
                #(#fields),*
            }

            #(#froms)*
        }
    }

    fn gen_attr_definition(&self) -> TokenStream {
        let mut fields = vec![];
        let mut froms = vec![];

        for el in &self.attr_fileds {
            fields.push(quote! {
                #el(Box<#el>)
            });

            froms.push(quote! {
                impl From<#el> for Attr {
                    fn from(value: #el) -> Self {
                        Self::#el(Box::new(value))
                    }
                }
            });
        }

        quote! {

            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Attr {
                #(#fields),*
            }

            #(#froms)*
        }
    }

    fn gen_opcode_definition(&self) -> TokenStream {
        quote! {
            #[derive(Debug, Clone, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Opcode {
                Apply(Attr),
                Element(Element),
                Pop,
                Leaf(Leaf),
            }

            impl From<Attr> for Opcode {
                fn from(value: Attr) -> Self {
                    Self::Apply(value)
                }
            }

            impl From<Element> for Opcode {
                fn from(value: Element) -> Self {
                    Self::Element(value)
                }
            }

            impl From<Leaf> for Opcode {
                fn from(value: Leaf) -> Self {
                    Self::Leaf(value)
                }
            }
        }
    }

    fn gen_sexpr_graphics_impl(&self) -> Vec<TokenStream> {
        self.el_fileds
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
            .chain(self.leaf_fields.iter().map(|ident| {
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
        self.el_fileds
            .iter()
            .map(|ident| {
                quote! {
                    impl super::opcode::#ident {
                        pub fn apply<A>(self, attrs: A) -> ApplyElement<A,Self> {
                            ApplyElement { attrs,node: self }
                        }
                    }
                }
            })
            .chain(self.leaf_fields.iter().map(|ident| {
                quote! {
                    impl super::opcode::#ident {
                        pub fn apply<A>(self, attrs: A) -> ApplyLeaf<A,Self>{
                            ApplyLeaf { attrs,node: self }
                        }
                    }
                }
            }))
            .collect()
    }

    fn gen_sexpr_children_fn(&self) -> Vec<TokenStream> {
        self.el_fileds
            .iter()
            .map(|ident| {
                quote! {
                    impl super::opcode::#ident {
                        pub fn children<C>(self, children: C) -> ElementChildren<Self,C> {
                            ElementChildren { node: self, children }
                        }
                    }
                }
            })
            .collect()
    }

    fn gen_sexpr_apply_to_impl(&self) -> Vec<TokenStream> {
        self.apply_to
            .iter()
            .map(|(from, to)| {
                quote! {
                    impl ApplyTo<super::opcode::#to> for super::opcode::#from {}
                }
            })
            .collect()
    }

    fn gen_sexpr_child_of_impl(&self) -> Vec<TokenStream> {
        self.child_of
            .iter()
            .map(|(from, to)| {
                quote! {
                    impl ContentOf<super::opcode::#to> for super::opcode::#from {}
                }
            })
            .collect()
    }

    fn gen_sexpr_mod(&self) -> TokenStream {
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
}

impl CodeGen for CoreGen {
    type Node = CoreNodeGen;

    type Enum = CoreEnumGen;

    fn create_node(&mut self, comments: TokenStream, ident: TokenStream) -> Self::Node {
        CoreNodeGen {
            comments,
            ident,
            fields: Default::default(),
            mixin: None,
        }
    }

    fn create_enum(&mut self, comments: TokenStream, ident: TokenStream) -> Self::Enum {
        CoreEnumGen {
            comments,
            ident,
            fields: Default::default(),
        }
    }

    fn push_mixin(&mut self, el: Self::Node) {
        self.mixin_gens.insert(el.ident.to_string(), el);
    }

    fn push_el(&mut self, el: Self::Node) {
        self.el_fileds.push(el.ident.clone());
        self.node_gens.push(el);
    }

    fn push_leaf(&mut self, el: Self::Node) {
        self.leaf_fields.push(el.ident.clone());
        self.node_gens.push(el);
    }

    fn push_attr(&mut self, el: Self::Node) {
        self.attr_fileds.push(el.ident.clone());
        self.node_gens.push(el);
    }

    fn push_data(&mut self, el: Self::Node) {
        self.data_fields.push(el.ident.clone());
        self.node_gens.push(el);
    }

    fn push_enum(&mut self, node: Self::Enum) {
        self.data_fields.push(node.ident.clone());
        self.enum_gens.push(node);
    }

    fn gen(self) -> TokenStream {
        let node_definitions = self
            .node_gens
            .iter()
            .map(|node| node.gen_definition(&self.mixin_gens))
            .collect::<Vec<_>>();

        let enum_definitions = self
            .enum_gens
            .iter()
            .map(|node| node.gen_definition(&self.mixin_gens))
            .collect::<Vec<_>>();

        let data_definition = self.gen_data_definition();
        let el_definition = self.gen_el_definition();
        let leaf_definition = self.gen_leaf_definition();
        let attr_definition = self.gen_attr_definition();
        let opcode_definition = self.gen_opcode_definition();

        let variables_mod = self.gen_variable_mod();
        let sexpr_mod = self.gen_sexpr_mod();

        quote! {
            pub mod opcode {
                #(#node_definitions)*
                #(#enum_definitions)*
                #data_definition
                #el_definition
                #leaf_definition
                #attr_definition
                #opcode_definition
            }

            pub mod variable {
                #variables_mod
            }

            pub mod sexpr {
                #sexpr_mod
            }
        }
    }

    fn push_apply_to(&mut self, from: TokenStream, to: TokenStream) {
        self.apply_to.push((from, to));
    }

    fn push_child_of(&mut self, from: TokenStream, to: TokenStream) {
        self.child_of.push((from, to));
    }
}

#[derive(Clone)]
struct CoreFieldGen {
    comments: TokenStream,
    ident: Option<TokenStream>,
    attrs: super::FieldAttrs,
    ty: super::FieldType,
}

impl CoreFieldGen {
    fn sexpr_where_type(token_stream: &TokenStream) -> TokenStream {
        match token_stream.to_string().as_str() {
            "f32" => {
                quote! {
                    super::sexpr::Number
                }
            }
            _ => token_stream.clone(),
        }
    }

    fn sexpr_into(ty: &TokenStream, param: &TokenStream) -> TokenStream {
        match ty.to_string().as_str() {
            "f32" => {
                quote! {
                    super::sexpr::Number::from(#param).0
                }
            }
            _ => quote! {
                #param.into()
            },
        }
    }
    fn gen_definition(&self, is_enum: bool) -> TokenStream {
        let comments = &self.comments;

        let mut ty = TokenStream::from(self.ty.clone());

        if self.attrs.variable {
            ty = quote! { super::variable::Variable<#ty> };
        }

        if self.attrs.option {
            ty = quote! { Option<#ty> };
        }

        let vis = if is_enum {
            quote! {}
        } else {
            quote! { pub }
        };

        if let Some(ident) = &self.ident {
            quote! {
                #comments
                #vis #ident: #ty
            }
        } else {
            quote! {
                #comments
                #vis #ty
            }
        }
    }
}

/// The core `el/leaf/..,etc` code generator
#[derive(Clone)]
pub struct CoreNodeGen {
    mixin: Option<String>,
    comments: TokenStream,
    ident: TokenStream,
    fields: Vec<CoreFieldGen>,
}

impl CoreNodeGen {
    fn gen_definition(&self, mixin: &HashMap<String, CoreNodeGen>) -> TokenStream {
        let comments = &self.comments;
        let ident = &self.ident;

        let (fields, is_tuple) = self.gen_fields_definition(false, mixin);

        println!(
            "{}: \n{}",
            self.ident,
            fields
                .iter()
                .map(|token| token.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        );

        let fns = self.gen_init_fns(mixin);

        if fields.is_empty() {
            quote! {
                #comments
                #[derive(Debug, PartialEq, PartialOrd, Clone)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct #ident;
            }
        } else {
            if is_tuple {
                quote! {
                    #comments
                    #[derive(Debug, PartialEq, PartialOrd, Clone)]
                    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                    pub struct #ident(#(#fields),*);

                    #fns
                }
            } else {
                quote! {
                    #comments
                    #[derive(Debug, PartialEq, PartialOrd, Clone)]
                    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                    pub struct #ident {
                        #(#fields),*
                    }

                    #fns
                }
            }
        }
    }

    fn gen_init_fns(&self, mixin: &HashMap<String, CoreNodeGen>) -> TokenStream {
        let ident = &self.ident;

        let mut fields = self.fields.clone();

        if let Some(target) = &self.mixin {
            let target = mixin.get(target).expect("Mixin not found");

            let mut mixin_fields = target.fields.clone();

            fields.append(&mut mixin_fields);
        }

        if fields.is_empty() {
            return quote! {};
        }

        let is_tuple = fields.first().unwrap().ident.is_none();

        let count = fields.iter().filter(|f| !f.attrs.option).count();

        if count == 0 {
            let assign = if is_tuple {
                (0..fields.len()).map(|_| quote! {None}).collect::<Vec<_>>()
            } else {
                fields
                    .iter()
                    .map(|field| {
                        let ident = field.ident.as_ref().unwrap();
                        quote! {
                            #ident: None
                        }
                    })
                    .collect()
            };

            if is_tuple {
                return quote! {
                    impl Default for #ident {
                        fn default() -> Self {
                            Self(#(#assign),*)
                        }
                    }
                };
            } else {
                return quote! {
                    impl Default for #ident {
                        fn default() -> Self {
                            Self { #(#assign),* }
                        }
                    }
                };
            }
        }

        let mut index = 0;

        let mut generics = vec![];

        let mut where_clause = vec![];

        let mut assign = vec![];

        for field in &fields {
            let ident = if let Some(ident) = &field.ident {
                quote! {#ident: }
            } else {
                quote! {}
            };

            if field.attrs.option {
                assign.push(quote! {
                    #ident None
                });
            } else {
                let param_type = format!("P{}", index).parse::<TokenStream>().unwrap();

                let param = if count > 1 {
                    format!("value.{}", index).parse::<TokenStream>().unwrap()
                } else {
                    format!("value").parse::<TokenStream>().unwrap()
                };

                index += 1;

                match &field.ty {
                    crate::codegen::FieldType::Noraml(token_stream) => {
                        let into_expr = CoreFieldGen::sexpr_into(token_stream, &param);
                        let where_type = CoreFieldGen::sexpr_where_type(token_stream);
                        where_clause.push(quote! {
                            #where_type: From<#param_type>
                        });
                        if field.attrs.variable {
                            assign.push(quote! {
                                #ident super::variable::Variable::Constant(#into_expr)
                            });
                        } else {
                            assign.push(quote! {
                                #ident #into_expr
                            });
                        }
                    }
                    crate::codegen::FieldType::List(token_stream) => {
                        where_clause.push(quote! {
                            #param_type: super::sexpr::MapCollect<#token_stream>
                        });
                        if field.attrs.variable {
                            assign.push(quote! {
                                #ident super::variable::Variable::Constant(#param.map_collect())
                            });
                        } else {
                            assign.push(quote! {
                                #ident #param.map_collect()
                            });
                        }
                    }
                    crate::codegen::FieldType::Array(token_stream, num) => {
                        where_clause.push(quote! {
                            [#token_stream;#num]: From<#param_type>
                        });
                        if field.attrs.variable {
                            assign.push(quote! {
                                #ident super::variable::Variable::Constant(#param.into())
                            });
                        } else {
                            assign.push(quote! {
                                #ident #param.into()
                            });
                        }
                    }
                }

                generics.push(param_type);
            }
        }

        let impl_generics = quote! {#(#generics),*};

        let type_generics = if generics.len() > 1 {
            quote! {
                (#(#generics),*)
            }
        } else {
            quote! {
                #(#generics),*
            }
        };

        if is_tuple {
            quote! {
                impl<#impl_generics> From<#type_generics> for #ident where #(#where_clause),* {
                    fn from(value: #type_generics) -> Self {
                        Self (
                            #(#assign),*
                        )
                    }
                }
            }
        } else {
            quote! {
                impl<#impl_generics> From<#type_generics> for #ident where #(#where_clause),* {
                    fn from(value: #type_generics) -> Self {
                        Self {
                            #(#assign),*
                        }
                    }
                }
            }
        }
    }

    fn gen_fields_definition(
        &self,
        is_enum: bool,
        mixin: &HashMap<String, CoreNodeGen>,
    ) -> (Vec<TokenStream>, bool) {
        let mut is_tuple = None;

        let mut fields = self.fields.clone();

        if let Some(target) = &self.mixin {
            let target = mixin.get(target).expect("Mixin not found");

            let mut mixin_fields = target.fields.clone();

            fields.append(&mut mixin_fields);
        }

        let mut token_streams = vec![];

        for field in &fields {
            if let Some(is_tuple) = is_tuple {
                if is_tuple {
                    assert!(field.ident.is_none(), "tuple field name must be none");
                } else {
                    assert!(field.ident.is_some(), "missing field name.");
                }
            } else {
                is_tuple = Some(field.ident.is_none());
            }

            token_streams.push(field.gen_definition(is_enum));
        }

        (token_streams, is_tuple.unwrap_or(true))
    }
}

impl NodeCodeGen for CoreNodeGen {
    fn mixin(&mut self, target: String) {
        self.mixin = Some(target)
    }
    fn push_field(
        &mut self,
        comments: TokenStream,
        ident: Option<TokenStream>,
        attrs: super::FieldAttrs,
        ty: super::FieldType,
    ) {
        self.fields.push(CoreFieldGen {
            comments,
            ident,
            attrs,
            ty,
        });
    }
}

/// The core `enum` code generator
pub struct CoreEnumGen {
    comments: TokenStream,
    ident: TokenStream,
    fields: Vec<CoreNodeGen>,
}

impl CoreEnumGen {
    fn gen_definition(&self, mixin: &HashMap<String, CoreNodeGen>) -> TokenStream {
        let mut token_streams = vec![];
        for field in &self.fields {
            let ident = &field.ident;
            let (fields, is_tuple) = field.gen_fields_definition(true, mixin);

            if fields.is_empty() {
                token_streams.push(quote! { #ident });
            } else {
                if is_tuple {
                    token_streams.push(quote! { #ident(#(#fields),*) });
                } else {
                    token_streams.push(quote! { #ident{ #(#fields),* } });
                }
            }
        }

        let ident = &self.ident;
        let comments = &self.comments;

        quote! {
            #comments
            #[derive(Debug, PartialEq, PartialOrd, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #ident {
                #(#token_streams),*
            }
        }
    }
}

impl EnumCodeGen for CoreEnumGen {
    type Node = CoreNodeGen;

    fn create_field(&mut self, comments: TokenStream, ident: TokenStream) -> Self::Node {
        CoreNodeGen {
            comments,
            ident,
            fields: Default::default(),
            mixin: None,
        }
    }

    fn push_field(&mut self, el: Self::Node) {
        self.fields.push(el);
    }
}
