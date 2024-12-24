//! `vglang` is a simple and fast **Vector Graphics programming Language**.

pub mod opcode;

#[cfg(feature = "sexpr")]
#[cfg_attr(docsrs, doc(cfg(feature = "sexpr")))]
pub mod sexpr;

#[cfg(feature = "surface")]
#[cfg_attr(docsrs, doc(cfg(feature = "surface")))]
pub mod surface;
