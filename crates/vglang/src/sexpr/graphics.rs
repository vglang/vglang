use crate::surface::Builder;

/// s-expr combinator must implement this trait.
pub trait Graphics<B: Builder> {
    /// Generate `opcode`s for specific surface.
    fn build(self, builder: &mut B);
}
