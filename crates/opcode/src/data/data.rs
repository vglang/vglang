use super::*;

/// the data types that can be passed to the animation registers.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
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

#[cfg(test)]
mod tests {
    use crate::data::{Angle, MapCollect};

    use super::Data;

    #[test]
    fn test_from_data() {
        let angle = Data::Angle(10.into());

        assert_eq!(TryFrom::try_from(&angle), Ok(&Angle::deg(10.0)));

        let angles = Data::ListOfAngle(Box::new((10, 11).map_collect()));

        assert_eq!(
            TryFrom::try_from(&angles),
            Ok(&MapCollect::<Angle>::map_collect((10, 11)))
        );
    }
}
