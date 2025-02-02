//! Compile and code generation tools for mlang.

pub mod analyzer;
pub mod ir;
pub mod parser;
pub mod rustgen;
pub use parserc;

/// Utilities module
pub mod ext {

    use parserc::{ParseContext, Result};
    use std::path::Path;

    use super::{analyzer::semantic_analyze, parser::parse};

    /// Compile `mlang` source code and generate rust source code.
    pub fn compile<S: AsRef<str>, T: AsRef<Path>>(source: S, target: T) -> Result<()> {
        let mut ctx = ParseContext::from(source.as_ref());

        let mut opcodes = parse(&mut ctx)?;

        semantic_analyze(&mut opcodes, &mut ctx);

        if ctx.report_size() > 0 {
            return Err(parserc::ControlFlow::Fatal);
        }

        let _ = target;

        Ok(())
    }
}
