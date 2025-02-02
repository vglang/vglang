//! Compile and code generation tools for mlang.

pub mod analyzer;
pub mod ir;
pub mod parser;
pub mod rustgen;

mod ext {

    use parserc::{ParseContext, PrintReport, Result};
    use std::path::Path;

    use super::{analyzer::semantic_analyze, parser::parse, rustgen::codegen};

    /// Compile `mlang` source code and generate rust source code.
    ///
    /// This function will output any errors encountered during compilation directly to the terminal
    pub fn compile<S: AsRef<str>, T: AsRef<Path>>(
        source: S,
        target: T,
        without_ext: bool,
    ) -> Result<()> {
        let mut ctx = ParseContext::from(source.as_ref());

        let mut stats = match parse(&mut ctx) {
            Ok(stats) => stats,
            Err(err) => {
                ctx.report().print_reports();
                return Err(err);
            }
        };

        semantic_analyze(&mut stats, &mut ctx);

        if ctx.report_size() > 0 {
            ctx.report().print_reports();
            return Err(parserc::ControlFlow::Fatal);
        }

        match codegen(stats, target, without_ext) {
            Err(err) => {
                eprintln!("codegen: {}", err);
                return Err(parserc::ControlFlow::Fatal);
            }
            _ => {}
        }

        Ok(())
    }
}

pub use ext::*;
