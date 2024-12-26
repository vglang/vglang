//! Defines inheritable attributes that can be applied to graphic elements.

mod painting;
pub use painting::*;

mod primitives;
pub use primitives::*;

mod attrs;
pub use attrs::*;

mod font;
pub use font::*;

mod canvas;
pub use canvas::*;

mod compositing;
pub use compositing::*;
