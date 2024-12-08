//! cotati is a general-purpose vector graphic language system.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

/// Intermediate Representation (IR) of VGL(cotati vector graphics language)
#[cfg(feature = "ir")]
#[cfg_attr(docsrs, doc(cfg(feature = "ir")))]
pub mod ir {
    pub use cotati_ir::*;
}
