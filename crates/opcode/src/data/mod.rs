//! This module defines the data types that can be passed to the animation registers.

mod data;
pub use data::*;

mod basic_types;
pub use basic_types::*;

mod painting;
pub use painting::*;

mod text;
pub use text::*;

#[cfg(feature = "sexpr")]
#[cfg_attr(docsrs, doc(cfg(feature = "sexpr")))]
mod map_collect;
#[cfg(feature = "sexpr")]
pub use map_collect::*;
