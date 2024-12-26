use std::ops::Range;

use crate::opcode::variable::{Path, Variable};

#[cfg(feature = "sexpr")]
use super::*;

/// Contruct `foreach` expression.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::container_element(boxed, Group, Text, TextSpan)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foreach(pub Path);

impl<T> From<T> for Foreach
where
    Path: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// Contruct `foreach` expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::container_element(boxed, Group, Text, TextSpan)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct For(pub Range<Variable<i32>>);

impl<T> From<T> for For
where
    Range<i32>: From<T>,
{
    fn from(value: T) -> Self {
        let value: Range<i32> = value.into();

        Self(Range {
            start: Variable::Constant(value.start),
            end: Variable::Constant(value.end),
        })
    }
}

/// Contruct `if` expression.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct If(pub Variable<bool>);

impl<T> From<T> for If
where
    bool: From<T>,
{
    fn from(value: T) -> Self {
        Self(Variable::Constant(value.into()))
    }
}

/// Contruct `else` expression.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Else;
