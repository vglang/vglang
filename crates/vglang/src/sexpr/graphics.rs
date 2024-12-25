use super::BuildContext;

/// s-expr combinator must implement this trait.
pub trait Graphics {
    /// Generate `opcode`s for specific surface.
    fn build(self, builder: &mut BuildContext);
}
