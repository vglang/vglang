use super::{
    AlignmentBaseline, Angle, BaselineShift, ClipRule, Coords, DominantBaseline, FillRule,
    FontFamily, FontStretch, FontStyle, FontVariant, FontWeight, FuncIRI, Length, Number, Paint,
    Point, PreserveAspectRatio, Rgb, SpreadMethod, StrokeLineCap, StrokeLineJoin, TextAnchor,
    TextLengthAdjust, TextPathMethod, TextPathSpacing, Transform,
};

/// Values passed by register.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Characters(String),
    Number(Number),
    TextLengthAdjust(TextLengthAdjust),
    Length(Length),
    LengthList(Box<Vec<Length>>),
    Angle(Angle),
    AngleList(Vec<Angle>),
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
    ClipRule(ClipRule),
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
        Self::AngleList(value)
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
