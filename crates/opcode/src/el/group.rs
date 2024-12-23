use crate::attrs::{impl_apply_to, Fill, Stroke};

use super::{impl_content_of, Container, Text};

/// A container element for grouping together related graphics elements.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Group;

impl_apply_to!(box, Fill, Group);
impl_apply_to!(box, Stroke, Group);

impl_content_of!(box, Text, Group, Container);
impl_content_of!(Group, Group, Container);
