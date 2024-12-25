use super::*;

/// the data types that can be passed to the animation registers.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Data))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Data {
    Bool(bool),
    ListOfBool(Box<Vec<bool>>),
    Integer(i32),
    ListOfInteger(Box<Vec<i32>>),
    Number(Number),
    ListOfNumber(Box<Vec<Number>>),
    Angle(Angle),
    ListOfAngle(Box<Vec<Angle>>),
    Rgb(Rgb),
    ListOfRgb(Box<Vec<Rgb>>),
    Length(Length),
    ListOfLength(Box<Vec<Length>>),
    Iri(Box<Iri>),
    ListOfIri(Box<Vec<Iri>>),
    FuncIRI(Box<FuncIRI>),
    ListOfFuncIRI(Box<Vec<FuncIRI>>),
    FontFamily(Box<FontFamily>),
    ListOfFontFamily(Box<Vec<FontFamily>>),
    Point(Point),
    ListOfPoint(Box<Vec<Point>>),
    Percentage(Percentage),
    ListOfPercentage(Box<Vec<Percentage>>),
    Paint(Box<Paint>),
    ListOfPaint(Box<Vec<Paint>>),
    NumberOptNumber(NumberOptNumber),
    ListOfNumberOptNumber(Box<Vec<NumberOptNumber>>),
    StrokeLineCap(StrokeLineCap),
    ListOfStrokeLineCap(Box<Vec<StrokeLineCap>>),
    StrokeLineJoin(StrokeLineJoin),
    ListOfStrokeLineJoin(Box<Vec<StrokeLineJoin>>),
    FillRule(FillRule),
    ListOfFillRule(Box<Vec<FillRule>>),
    TextLengthAdjust(TextLengthAdjust),
    ListOfTextLengthAdjust(Box<Vec<TextLengthAdjust>>),
}
