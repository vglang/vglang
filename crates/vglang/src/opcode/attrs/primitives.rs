/// Define a fragment by name.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Id(pub String);

impl<T> From<T> for Id
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
