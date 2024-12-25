use super::Element;

/// An attribute that can used as appliable attribute for one element must implement this trait.
pub trait ApplyTo<E>
where
    E: Element,
{
}
