use std::collections::HashMap;

use crate::errors::{Error, Result};

/// This is a marker trait that a type with this trait can be used as frame register variable.
pub trait FrameVariable {}

impl FrameVariable for bool {}
impl FrameVariable for f32 {}
impl FrameVariable for u32 {}
impl FrameVariable for i32 {}
impl<T> FrameVariable for Vec<T> where T: FrameVariable {}

/// An variable container, indicates that this variable can be used as animation frame variable.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Animatable<T>
where
    T: FrameVariable,
{
    /// a reference variable to `frame register`.
    Animated(String),
    /// non-animated variable
    Constant(T),
}

impl<T> FrameVariable for Animatable<T> where T: FrameVariable {}

impl<T> Animatable<T>
where
    T: FrameVariable,
{
    /// Convert self into [`Result<T>`].
    ///
    /// * returns [`Error::UnsatisfiedFrameVariable`] if this variant is a [`frame register`](Animatable::Frame) variable.
    /// * returns [`Ok(T)`](Ok) if this variant is a [`constant`](Animatable::Constant) variable
    pub fn ok(self) -> Result<T> {
        match self {
            Animatable::Animated(n) => Err(Error::UnsatisfiedFrameVariable(n)),
            Animatable::Constant(v) => Ok(v),
        }
    }

    /// Get animatable variable from constant storage or registers.
    pub fn get(
        &self,
        animatable: &HashMap<String, AnimatableValue>,
    ) -> std::result::Result<&T, &str> {
        match self {
            Animatable::Animated(name) => {
                animatable.get(name);

                todo!("cast AnimatableValue to `T`")
            }
            Animatable::Constant(v) => Ok(v),
        }
    }
}

impl<T> From<T> for Animatable<T>
where
    T: FrameVariable,
{
    fn from(value: T) -> Self {
        Self::Constant(value)
    }
}

impl<T> From<&str> for Animatable<T>
where
    T: FrameVariable,
{
    fn from(value: &str) -> Self {
        Self::Animated(value.to_string())
    }
}

impl<T> From<String> for Animatable<T>
where
    T: FrameVariable,
{
    fn from(value: String) -> Self {
        Self::Animated(value)
    }
}

impl<T> Default for Animatable<T>
where
    T: Default + FrameVariable,
{
    fn default() -> Self {
        Self::Constant(T::default())
    }
}

/// An variant that referenced by one animatable register.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnimatableValue {}
