use std::collections::HashMap;

use parserc::{ControlFlow, ParseContext, Result, Span};

use crate::opcode::*;

/// Errors return by [`SemanticAnalyzer`]
#[derive(Debug, thiserror::Error, PartialEq, PartialOrd)]
pub enum MlSemanticError {
    #[error("Duplicate definition `{0}`, {1} previous definition is here.")]
    DuplicateSymbol(String, Span),

    #[error("Can't find symbol '{0}'")]
    UnknownSymbol(String),

    #[error("The element only accept leaf/element as its children, {0} type definition is here.")]
    LinkFrom(Span),

    #[error("Only the elements can have child nodes, {0} type definition is here.")]
    LinkTo(Span),

    #[error("The apply from node must be attr nodes, {0} type definition is here.")]
    ApplyFrom(Span),

    #[error("The apply to nodes must be element/leaf nodes, {0} type definition is here.")]
    ApplyTo(Span),

    #[error("Only data type can be used as field type, type definition is here({0}).")]
    TypeRef(Span),
}

#[derive(Default)]
struct SymbolTable(HashMap<String, (Span, usize)>);

impl SymbolTable {
    fn insert(&mut self, input: &mut ParseContext<'_>, ident: Ident, index: usize) {
        if let Some((other, _)) = self.0.get(&ident.0) {
            input.report_error(
                MlSemanticError::DuplicateSymbol(ident.0.clone(), *other),
                ident.1,
            );
            return;
        }

        self.0.insert(ident.0, (ident.1, index));
    }

    fn find(&self, ident: &Ident) -> Option<&(Span, usize)> {
        self.0.get(&ident.0)
    }
}

/// semantic analyzer for `mlang`.
///
/// You can create `analyzer` from [`Vec<Opcode>`], and call [`analyze`](Self::analyze)
/// to invoke real analyze produce.
pub struct SemanticAnalyzer<'a> {
    opcodes: &'a mut [Opcode],
    symbol_table: SymbolTable,
}

#[allow(unused)]
impl<'a> SemanticAnalyzer<'a> {
    /// Create a new `analyzer` from `opcodes`
    pub fn new(opcodes: &'a mut [Opcode]) -> Self {
        Self {
            opcodes,
            symbol_table: Default::default(),
        }
    }
    /// Do analyze and report any semantic error via [`ParseContext`].
    ///
    /// If any semantic error raised, this fn will returns `ControlFlow::Fatal` error.
    pub fn analyze(mut self, input: &mut ParseContext<'_>) -> Result<()> {
        let before = input.report_size();
        self.build_symbol_table(input);

        self.check_symbol(input);

        if before != input.report_size() {
            return Err(ControlFlow::Fatal);
        }

        Ok(())
    }

    fn check_symbol(&mut self, input: &mut ParseContext<'_>) {
        let mut updates = vec![];

        for (index, opcode) in self.opcodes.iter().enumerate() {
            match opcode {
                Opcode::Element(node)
                | Opcode::Leaf(node)
                | Opcode::Attr(node)
                | Opcode::Mixin(node)
                | Opcode::Data(node) => {
                    self.check_symbol_node(node, input);
                }
                Opcode::Enum(node) => self.check_symbol_enum(node, input),
                Opcode::Group(node) => self.check_symbol_group(node, input),
                Opcode::ApplyTo(node) => {
                    if let Some(update) = self.check_symbol_apply_to(node, input) {
                        updates.push((index, update));
                    }
                }
                Opcode::ChildrenOf(node) => {
                    if let Some(update) = self.check_symbol_children_of(node, input) {
                        updates.push((index, update));
                    }
                }
            }
        }

        for (index, update) in updates {
            self.opcodes[index] = update;
        }
    }

    fn symbol_lookup(&self, ident: &Ident) -> Option<(&Opcode, &Span)> {
        self.symbol_table
            .find(ident)
            .map(|(ident, index)| (&self.opcodes[*index], ident))
    }

    fn check_symbol_children_of(
        &self,
        node: &ChildrenOf,
        input: &mut ParseContext<'_>,
    ) -> Option<Opcode> {
        let mut from = vec![];
        for ident in &node.from {
            if let Some((opcode, def_span)) = self.symbol_lookup(ident) {
                match opcode {
                    Opcode::Element(_) | Opcode::Leaf(_) => {
                        from.push(ident.clone());
                    }
                    Opcode::Group(group) => {
                        let mut children = group.children.clone();

                        from.append(&mut children);
                    }
                    _ => {
                        input.report_error(MlSemanticError::LinkFrom(*def_span), ident.1.clone());
                    }
                }
            } else {
                input.report_error(
                    MlSemanticError::UnknownSymbol(ident.0.clone()),
                    ident.1.clone(),
                );
            }
        }

        let mut to = vec![];

        for ident in &node.to {
            if let Some((opcode, def_span)) = self.symbol_lookup(ident) {
                match opcode {
                    Opcode::Element(_) => {
                        to.push(ident.clone());
                    }
                    Opcode::Group(group) => {
                        let mut children = group.children.clone();

                        to.append(&mut children);
                    }
                    _ => {
                        input.report_error(MlSemanticError::LinkTo(*def_span), ident.1.clone());
                    }
                }
            } else {
                input.report_error(
                    MlSemanticError::UnknownSymbol(ident.0.clone()),
                    ident.1.clone(),
                );
            }
        }

        Some(Opcode::ChildrenOf(Box::new(ChildrenOf {
            comments: node.comments.clone(),
            properties: node.properties.clone(),
            span: node.span,
            from,
            to,
        })))
    }

    fn check_symbol_group(&self, node: &Group, input: &mut ParseContext<'_>) {
        for ident in &node.children {
            if let Some(_) = self.symbol_lookup(&ident) {
            } else {
                input.report_error(
                    MlSemanticError::UnknownSymbol(ident.0.clone()),
                    ident.1.clone(),
                );
            }
        }
    }
    fn check_symbol_apply_to(
        &self,
        node: &ApplyTo,
        input: &mut ParseContext<'_>,
    ) -> Option<Opcode> {
        let mut from = vec![];

        for ident in &node.from {
            if let Some((opcode, def_span)) = self.symbol_lookup(ident) {
                match opcode {
                    Opcode::Attr(_) => {
                        from.push(ident.clone());
                    }
                    Opcode::Group(group) => {
                        let mut children = group.children.clone();

                        from.append(&mut children);
                    }
                    _ => {
                        input.report_error(MlSemanticError::ApplyFrom(*def_span), ident.1.clone());
                    }
                }
            } else {
                input.report_error(
                    MlSemanticError::UnknownSymbol(ident.0.clone()),
                    ident.1.clone(),
                );
            }
        }

        let mut to = vec![];

        for ident in &node.to {
            if let Some((opcode, def_span)) = self.symbol_lookup(ident) {
                match opcode {
                    Opcode::Element(_) | Opcode::Leaf(_) => {
                        to.push(ident.clone());
                    }
                    Opcode::Group(group) => {
                        let mut children = group.children.clone();

                        to.append(&mut children);
                    }
                    _ => {
                        input.report_error(MlSemanticError::ApplyTo(*def_span), ident.1.clone());
                    }
                }
            } else {
                input.report_error(
                    MlSemanticError::UnknownSymbol(ident.0.clone()),
                    ident.1.clone(),
                );
            }
        }

        Some(Opcode::ApplyTo(Box::new(ApplyTo {
            comments: node.comments.clone(),
            properties: node.properties.clone(),
            span: node.span,
            from,
            to,
        })))
    }

    fn check_symbol_enum(&self, node: &Enum, input: &mut ParseContext<'_>) {
        for field in &node.fields {
            self.check_symbol_node(field, input);
        }
    }

    fn check_symbol_node(&self, node: &Node, input: &mut ParseContext<'_>) {
        for field in &node.fields {
            self.check_symbol_ty(&field.ty, input);
        }
    }

    fn check_symbol_ty(&self, ty: &Type, input: &mut ParseContext<'_>) {
        match ty {
            Type::ArrayOf(ty, _, _) => {
                self.check_symbol_ty(ty, input);
            }
            Type::ListOf(ty, _) => {
                self.check_symbol_ty(ty, input);
            }
            Type::Data(ty) => {
                if let Some((opcode, def_span)) = self.symbol_lookup(ty) {
                    match opcode {
                        Opcode::Data(_) | Opcode::Enum(_) => {}
                        _ => {
                            input.report_error(MlSemanticError::TypeRef(*def_span), ty.1);
                        }
                    }
                } else {
                    input.report_error(MlSemanticError::UnknownSymbol(ty.0.clone()), ty.1);
                }
            }
            _ => {}
        }
    }

    fn build_symbol_table(&mut self, input: &mut ParseContext<'_>) {
        for (index, opcode) in self.opcodes.iter().enumerate() {
            match opcode {
                Opcode::Element(node) => {
                    self.symbol_table.insert(input, node.ident.clone(), index);
                }
                Opcode::Leaf(node) => {
                    self.symbol_table.insert(input, node.ident.clone(), index);
                }
                Opcode::Attr(node) => {
                    self.symbol_table.insert(input, node.ident.clone(), index);
                }
                Opcode::Mixin(node) => {
                    self.symbol_table.insert(input, node.ident.clone(), index);
                }
                Opcode::Data(node) => {
                    self.symbol_table.insert(input, node.ident.clone(), index);
                }
                Opcode::Enum(node) => {
                    self.symbol_table.insert(input, node.ident.clone(), index);
                }
                Opcode::Group(node) => {
                    self.symbol_table.insert(input, node.ident.clone(), index);
                }

                _ => {}
            }
        }
    }
}
