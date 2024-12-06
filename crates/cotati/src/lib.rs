//! cotati is a general-purpose vector graphic language system.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "ir")]
#[cfg_attr(docsrs, doc(cfg(feature = "ir")))]
pub use cotati_ir as ir;
