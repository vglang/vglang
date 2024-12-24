use vglang_surface::Builder;

/// The returns types of all sexprs must implement this trait.
pub trait Graphics<B>
where
    B: Builder,
{
    /// Generate real graphic drawing [`Opcode`].
    fn build(self, builder: &mut B);
}

impl<F, B> Graphics<B> for F
where
    B: Builder,
    F: FnOnce(&mut B),
{
    fn build(self, builder: &mut B) {
        self(builder)
    }
}
