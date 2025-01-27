//! A code generation tool to help developing `vglang`.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "compiler")]
#[cfg_attr(docsrs, doc(cfg(feature = "compiler")))]
pub mod lang;

#[cfg(feature = "rt")]
#[cfg_attr(docsrs, doc(cfg(feature = "rt")))]
pub mod rt;
