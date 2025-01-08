//! `mlang` is a programming language to define declarative programming language.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod opcode;
mod parser;
pub use parser::*;
mod semantic;
pub use semantic::*;

#[cfg(feature = "codegen")]
#[cfg_attr(docsrs, doc(cfg(feature = "codegen")))]
pub mod codegen;
