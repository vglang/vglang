use super::*;

/// the data types that can be passed to the animation registers.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Data))]
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
