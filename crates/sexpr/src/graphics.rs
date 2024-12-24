use vglang_opcode::el::{Element, Group, Text, TextSpan};
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

macro_rules! impl_graphics {
    ($($name: ident),+) => {
        $(impl<B> Graphics<B> for $name
        where
            B: Builder,{
            fn build(self, builder: &mut B) {
                builder.push(self.into_opcode())
            }
        })+
    };
}

impl_graphics!(Text, TextSpan, Group);
