use super::*;

/// Any value that can be put into [`Data`] must implement this trait.
pub trait DataType {
    /// Returns some reference to the inner value of [`Data`] if it is of type `Self`, or None if it isn’t.
    fn downcast_ref(data: &Data) -> Option<&Self>;

    /// Convert self into [`Data`]
    fn into_data(self) -> Data;
}

macro_rules! impl_data_type {
    ($name: ident) => {
        impl $crate::data::DataType for $name {
            fn downcast_ref(data: &super::Data) -> Option<&Self> {
                match data {
                    $crate::data::Data::$name(v) => Some(v),
                    _ => None,
                }
            }

            fn into_data(self) -> super::Data {
                $crate::data::Data::$name(self)
            }
        }
    };

    (box, $name: ident) => {
        impl $crate::data::DataType for $name {
            fn downcast_ref(data: &super::Data) -> Option<&Self> {
                match data {
                    $crate::data::Data::$name(v) => Some(v),
                    _ => None,
                }
            }

            fn into_data(self) -> super::Data {
                $crate::data::Data::$name(Box::new(self))
            }
        }
    };

    (listof, $name: ident) => {
        concat_idents::concat_idents!(field_name = ListOf, $name {
        impl $crate::data::DataType for Vec<$name> {
            fn downcast_ref(data: &super::Data) -> Option<&Vec<$name>> {
                match data {
                        $crate::data::Data::field_name(v) => return Some(v),
                        _ => None,
                }

            }

            fn into_data(self) -> super::Data {
                $crate::data::Data::field_name(Box::new(self))
            }
        }
    });
    };
}

pub(super) use impl_data_type;

/// the data types that can be passed to the animation registers.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Data {
    Integer(Integer),
    Number(Number),
    Angle(Angle),
    ListOfAngle(Box<Vec<Angle>>),
    Rgb(Rgb),
    Length(Length),
    ListOfLength(Box<Vec<Length>>),
    Iri(Box<Iri>),
    FuncIRI(Box<FuncIRI>),
    FontFamily(Box<FontFamily>),
    ListOfFontFamily(Box<Vec<FontFamily>>),
    Point(Point),
    ListOfPoint(Box<Vec<Point>>),
    Percentage(Percentage),
    Paint(Box<Paint>),
    NumberOptNumber(NumberOptNumber),
    StrokeLineCap(StrokeLineCap),
    StrokeLineJoin(StrokeLineJoin),
    FillRule(FillRule),
    TextLengthAdjust(TextLengthAdjust),
}
