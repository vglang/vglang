use std::collections::HashMap;

use parserc::{ControlFlow, ParseContext, Result, Span};

use crate::opcode::*;

/// Errors return by [`SemanticAnalyzer`]
#[derive(Debug, thiserror::Error, PartialEq, PartialOrd)]
pub enum MlSemanticError {
    #[error("Duplicate definition `{0}`, {1} previous definition is here.")]
    DuplicateSymbol(String, Span),
}

#[derive(Default)]
struct SymbolTable(HashMap<String, (Ident, usize)>);

impl SymbolTable {
    fn insert(&mut self, input: &mut ParseContext<'_>, ident: Ident, index: usize) {
        if let Some((other, _)) = self.0.get(&ident.0) {
            input.report_error(
                MlSemanticError::DuplicateSymbol(other.0.clone(), other.1),
                ident.1,
            );
            return;
        }

        self.0.insert(ident.0.clone(), (ident, index));
    }
}

/// semantic analyzer for `mlang`.
///
/// You can create `analyzer` from [`Vec<Opcode>`], and call [`analyze`](Self::analyze)
/// to invoke real analyze produce.
pub struct SemanticAnalyzer<'a> {
    opcodes: &'a [Opcode],
    symbol_table: SymbolTable,
}

#[allow(unused)]
impl<'a> SemanticAnalyzer<'a> {
    /// Create a new `analyzer` from `opcodes`
    pub fn new(opcodes: &'a [Opcode]) -> Self {
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

    fn check_symbol(&self, input: &mut ParseContext<'_>) {
        for opcode in self.opcodes {
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
                Opcode::ApplyTo(node) => self.check_symbol_apply_to(node, input),
                Opcode::ChildrenOf(node) => self.check_symbol_children_of(node, input),
            }
        }
    }

    fn check_symbol_children_of(&self, node: &ChildrenOf, input: &mut ParseContext<'_>) {}
    fn check_symbol_apply_to(&self, node: &ApplyTo, input: &mut ParseContext<'_>) {}
    fn check_symbol_group(&self, node: &Group, input: &mut ParseContext<'_>) {}
    fn check_symbol_enum(&self, node: &Enum, input: &mut ParseContext<'_>) {}

    fn check_symbol_node(&self, node: &Node, input: &mut ParseContext<'_>) {}

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
