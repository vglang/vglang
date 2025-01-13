//! **vglang** is a simple and fast *`Vector Graphics programming Language`*

#![cfg_attr(docsrs, feature(doc_cfg))]

mod ml;
pub use ml::opcode;

mod manual;

pub use vglang_derive::rgb;

#[cfg(feature = "lang")]
#[cfg_attr(docsrs, doc(cfg(feature = "lang")))]
pub mod lang;
