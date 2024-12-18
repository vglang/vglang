//! This crate defines the binary format of vglang and the set of optimised low-level instructions
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

mod opcode;
pub use opcode::*;

pub mod operand;

#[cfg_attr(docsrs, doc(cfg(feature = "sexpr")))]
#[cfg(feature = "sexpr")]
mod sexpr;

#[cfg(feature = "sexpr")]
pub use sexpr::*;
