//! Rust dsl for cotati vector graphics language(VGL)

pub mod dsl;
pub mod generator;

/// The attributes used by graphic elements.
pub mod attrs {
    pub use vglang_ir::*;
}
