use super::{Angle, FillRule, Length, Paint, Rgb, StrokeLineCap, StrokeLineJoin, TextLengthAdjust};

/// Values passed by register.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Characters(String),
    Float32(f32),
    TextLengthAdjust(TextLengthAdjust),
    Length(Length),
    Angle(Angle),
    ListOf(Box<Vec<Value>>),
    Rgb(Rgb),
    Paint(Box<Paint>),
    FillRule(FillRule),
    StrokeLineCap(StrokeLineCap),
    StrokeLineJoin(StrokeLineJoin),
}

impl<T> From<Vec<T>> for Value
where
    Value: From<T>,
{
    fn from(value: Vec<T>) -> Self {
        Self::ListOf(Box::new(value.into_iter().map(|v| v.into()).collect()))
    }
}

impl<T> TryFrom<Value> for Vec<T>
where
    T: TryFrom<Value, Error = Value>,
{
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::ListOf(v) => v.into_iter().map(|v| v.try_into()).collect(),
            _ => Err(value),
        }
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float32(value)
    }
}

impl TryFrom<Value> for f32 {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float32(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<TextLengthAdjust> for Value {
    fn from(value: TextLengthAdjust) -> Self {
        Self::TextLengthAdjust(value)
    }
}

impl TryFrom<Value> for TextLengthAdjust {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::TextLengthAdjust(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<Length> for Value {
    fn from(value: Length) -> Self {
        Self::Length(value)
    }
}

impl TryFrom<Value> for Length {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Length(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<Angle> for Value {
    fn from(value: Angle) -> Self {
        Self::Angle(value)
    }
}

impl TryFrom<Value> for Angle {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Angle(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<Rgb> for Value {
    fn from(value: Rgb) -> Self {
        Self::Rgb(value)
    }
}

impl TryFrom<Value> for Rgb {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Rgb(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<Paint> for Value {
    fn from(value: Paint) -> Self {
        Self::Paint(Box::new(value))
    }
}

impl TryFrom<Value> for Paint {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Paint(v) => Ok(*v),
            _ => Err(value),
        }
    }
}

impl From<FillRule> for Value {
    fn from(value: FillRule) -> Self {
        Self::FillRule(value)
    }
}

impl TryFrom<Value> for FillRule {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::FillRule(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<StrokeLineCap> for Value {
    fn from(value: StrokeLineCap) -> Self {
        Self::StrokeLineCap(value)
    }
}

impl TryFrom<Value> for StrokeLineCap {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::StrokeLineCap(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<StrokeLineJoin> for Value {
    fn from(value: StrokeLineJoin) -> Self {
        Self::StrokeLineJoin(value)
    }
}

impl TryFrom<Value> for StrokeLineJoin {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::StrokeLineJoin(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Characters(value)
    }
}

impl TryFrom<Value> for String {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Characters(v) => Ok(v),
            _ => Err(value),
        }
    }
}
