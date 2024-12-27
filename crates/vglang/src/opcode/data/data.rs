use super::*;

/// the data types that can be passed to the animation registers.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Data))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Data {
    Bool(bool),
    ListOfBool(Box<Vec<bool>>),
    String(String),
    ListOfString(Box<Vec<String>>),
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
    FontStyle(FontStyle),
    ListOfFontStyle(Box<Vec<FontStyle>>),
    FontVariant(FontVariant),
    ListOfFontVariant(Box<Vec<FontVariant>>),
    FontWeight(FontWeight),
    ListOfFontWeight(Box<Vec<FontWeight>>),
    FontStretch(FontStretch),
    ListOfFontStretch(Box<Vec<FontStretch>>),
    ListOfPathEvent(Box<Vec<PathEvent>>),
    PreserveAspectRatio(PreserveAspectRatio),
    ListOfPreserveAspectRatio(Box<Vec<PreserveAspectRatio>>),
    SpreadMethod(SpreadMethod),
    ListOfSpreadMethod(Box<Vec<SpreadMethod>>),
    Transform(Box<Transform>),
    ListOfTransform(Box<Vec<Transform>>),
    Coords(Coords),
    ListOfCoords(Box<Vec<Coords>>),
    FeIn(FeIn),
    ListOfFeIn(Box<Vec<FeIn>>),
    FeOut(FeOut),
    ListOfFeOut(Box<Vec<FeOut>>),
    FeBlendMode(FeBlendMode),
    ListOfFeBlendMode(Box<Vec<FeBlendMode>>),
    FeColorMatrixValues(Box<FeColorMatrixValues>),
    ListOfFeColorMatrixValues(Box<Vec<FeColorMatrixValues>>),
    FeFunc(Box<FeFunc>),
    ListOfFeFunc(Box<Vec<FeFunc>>),
    FeCompositeOperator(Box<FeCompositeOperator>),
    ListOfFeCompositeOperator(Box<Vec<FeCompositeOperator>>),
    FeConvolveMatrixEdgeMode(FeConvolveMatrixEdgeMode),
    ListOfFeConvolveMatrixEdgeMode(Box<Vec<FeConvolveMatrixEdgeMode>>),
    ChannelSelector(ChannelSelector),
    ListOfChannelSelector(Box<Vec<ChannelSelector>>),
    FeMorphologyOperator(FeMorphologyOperator),
    ListOfFeMorphologyOperator(Box<Vec<FeMorphologyOperator>>),
    FeStitchTiles(FeStitchTiles),
    ListOfFeStitchTiles(Box<Vec<FeStitchTiles>>),
    FeTurbulenceType(FeTurbulenceType),
    ListOfFeTurbulenceType(Box<Vec<FeTurbulenceType>>),
    ClipRule(ClipRule),
    ListOfClipRule(Box<Vec<ClipRule>>),
    WritingMode(WritingMode),
    ListOfWritingMode(Box<Vec<WritingMode>>),
    TextDirection(TextDirection),
    ListOfTextDirection(Box<Vec<TextDirection>>),
    UnicodeBidi(UnicodeBidi),
    ListOfUnicodeBidi(Box<Vec<UnicodeBidi>>),
    TextAnchor(TextAnchor),
    ListOfTextAnchor(Box<Vec<TextAnchor>>),
    DominantBaseline(DominantBaseline),
    ListOfDominantBaseline(Box<Vec<DominantBaseline>>),
    AlignmentBaseline(AlignmentBaseline),
    ListOfAlignmentBaseline(Box<Vec<AlignmentBaseline>>),
    BaselineShift(BaselineShift),
    ListOfBaselineShift(Box<Vec<BaselineShift>>),
    TextDecoration(TextDecoration),
    ListOfTextDecoration(Box<Vec<TextDecoration>>),
    TextPathMethod(TextPathMethod),
    ListOfTextPathMethod(Box<Vec<TextPathMethod>>),
    TextPathSpacing(TextPathSpacing),
    ListOfTextPathSpacing(Box<Vec<TextPathSpacing>>),
    LetterSpacing(LetterSpacing),
    ListOfLetterSpacing(Box<Vec<LetterSpacing>>),
    WordSpacing(WordSpacing),
    ListOfWordSpacing(Box<Vec<WordSpacing>>),
}
