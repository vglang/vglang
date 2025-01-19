//! **vglang** is a simple and fast *`Vector Graphics programming Language`*

#![cfg_attr(docsrs, feature(doc_cfg))]

mod codegen;
pub use codegen::opcode;

mod sexpr_manual;

#[cfg(feature = "sexpr")]
#[cfg_attr(docsrs, doc(cfg(feature = "sexpr")))]
pub mod sexpr {
    pub use super::codegen::sexpr::*;
    pub use super::sexpr_manual::*;
}

#[cfg(feature = "surface")]
#[cfg_attr(docsrs, doc(cfg(feature = "surface")))]
pub mod surface;
