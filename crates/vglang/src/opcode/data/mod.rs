//! This module defines the data types that can be passed to the animation registers.

mod data;
pub use data::*;

mod primitives;
pub use primitives::*;

mod painting;
pub use painting::*;

mod text;
pub use text::*;

mod font;
pub use font::*;

mod path;
pub use path::*;

mod canvas;
pub use canvas::*;

mod gradients;
pub use gradients::*;

mod filter;
pub use filter::*;
