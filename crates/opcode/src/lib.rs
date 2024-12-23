//! This crate defines the binary format of vglang and the set of optimised low-level instructions
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod attrs;
pub mod data;
pub mod el;
pub mod variable;

mod opcode;
pub use opcode::*;
