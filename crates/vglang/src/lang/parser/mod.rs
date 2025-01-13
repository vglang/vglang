//! The parser frontend of vglang script.

mod error;
pub use error::*;

mod parser;
pub use parser::*;

mod color;
pub use color::*;

mod variable;
pub use variable::*;
