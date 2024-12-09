//! Rust dsl for cotati vector graphics language(VGL)

pub mod drawing;
pub mod generator;

/// The attributes used by graphic elements.
pub mod attrs {
    pub use cotati_ir::*;
}
