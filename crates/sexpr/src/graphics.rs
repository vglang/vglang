use vglang_opcode::el::Element;
use vglang_surface::Builder;

/// The returns types of all sexprs must implement this trait.
pub trait Graphics<B>
where
    B: Builder,
{
    /// Generate real graphic drawing [`Opcode`].
    fn build(self, builder: &mut B);
}

impl<B, E> Graphics<B> for E
where
    E: Element,
    B: Builder,
{
    fn build(self, builder: &mut B) {
        builder.push(self.into_opcode());
    }
}
