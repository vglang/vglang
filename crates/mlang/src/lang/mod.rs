//! Compile and code generation tools for mlang.

pub mod analyzer;
pub mod ir;
pub mod parser;
pub mod rustgen;

mod ext {

    use parserc::{ParseContext, PrintReport, Result};
    use std::path::Path;

    use super::{analyzer::semantic_analyze, parser::parse};

    /// Compile `mlang` source code and generate rust source code.
    ///
    /// This function will output any errors encountered during compilation directly to the terminal
    pub fn compile<S: AsRef<str>, T: AsRef<Path>>(source: S, target: T) -> Result<()> {
        let mut ctx = ParseContext::from(source.as_ref());

        let mut opcodes = match parse(&mut ctx) {
            Ok(opcodes) => opcodes,
            Err(err) => {
                ctx.report().print_reports();
                return Err(err);
            }
        };

        semantic_analyze(&mut opcodes, &mut ctx);

        if ctx.report_size() > 0 {
            ctx.report().print_reports();
            return Err(parserc::ControlFlow::Fatal);
        }

        let _ = target;

        Ok(())
    }
}

pub use ext::*;
