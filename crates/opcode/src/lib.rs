//! This crate defines the binary format of vglang and the set of optimised low-level instructions

mod opcode;
pub use opcode::*;

mod operand;
pub use operand::*;
