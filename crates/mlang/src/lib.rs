//! `mlang` is a programming language to define declarative programming language.

pub mod opcode;
mod parser;
pub use parser::*;
mod semantic;
pub use semantic::*;
