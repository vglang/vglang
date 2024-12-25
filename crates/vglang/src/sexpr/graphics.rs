use crate::opcode::el::Container;

use super::{BuildContext, ContentOf};

/// s-expr combinator must implement this trait.
pub trait Graphics {
    /// Generate `opcode`s for specific surface.
    fn build(self, builder: &mut BuildContext);
}

pub trait Child<Target>: Graphics + ContentOf<Target>
where
    Container: From<Target>,
{
}

impl<T, Target> Child<Target> for T
where
    T: ContentOf<Target> + Graphics,

    Container: From<Target>,
{
}
