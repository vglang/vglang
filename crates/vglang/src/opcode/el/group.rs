#[cfg(feature = "sexpr")]
use super::*;

/// A container element for grouping together related graphics elements.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::container_element(Group, If, Foreach, For)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Group;
