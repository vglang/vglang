use std::collections::HashMap;

use parserc::{ParseContext, Span};

use crate::opcode::{ApplyTo, ChildrenOf, Enum, Group, Ident, Node, Opcode, Type};

/// Errors return by semantic analyzer.
#[derive(Debug, thiserror::Error, PartialEq, PartialOrd)]
pub enum MlSemanticError {
    #[error("duplicate ident {0}, previous definition is here {1}.")]
    DuplicateSymbol(String, Span),

    #[error("expect `{0}` is a type, group definition is here {1}.")]
    UnexpectGroup(String, Span),

    #[error("undeclared type `{0}`.")]
    UndeclaredType(String),

    #[error(
        "mixin `{0}` field, named/unnamed fields can't be merged. mixin defintion is here {1}."
    )]
    UnableMixin(String, Span),
}

#[derive(Default)]
struct SymbolTable(HashMap<String, (Span, usize)>);

impl SymbolTable {
    /// Add a new symbol to the checker.
    fn add(&mut self, ctx: &mut ParseContext<'_>, index: usize, ident: &Ident) {
        if let Some((span, _)) = self.0.insert(ident.0.clone(), (ident.1, index)) {
            ctx.on_fatal(
                MlSemanticError::DuplicateSymbol(ident.0.clone(), span),
                ident.1,
            );
        }
    }

    /// Search symbol.
    fn lookup(&self, ident: &Ident) -> Option<usize> {
        self.0.get(&ident.0).map(|(_, index)| *index)
    }
}

#[derive(Default)]
struct MixinTable {
    mixin: HashMap<String, (Span, usize)>,
}

impl MixinTable {
    /// add new mixin item.
    ///
    /// This function delegate symbol conflict check to `SymbolChecker`
    fn add(&mut self, index: usize, ident: &Ident) {
        self.mixin.insert(ident.0.clone(), (ident.1, index));
    }

    fn lookup(&self, ident: &Ident) -> Option<usize> {
        self.mixin.get(ident.0.as_str()).map(|(_, index)| *index)
    }
}

#[derive(Default)]
struct GroupTable {
    groups: HashMap<String, (Span, usize)>,
}

impl GroupTable {
    /// See a group item.
    fn add(&mut self, index: usize, ident: &Ident) {
        self.groups.insert(ident.0.clone(), (ident.1, index));
    }

    fn lookup(&self, ident: &Ident) -> Option<usize> {
        self.groups.get(ident.0.as_str()).map(|(_, index)| *index)
    }
}

/// A semantic analyzer for `mlang`.
#[derive(Default)]
struct SemanticAnalyzer<'a> {
    opcodes: &'a mut [Opcode],
    /// A symbol index database.
    symbol_table: SymbolTable,
    /// A mixin fields merger.
    merger: MixinTable,
    /// `apply..to..` `chidlren..of..` syntax checker.
    digraph_analyzer: GroupTable,
}

impl<'a> SemanticAnalyzer<'a> {
    fn new(opcodes: &'a mut [Opcode]) -> Self {
        Self {
            opcodes,
            ..Default::default()
        }
    }

    fn analyze(&mut self, ctx: &mut ParseContext<'_>) {
        self.build_index(ctx);
        self.check(ctx);
    }

    fn build_index(&mut self, ctx: &mut ParseContext<'_>) {
        for (index, opcode) in self.opcodes.iter().enumerate() {
            match opcode {
                Opcode::Element(node)
                | Opcode::Leaf(node)
                | Opcode::Attr(node)
                | Opcode::Data(node) => {
                    self.symbol_table.add(ctx, index, &node.ident);
                }
                Opcode::Mixin(node) => {
                    self.symbol_table.add(ctx, index, &node.ident);
                    self.merger.add(index, &node.ident);
                }
                Opcode::Enum(node) => {
                    self.symbol_table.add(ctx, index, &node.ident);
                }
                Opcode::Group(node) => {
                    self.symbol_table.add(ctx, index, &node.ident);
                    self.digraph_analyzer.add(index, &node.ident);
                }
                Opcode::ApplyTo(_) => {}
                Opcode::ChildrenOf(_) => {}
            }
        }
    }

    fn check(&mut self, ctx: &mut ParseContext<'_>) {
        let mut updates = vec![];
        for (index, opcode) in self.opcodes.iter().enumerate() {
            match opcode {
                Opcode::Element(node) => {
                    if let Some(node) = self.node_check(ctx, node) {
                        updates.push((index, Opcode::Element(Box::new(node))));
                    }
                }
                Opcode::Leaf(node) => {
                    if let Some(node) = self.node_check(ctx, node) {
                        updates.push((index, Opcode::Leaf(Box::new(node))));
                    }
                }
                Opcode::Attr(node) => {
                    if let Some(node) = self.node_check(ctx, node) {
                        updates.push((index, Opcode::Attr(Box::new(node))));
                    }
                }
                Opcode::Mixin(node) => {
                    assert_eq!(
                        self.node_check(ctx, node),
                        None,
                        "Mixin: inner error, mixin can't mixin other one."
                    );
                }
                Opcode::Data(node) => {
                    if let Some(node) = self.node_check(ctx, node) {
                        updates.push((index, Opcode::Data(Box::new(node))));
                    }
                }
                Opcode::Enum(node) => {
                    self.enum_check(ctx, node);
                }
                Opcode::Group(group) => {
                    self.group_check(ctx, group);
                }
                Opcode::ApplyTo(apply_to) => {
                    if let Some(opcode) = self.apply_to_check(ctx, apply_to) {
                        updates.push((index, opcode));
                    }
                }
                Opcode::ChildrenOf(children_of) => {
                    if let Some(opcode) = self.children_of_check(ctx, children_of) {
                        updates.push((index, opcode));
                    }
                }
            }
        }

        for (index, update) in updates {
            self.opcodes[index] = update;
        }
    }

    fn symbol_check(&self, ctx: &mut ParseContext<'_>, ident: &Ident, expect_type: bool) -> bool {
        if let Some(index) = self.symbol_table.lookup(ident) {
            if let Opcode::Group(group) = &self.opcodes[index] {
                if expect_type {
                    ctx.on_fatal(
                        MlSemanticError::UnexpectGroup(group.ident.0.clone(), group.ident.1),
                        ident.1,
                    );
                }

                return false;
            }

            return true;
        } else {
            ctx.on_fatal(MlSemanticError::UndeclaredType(ident.0.clone()), ident.1);
            return false;
        }
    }

    fn type_check(&self, ctx: &mut ParseContext<'_>, ty: &Type) {
        match ty {
            Type::Data(ident) => {
                self.symbol_check(ctx, ident, true);
            }

            Type::ListOf(component, _) => {
                self.type_check(ctx, component);
            }
            Type::ArrayOf(component, _, _) => {
                self.type_check(ctx, component);
            }
            _ => {}
        }
    }

    fn node_check(&self, ctx: &mut ParseContext<'_>, node: &Node) -> Option<Node> {
        for field in node.fields.iter() {
            self.type_check(ctx, &field.ty());
        }

        if let Some(mixin) = &node.mixin {
            if let Some(index) = self.merger.lookup(mixin) {
                if let Opcode::Mixin(mixin) = &self.opcodes[index] {
                    let expand = mixin.fields.clone();
                    let fields = node.fields.clone();

                    let fields = match fields.append(expand) {
                        Ok(fields) => fields,
                        Err(fields) => {
                            ctx.on_fatal(
                                MlSemanticError::UnableMixin(mixin.ident.0.clone(), mixin.ident.1),
                                node.ident.1,
                            );
                            fields
                        }
                    };

                    return Some(Node {
                        comments: node.comments.clone(),
                        mixin: None,
                        properties: node.properties.clone(),
                        ident: node.ident.clone(),
                        fields,
                    });
                } else {
                    panic!("node_check(mxin): inner error.");
                }
            } else {
                ctx.on_fatal(MlSemanticError::UndeclaredType(mixin.0.clone()), mixin.1);
                return None;
            }
        }

        return None;
    }

    fn enum_check(&self, ctx: &mut ParseContext<'_>, node: &Enum) {
        for field_node in &node.fields {
            for field in field_node.fields.iter() {
                self.type_check(ctx, field.ty());
            }
        }
    }

    fn group_check(&self, ctx: &mut ParseContext<'_>, node: &Group) {
        for ident in &node.children {
            self.symbol_check(ctx, ident, true);
        }
    }

    fn expand_with_group(&self, ctx: &mut ParseContext<'_>, ident: &Ident) -> Option<Vec<Ident>> {
        if let Some(index) = self.digraph_analyzer.lookup(ident) {
            if let Opcode::Group(group) = &self.opcodes[index] {
                return Some(group.children.clone());
            } else {
                panic!("expand_with_group: inner error.");
            }
        } else {
            ctx.on_fatal(MlSemanticError::UndeclaredType(ident.0.clone()), ident.1);
            None
        }
    }

    fn apply_to_check(&self, ctx: &mut ParseContext<'_>, node: &ApplyTo) -> Option<Opcode> {
        let mut from_expand = vec![];

        for ident in &node.from {
            if !self.symbol_check(ctx, ident, false) {
                if let Some(mut expand) = self.expand_with_group(ctx, ident) {
                    from_expand.append(&mut expand);
                }
            } else {
                from_expand.push(ident.clone());
            }
        }

        let mut to_expand = vec![];

        for ident in &node.to {
            if !self.symbol_check(ctx, ident, false) {
                if let Some(mut expand) = self.expand_with_group(ctx, ident) {
                    to_expand.append(&mut expand);
                }
            } else {
                to_expand.push(ident.clone());
            }
        }

        Some(Opcode::ApplyTo(Box::new(ApplyTo {
            from: from_expand,
            to: to_expand,
            span: node.span,
            comments: node.comments.clone(),
            properties: node.properties.clone(),
        })))
    }

    fn children_of_check(&self, ctx: &mut ParseContext<'_>, node: &ChildrenOf) -> Option<Opcode> {
        let mut from_expand = vec![];

        for ident in &node.from {
            if !self.symbol_check(ctx, ident, false) {
                if let Some(mut expand) = self.expand_with_group(ctx, ident) {
                    from_expand.append(&mut expand);
                }
            } else {
                from_expand.push(ident.clone());
            }
        }

        let mut to_expand = vec![];

        for ident in &node.to {
            if !self.symbol_check(ctx, ident, false) {
                if let Some(mut expand) = self.expand_with_group(ctx, ident) {
                    to_expand.append(&mut expand);
                }
            } else {
                to_expand.push(ident.clone());
            }
        }

        Some(Opcode::ChildrenOf(Box::new(ChildrenOf {
            from: from_expand,
            to: to_expand,
            span: node.span,
            comments: node.comments.clone(),
            properties: node.properties.clone(),
        })))
    }
}

/// Process semantic analyze on `opcodes` slice.
pub fn semantic_analyze(opcodes: &mut [Opcode], ctx: &mut ParseContext<'_>) {
    SemanticAnalyzer::new(opcodes).analyze(ctx);
}
