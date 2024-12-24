use crate::opcode::el::Container;

/// An element that can used as container element's child must implement this trait.
pub trait ContentOf<E>
where
    E: Into<Container>,
{
}
