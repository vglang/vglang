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

    fn gen_sexpr_apply_impl(&self) -> Vec<TokenStream> {
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

    fn gen_sexpr_children_impl(&self) -> Vec<TokenStream> {
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

    fn gen_sexpr_mod(&self) -> TokenStream {
        let graphics_impl = self.gen_sexpr_graphics_impl();
        let apply_impl = self.gen_sexpr_apply_impl();
        let children_impl = self.gen_sexpr_children_impl();
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

            /// build context used by [`Graphics`](super::Graphics) trait.
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

            /// A wrapper [`Graphics`] returns by [`ApplyContainer::children`] or container's `children` function.
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

            #(#graphics_impl)*

            #(#apply_impl)*

            #(#children_impl)*
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
}

struct CoreFieldGen {
    comments: TokenStream,
    ident: Option<TokenStream>,
    attrs: super::FieldAttrs,
    ty: super::FieldType,
}

impl CoreFieldGen {
    fn gen_definition(&self) -> TokenStream {
        let comments = &self.comments;

        let mut ty = TokenStream::from(self.ty.clone());

        if self.attrs.variable {
            ty = quote! { super::variable::Variable<#ty> };
        }

        if self.attrs.option {
            ty = quote! { Option<#ty> };
        }

        if let Some(ident) = &self.ident {
            quote! {
                #comments
                #ident: #ty
            }
        } else {
            quote! {
                #comments
                #ty
            }
        }
    }
}

/// The core `el/leaf/..,etc` code generator
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

        let (fields, is_tuple) = self.gen_fields_definition(mixin);

        if self.fields.is_empty() {
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
                }
            } else {
                quote! {
                    #comments
                    #[derive(Debug, PartialEq, PartialOrd, Clone)]
                    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                    pub struct #ident {
                        #(#fields),*
                    }
                }
            }
        }
    }

    fn gen_fields_definition(
        &self,
        mixin: &HashMap<String, CoreNodeGen>,
    ) -> (Vec<TokenStream>, bool) {
        let mut is_tuple = None;

        let mut fields = vec![];

        if let Some(target) = &self.mixin {
            let target = mixin.get(target).expect("Mixin not found");

            let (mut mixin_fields, mixin_is_tuple) = target.gen_fields_definition(mixin);

            is_tuple = Some(mixin_is_tuple);

            fields.append(&mut mixin_fields);
        }

        for field in &self.fields {
            if let Some(is_tuple) = is_tuple {
                if is_tuple {
                    assert!(field.ident.is_none(), "tuple field name must be none");
                } else {
                    assert!(field.ident.is_some(), "missing field name.");
                }
            } else {
                is_tuple = Some(field.ident.is_none());
            }

            fields.push(field.gen_definition());
        }

        (fields, is_tuple.unwrap_or(true))
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
            let (fields, is_tuple) = field.gen_fields_definition(mixin);

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