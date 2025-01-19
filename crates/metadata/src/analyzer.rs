use std::collections::HashMap;

use parserc::{ParseContext, Span};

use crate::ir::*;

/// Error report by semantic analyze step.
#[derive(Debug, thiserror::Error)]
pub enum AnalyzerError {
    #[error("duplicate symbol `{0}`, previous declaration is here {1}")]
    Duplicate(String, Span),

    #[error("Unknown symbol `{0}`.")]
    Unknown(String),

    #[error("Use group `{0}` as field type declaration, group declaration is here {1}")]
    Group(String, Span),

    #[error("Unable merge mixin({0})'s fields into node, mixin declaration is here {1}.")]
    Merge(String, Span),

    #[error("Custom property `{0}`, expect empty call list.")]
    VariableOption(String),

    #[error("Custom property `xml`, expect one `literial str` as call list.")]
    Xml,
}

#[derive(Default)]
struct SymbolTable(HashMap<String, (Span, usize)>);

impl SymbolTable {
    /// Add a new symbol to the checker.
    fn add(&mut self, ctx: &mut ParseContext<'_>, index: usize, ident: &Ident) {
        if let Some((span, _)) = self.0.insert(ident.1.clone(), (ident.0, index)) {
            ctx.report_err(AnalyzerError::Duplicate(ident.1.clone(), span), ident.0);
        }
    }

    /// Search symbol.
    fn lookup(&self, ident: &Ident) -> Option<usize> {
        self.0.get(&ident.1).map(|(_, index)| *index)
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
        self.mixin.insert(ident.1.clone(), (ident.0, index));
    }

    fn lookup(&self, ident: &Ident) -> Option<usize> {
        self.mixin.get(ident.1.as_str()).map(|(_, index)| *index)
    }
}

#[derive(Default)]
struct GroupTable {
    groups: HashMap<String, (Span, usize)>,
}

impl GroupTable {
    /// See a group item.
    fn add(&mut self, index: usize, ident: &Ident) {
        self.groups.insert(ident.1.clone(), (ident.0, index));
    }

    fn lookup(&self, ident: &Ident) -> Option<usize> {
        self.groups.get(ident.1.as_str()).map(|(_, index)| *index)
    }
}

/// A semantic analyzer for `mlang`.
#[derive(Default)]
struct SemanticAnalyzer<'a> {
    opcodes: &'a mut [Stat],
    /// A symbol index database.
    symbol_table: SymbolTable,
    /// A mixin fields merger.
    merger: MixinTable,
    /// `apply..to..` `chidlren..of..` syntax checker.
    digraph_analyzer: GroupTable,
}

impl<'a> SemanticAnalyzer<'a> {
    fn new(opcodes: &'a mut [Stat]) -> Self {
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
                Stat::Element(node) | Stat::Leaf(node) | Stat::Attr(node) | Stat::Data(node) => {
                    self.symbol_table.add(ctx, index, &node.ident);
                }
                Stat::Mixin(node) => {
                    self.symbol_table.add(ctx, index, &node.ident);
                    self.merger.add(index, &node.ident);
                }
                Stat::Enum(node) => {
                    self.symbol_table.add(ctx, index, &node.ident);
                }
                Stat::Group(node) => {
                    self.symbol_table.add(ctx, index, &node.ident);
                    self.digraph_analyzer.add(index, &node.ident);
                }
                Stat::ApplyTo(_) => {}
                Stat::ChildrenOf(_) => {}
            }
        }
    }

    fn check(&mut self, ctx: &mut ParseContext<'_>) {
        let mut updates = vec![];
        for (index, opcode) in self.opcodes.iter().enumerate() {
            match opcode {
                Stat::Element(node) => {
                    if let Some(node) = self.node_check(ctx, node) {
                        updates.push((index, Stat::Element(Box::new(node))));
                    }
                }
                Stat::Leaf(node) => {
                    if let Some(node) = self.node_check(ctx, node) {
                        updates.push((index, Stat::Leaf(Box::new(node))));
                    }
                }
                Stat::Attr(node) => {
                    if let Some(node) = self.node_check(ctx, node) {
                        updates.push((index, Stat::Attr(Box::new(node))));
                    }
                }
                Stat::Mixin(node) => {
                    assert_eq!(
                        self.node_check(ctx, node),
                        None,
                        "Mixin: inner error, mixin can't mixin other one."
                    );
                }
                Stat::Data(node) => {
                    if let Some(node) = self.node_check(ctx, node) {
                        updates.push((index, Stat::Data(Box::new(node))));
                    }
                }
                Stat::Enum(node) => {
                    self.enum_check(ctx, node);
                }
                Stat::Group(group) => {
                    self.group_check(ctx, group);
                }
                Stat::ApplyTo(apply_to) => {
                    if let Some(opcode) = self.apply_to_check(ctx, apply_to) {
                        updates.push((index, opcode));
                    }
                }
                Stat::ChildrenOf(children_of) => {
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
            if let Stat::Group(group) = &self.opcodes[index] {
                if expect_type {
                    ctx.report_err(
                        AnalyzerError::Group(group.ident.1.clone(), group.ident.0),
                        ident.0,
                    );
                }

                return false;
            }

            return true;
        } else {
            ctx.report_err(AnalyzerError::Unknown(ident.1.clone()), ident.0);
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

        for property in &node.properties {
            for call in &property.calls {
                match call.target.1.as_str() {
                    "option" | "variable" | "init" | "xml_skip" => {
                        if call.params.len() != 0 {
                            ctx.report_err(
                                AnalyzerError::VariableOption(call.target.1.clone()),
                                call.target.0,
                            );
                        }
                    }
                    "xml" => {
                        if call.params.len() != 1 {
                            ctx.report_err(AnalyzerError::Xml, call.target.0);
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(mixin) = &node.mixin {
            if let Some(index) = self.merger.lookup(mixin) {
                if let Stat::Mixin(mixin) = &self.opcodes[index] {
                    let expand = mixin.fields.clone();
                    let fields = node.fields.clone();

                    let fields = match fields.append(expand) {
                        Ok(fields) => fields,
                        Err(fields) => {
                            ctx.report_err(
                                AnalyzerError::Merge(mixin.ident.1.clone(), mixin.ident.0),
                                node.ident.0,
                            );
                            fields
                        }
                    };

                    return Some(Node {
                        span: node.span,
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
                ctx.report_err(AnalyzerError::Unknown(mixin.1.clone()), mixin.0);
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
            if let Stat::Group(group) = &self.opcodes[index] {
                return Some(group.children.clone());
            } else {
                panic!("expand_with_group: inner error.");
            }
        } else {
            ctx.report_err(AnalyzerError::Unknown(ident.1.clone()), ident.0);
            None
        }
    }

    fn apply_to_check(&self, ctx: &mut ParseContext<'_>, node: &ApplyTo) -> Option<Stat> {
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

        Some(Stat::ApplyTo(Box::new(ApplyTo {
            from: from_expand,
            to: to_expand,
            span: node.span,
            comments: node.comments.clone(),
            properties: node.properties.clone(),
        })))
    }

    fn children_of_check(&self, ctx: &mut ParseContext<'_>, node: &ChildrenOf) -> Option<Stat> {
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

        Some(Stat::ChildrenOf(Box::new(ChildrenOf {
            from: from_expand,
            to: to_expand,
            span: node.span,
            comments: node.comments.clone(),
            properties: node.properties.clone(),
        })))
    }
}

/// Process semantic analyze on `opcodes` slice.
pub fn semantic_analyze(opcodes: &mut [Stat], ctx: &mut ParseContext<'_>) {
    SemanticAnalyzer::new(opcodes).analyze(ctx);
}
