use super::{
    AlignmentBaseline, Angle, BaselineShift, ClipRule, Coords, DominantBaseline, FeBlendMode,
    FeColorMatrixValues, FeCompositeOperator, FeConvolveMatrixEdgeMode, FeFunc, FeIn,
    FeMorphologyOperator, FeOut, FePrimitiveRef, FeStitchTiles, FeTurbulenceType, FillRule,
    FilterRes, FontFamily, FontStretch, FontStyle, FontVariant, FontWeight, Length, Paint, Point,
    PreserveAspectRatio, Rgb, SpreadMethod, StrokeLineCap, StrokeLineJoin, TextAnchor,
    TextLengthAdjust, TextPathMethod, TextPathSpacing, Transform,
};

use std::{fmt::Display, ops::Deref};

/// Within the SVG DOM, a `number` is represented as a float,
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Number(pub f32);

impl Deref for Number {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self(value as f32)
    }
}

impl From<Number> for f32 {
    fn from(value: Number) -> Self {
        value.0
    }
}

/// Within the SVG DOM, an <integer> is represented as a i32, the range for an <integer> encompasses (at a minimum) -2147483648 to 2147483647.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Integer(pub i32);

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Integer {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

/// Functional notation for an IRI: "url(" <IRI> ")".
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FuncIRI(pub String);

impl Deref for FuncIRI {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for FuncIRI {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for FuncIRI {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl Display for FuncIRI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "url(#{})", self.0)
    }
}

/// A pair of `number`s, where the second `number` is optional.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberOptNumber(pub Number, pub Option<Number>);

impl<V> From<V> for NumberOptNumber
where
    Number: From<V>,
{
    fn from(value: V) -> Self {
        Self(value.into(), None)
    }
}

impl<X, Y> From<(X, Y)> for NumberOptNumber
where
    Number: From<X> + From<Y>,
{
    fn from(value: (X, Y)) -> Self {
        Self(value.0.into(), Some(value.1.into()))
    }
}

/// Within the SVG DOM, a `bool` is represented as a bool,
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bool(pub bool);

impl Deref for Bool {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

/// Indicates which channel of rgba is selected.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChannelSelector {
    R,
    G,
    B,
    A,
}

/// Values passed by register.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Characters(String),
    Number(Number),
    TextLengthAdjust(TextLengthAdjust),
    Length(Length),
    LengthList(Box<Vec<Length>>),
    NumberList(Box<Vec<Number>>),
    Angle(Angle),
    AngleList(Box<Vec<Angle>>),
    ListOf(Box<Vec<Value>>),
    Rgb(Rgb),
    Paint(Box<Paint>),
    FillRule(FillRule),
    StrokeLineCap(StrokeLineCap),
    StrokeLineJoin(StrokeLineJoin),
    PreserveAspectRatio(PreserveAspectRatio),
    TextAnchor(TextAnchor),
    DominantBaseline(DominantBaseline),
    AlignmentBaseline(AlignmentBaseline),
    BaselineShift(BaselineShift),
    FontFamily(FontFamily),
    FontStyle(FontStyle),
    FontWeight(FontWeight),
    FontStretch(FontStretch),
    FontVariant(FontVariant),
    Transform(Transform),
    SpreadMethod(SpreadMethod),
    Coords(Coords),
    PointList(Box<Vec<Point>>),
    TextPathMethod(TextPathMethod),
    TextPathSpacing(TextPathSpacing),
    FuncIRI(Box<FuncIRI>),
    FilterRes(FilterRes),
    FeIn(FeIn),
    FeOut(FeOut),
    ClipRule(ClipRule),
    FeBlendMode(FeBlendMode),
    FePrimitiveRef(FePrimitiveRef),
    FeColorMatrixValues(FeColorMatrixValues),
    FeFunc(FeFunc),
    FeCompositeOperator(Box<FeCompositeOperator>),

    FeConvolveMatrixEdgeMode(FeConvolveMatrixEdgeMode),
    Integer(Integer),
    NumberOptNumber(NumberOptNumber),
    Bool(Bool),
    ChannelSelector(ChannelSelector),
    FeStitchTiles(FeStitchTiles),
    FeTurbulenceType(FeTurbulenceType),
    FeMorphologyOperator(FeMorphologyOperator),
}

macro_rules! value_type {
    ($v: tt) => {
        impl From<$v> for Value {
            fn from(value: $v) -> Self {
                Self::$v(value)
            }
        }

        impl<'a> TryFrom<&'a Value> for &'a $v {
            type Error = &'a Value;
            fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
                match value {
                    Value::$v(v) => Ok(v),
                    _ => Err(value),
                }
            }
        }
    };
    (box, $v: tt) => {
        impl From<$v> for Value {
            fn from(value: $v) -> Self {
                Self::$v(Box::new(value))
            }
        }

        impl<'a> TryFrom<&'a Value> for &'a $v {
            type Error = &'a Value;
            fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
                match value {
                    Value::$v(v) => Ok(v),
                    _ => Err(value),
                }
            }
        }
    };
}

value_type!(FeMorphologyOperator);
value_type!(FeTurbulenceType);
value_type!(FeStitchTiles);
value_type!(ChannelSelector);
value_type!(Bool);
value_type!(NumberOptNumber);
value_type!(Integer);
value_type!(FeConvolveMatrixEdgeMode);
value_type!(box, FeCompositeOperator);
value_type!(FeFunc);
value_type!(FeColorMatrixValues);
value_type!(FePrimitiveRef);
value_type!(FeBlendMode);
value_type!(FeIn);
value_type!(FeOut);
value_type!(FilterRes);
value_type!(box, FuncIRI);
value_type!(ClipRule);
value_type!(SpreadMethod);
value_type!(Coords);
value_type!(TextPathSpacing);
value_type!(TextPathMethod);
value_type!(Transform);
value_type!(FontVariant);
value_type!(FontFamily);
value_type!(FontStyle);
value_type!(FontWeight);
value_type!(FontStretch);
value_type!(TextAnchor);
value_type!(DominantBaseline);
value_type!(AlignmentBaseline);
value_type!(BaselineShift);
value_type!(TextLengthAdjust);
value_type!(Length);
value_type!(Angle);
value_type!(Rgb);
value_type!(box, Paint);
value_type!(FillRule);
value_type!(StrokeLineCap);
value_type!(StrokeLineJoin);
value_type!(PreserveAspectRatio);
value_type!(Number);

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Characters(value)
    }
}

impl<'a> TryFrom<&'a Value> for &'a String {
    type Error = &'a Value;
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Characters(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<Vec<Length>> for Value {
    fn from(value: Vec<Length>) -> Self {
        Self::LengthList(Box::new(value))
    }
}

impl<'a> TryFrom<&'a Value> for &'a Vec<Length> {
    type Error = &'a Value;
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::LengthList(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<Vec<Point>> for Value {
    fn from(value: Vec<Point>) -> Self {
        Self::PointList(Box::new(value))
    }
}

impl<'a> TryFrom<&'a Value> for &'a Vec<Point> {
    type Error = &'a Value;
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::PointList(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<Vec<Angle>> for Value {
    fn from(value: Vec<Angle>) -> Self {
        Self::AngleList(Box::new(value))
    }
}

impl<'a> TryFrom<&'a Value> for &'a Vec<Angle> {
    type Error = &'a Value;
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::AngleList(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<Vec<Number>> for Value {
    fn from(value: Vec<Number>) -> Self {
        Self::NumberList(Box::new(value))
    }
}

impl<'a> TryFrom<&'a Value> for &'a Vec<Number> {
    type Error = &'a Value;
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::NumberList(v) => Ok(v),
            _ => Err(value),
        }
    }
}
