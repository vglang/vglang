/// Apply a scoped attribute.
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sattribute {
    String(String),
}
