//! This crate is the official svg [`Surface`] implementation.
//!
//!
//! [`Surface`]: vglang::surface::Surface

mod errors;
pub use errors::*;

mod renderer;
pub use renderer::*;

mod surface;
pub use surface::*;

mod element;
pub use element::*;

#[allow(unused)]
mod attr;
pub use attr::*;

mod variable;
pub use variable::*;
