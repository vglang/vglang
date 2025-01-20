#[doc = " [Ln 1, Col 1]"]
#[doc = " [Ln 2, Col 1]"]
#[doc = " [Ln 3, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Angle {
    Deg(f32),
    Grad(f32),
    Rad(f32),
}
#[doc = " [Ln 6, Col 1]"]
#[doc = " [Ln 7, Col 1]"]
#[doc = " [Ln 8, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Length {
    #[doc = " [Ln 11, Col 5]"]
    #[doc = " [Ln 12, Col 5]"]
    Em(f32),
    #[doc = " [Ln 14, Col 5]"]
    #[doc = " [Ln 15, Col 5]"]
    Ex(f32),
    #[doc = " [Ln 17, Col 5]"]
    Px(f32),
    #[doc = " [Ln 19, Col 5]"]
    Inch(f32),
    #[doc = " [Ln 21, Col 5]"]
    Cm(f32),
    #[doc = " [Ln 23, Col 5]"]
    Mm(f32),
    #[doc = " [Ln 25, Col 5]"]
    Pt(f32),
    #[doc = " [Ln 27, Col 5]"]
    Pc(f32),
    #[doc = " [Ln 29, Col 5]"]
    Percent(f32),
}
#[doc = " [Ln 33, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Color {
    Aliceblue,
    Antiquewhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    Blanchedalmond,
    Blue,
    Blueviolet,
    Brown,
    Burlywood,
    Cadetblue,
    Chartreuse,
    Chocolate,
    Coral,
    Cornflowerblue,
    Cornsilk,
    Crimson,
    Cyan,
    Darkblue,
    Darkcyan,
    Darkgoldenrod,
    Darkgray,
    Darkgreen,
    Darkgrey,
    Darkkhaki,
    Darkmagenta,
    Darkolivegreen,
    Darkorange,
    Darkorchid,
    Darkred,
    Darksalmon,
    Darkseagreen,
    Darkslateblue,
    Darkslategray,
    Darkslategrey,
    Darkturquoise,
    Darkviolet,
    Deeppink,
    Deepskyblue,
    Dimgray,
    Dimgrey,
    Dodgerblue,
    Firebrick,
    Floralwhite,
    Forestgreen,
    Fuchsia,
    Gainsboro,
    Ghostwhite,
    Gold,
    Goldenrod,
    Gray,
    Grey,
    Green,
    Greenyellow,
    Honeydew,
    Hotpink,
    Indianred,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    Lavenderblush,
    Lawngreen,
    Lemonchiffon,
    Lightblue,
    Lightcoral,
    Lightcyan,
    Lightgoldenrodyellow,
    Lightgray,
    Lightgreen,
    Lightgrey,
    Lightpink,
    Lightsalmon,
    Lightseagreen,
    Lightskyblue,
    Lightslategray,
    Lightslategrey,
    Lightsteelblue,
    Lightyellow,
    Lime,
    Limegreen,
    Linen,
    Magenta,
    Maroon,
    Mediumaquamarine,
    Mediumblue,
    Mediumorchid,
    Mediumpurple,
    Mediumseagreen,
    Mediumslateblue,
    Mediumspringgreen,
    Mediumturquoise,
    Mediumvioletred,
    Midnightblue,
    Mintcream,
    Mistyrose,
    Moccasin,
    Navajowhite,
    Navy,
    Oldlace,
    Olive,
    Olivedrab,
    Orange,
    Orangered,
    Orchid,
    Palegoldenrod,
    Palegreen,
    Paleturquoise,
    Palevioletred,
    Papayawhip,
    Peachpuff,
    Peru,
    Pink,
    Plum,
    Powderblue,
    Purple,
    Red,
    Rosybrown,
    Royalblue,
    Saddlebrown,
    Salmon,
    Sandybrown,
    Seagreen,
    Seashell,
    Sienna,
    Silver,
    Skyblue,
    Slateblue,
    Slategray,
    Slategrey,
    Snow,
    Springgreen,
    Steelblue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    Whitesmoke,
    Yellow,
    Yellowgreen,
}
#[doc = " [Ln 184, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgb(pub u8, pub u8, pub u8);
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Iri {
    Local(String),
    Path(String),
}
#[doc = " [Ln 194, Col 1]"]
#[doc = " [Ln 195, Col 1]"]
#[doc = " [Ln 196, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FuncIri(pub String);
#[doc = " [Ln 199, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point(pub f32, pub f32);
#[doc = " [Ln 202, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Percent(pub f32);
#[doc = " [Ln 205, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Paint {
    #[doc = " [Ln 207, Col 5]"]
    Color(Rgb),
    #[doc = " [Ln 209, Col 5]"]
    Server(FuncIri),
}
#[doc = " [Ln 213, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberOptNumber(pub f32, pub Option<f32>);
#[doc = " [Ln 216, Col 1]"]
#[doc = " [Ln 217, Col 1]"]
#[doc = " [Ln 218, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Coords {
    #[doc = " [Ln 220, Col 5]"]
    #[doc = " [Ln 221, Col 5]"]
    #[doc = " [Ln 222, Col 5]"]
    #[doc = " [Ln 223, Col 5]"]
    UserSpaceOnUse,
    #[doc = " [Ln 226, Col 5]"]
    #[doc = " [Ln 227, Col 5]"]
    #[doc = " [Ln 228, Col 5]"]
    #[doc = " [Ln 229, Col 5]"]
    #[doc = " [Ln 230, Col 5]"]
    #[doc = " [Ln 231, Col 5]"]
    #[doc = " [Ln 232, Col 5]"]
    #[doc = " [Ln 233, Col 5]"]
    #[doc = " [Ln 234, Col 5]"]
    #[doc = " [Ln 235, Col 5]"]
    #[doc = " [Ln 236, Col 5]"]
    ObjectBoundingBox,
}
#[doc = " [Ln 240, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Transform {
    Translate(f32, f32),
    Matrix([f32; 6usize]),
    Scale(f32, Option<f32>),
    Rotate { angle: f32, cx: f32, cy: f32 },
    SkewX(f32),
    SkewY(f32),
}
#[doc = " [Ln 250, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Channel {
    R,
    G,
    B,
    A,
}
#[doc = " [Ln 253, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClipRule {
    Nonzero,
    EvenOdd,
}
#[doc = " [Ln 256, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathEvent {
    #[doc = " [Ln 259, Col 5]"]
    Close,
    #[doc = " [Ln 261, Col 5]"]
    MoveTo(Point),
    #[doc = " [Ln 263, Col 5]"]
    MoveToRelative(Point),
    #[doc = " [Ln 265, Col 5]"]
    LineTo(Point),
    #[doc = " [Ln 267, Col 5]"]
    LineToRelative(Point),
    #[doc = " [Ln 269, Col 5]"]
    Polyline(Vec<Point>),
    #[doc = " [Ln 271, Col 5]"]
    PolylineRelative(Vec<Point>),
    #[doc = " [Ln 273, Col 5]"]
    #[doc = " [Ln 274, Col 5]"]
    CubicBezier {
        ctrl1: Point,
        ctrl2: Point,
        to_point: Point,
    },
    #[doc = " [Ln 276, Col 5]"]
    #[doc = " [Ln 277, Col 5]"]
    CubicBezierRelative {
        ctrl1: Point,
        ctrl2: Point,
        to_point: Point,
    },
    #[doc = " [Ln 279, Col 5]"]
    #[doc = " [Ln 280, Col 5]"]
    CubicBezierSmooth { ctrl2: Point, to_point: Point },
    #[doc = " [Ln 282, Col 5]"]
    #[doc = " [Ln 283, Col 5]"]
    CubicBezierSmoothRelative { ctrl2: Point, to_point: Point },
    #[doc = " [Ln 285, Col 5]"]
    QuadraticBezier { ctrl: Point, to_point: Point },
    #[doc = " [Ln 287, Col 5]"]
    QuadraticBezierRelative { ctrl: Point, to_point: Point },
    #[doc = " [Ln 289, Col 5]"]
    QuadraticBezierSmooth(Point),
    #[doc = " [Ln 291, Col 5]"]
    QuadraticBezierSmoothRelative(Point),
    #[doc = " [Ln 293, Col 5]"]
    #[doc = " [Ln 294, Col 5]"]
    #[doc = " [Ln 295, Col 5]"]
    #[doc = " [Ln 296, Col 5]"]
    Arc {
        rx: f32,
        ry: f32,
        x_rotation: f32,
        large_arc: bool,
        sweep: bool,
        to_point: Point,
    },
    #[doc = " [Ln 301, Col 5]"]
    #[doc = " [Ln 302, Col 5]"]
    #[doc = " [Ln 303, Col 5]"]
    #[doc = " [Ln 304, Col 5]"]
    ArcRelative {
        rx: f32,
        ry: f32,
        x_rotation: f32,
        large_arc: bool,
        sweep: bool,
        to_point: Point,
    },
}
#[doc = " [Ln 310, Col 1]"]
#[doc = " [Ln 311, Col 1]"]
#[doc = " [Ln 312, Col 1]"]
#[doc = " [Ln 313, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FillRule {
    #[doc = " [Ln 315, Col 5]"]
    #[doc = " [Ln 316, Col 5]"]
    #[doc = " [Ln 317, Col 5]"]
    #[doc = " [Ln 318, Col 5]"]
    #[doc = " [Ln 319, Col 5]"]
    Nonzero,
    #[doc = " [Ln 321, Col 5]"]
    #[doc = " [Ln 322, Col 5]"]
    #[doc = " [Ln 323, Col 5]"]
    EvenOdd,
}
#[doc = " [Ln 327, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrokeLineCap {
    Butt,
    Round,
    Square,
}
#[doc = " [Ln 331, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrokeLineJoin {
    Miter(f32),
    Round,
    Bevel,
}
#[doc = " [Ln 335, Col 1]"]
#[doc = " [Ln 336, Col 1]"]
#[doc = " [Ln 337, Col 1]"]
#[doc = " [Ln 338, Col 1]"]
#[doc = " [Ln 339, Col 1]"]
#[doc = " [Ln 340, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpreadMethod {
    Pad,
    Reflect,
    Repeat,
}
#[doc = " [Ln 343, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}
#[doc = " [Ln 346, Col 1]"]
#[doc = " [Ln 347, Col 1]"]
#[doc = " [Ln 348, Col 1]"]
#[doc = " [Ln 349, Col 1]"]
#[doc = " [Ln 350, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontVariant {
    Normal,
    SmallCaps,
}
#[doc = " [Ln 353, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    W100,
    W200,
    W300,
    W400,
    W500,
    W600,
    W700,
    W800,
    W900,
}
#[doc = " [Ln 356, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    Generic(String),
}
#[doc = " [Ln 359, Col 1]"]
#[doc = " [Ln 360, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStretch {
    Normal,
    Wider,
    Narrower,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}
#[doc = " [Ln 375, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Background {
    #[doc = " [Ln 377, Col 5]"]
    #[doc = " [Ln 378, Col 5]"]
    #[doc = " [Ln 379, Col 5]"]
    #[doc = " [Ln 380, Col 5]"]
    #[doc = " [Ln 381, Col 5]"]
    #[doc = " [Ln 382, Col 5]"]
    #[doc = " [Ln 383, Col 5]"]
    #[doc = " [Ln 384, Col 5]"]
    Accumulate,
    #[doc = " [Ln 387, Col 5]"]
    New {
        x: Option<f32>,
        y: Option<f32>,
        width: Option<f32>,
        height: Option<f32>,
    },
}
#[doc = " [Ln 400, Col 1]"]
#[doc = " [Ln 401, Col 1]"]
#[doc = " [Ln 402, Col 1]"]
#[doc = " [Ln 403, Col 1]"]
#[doc = " [Ln 404, Col 1]"]
#[doc = " [Ln 405, Col 1]"]
#[doc = " [Ln 406, Col 1]"]
#[doc = " [Ln 407, Col 1]"]
#[doc = " [Ln 408, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeIn {
    #[doc = " [Ln 410, Col 5]"]
    #[doc = " [Ln 411, Col 5]"]
    #[doc = " [Ln 412, Col 5]"]
    #[doc = " [Ln 413, Col 5]"]
    #[doc = " [Ln 414, Col 5]"]
    #[doc = " [Ln 415, Col 5]"]
    SourceGraphic,
    #[doc = " [Ln 418, Col 5]"]
    #[doc = " [Ln 419, Col 5]"]
    #[doc = " [Ln 420, Col 5]"]
    #[doc = " [Ln 421, Col 5]"]
    #[doc = " [Ln 422, Col 5]"]
    SourceAlpha,
    #[doc = " [Ln 425, Col 5]"]
    #[doc = " [Ln 426, Col 5]"]
    BackgroundImage,
    #[doc = " [Ln 429, Col 5]"]
    BackgroundAlpha,
    #[doc = " [Ln 432, Col 5]"]
    #[doc = " [Ln 433, Col 5]"]
    #[doc = " [Ln 434, Col 5]"]
    #[doc = " [Ln 435, Col 5]"]
    FillPaint,
    #[doc = " [Ln 438, Col 5]"]
    #[doc = " [Ln 439, Col 5]"]
    #[doc = " [Ln 440, Col 5]"]
    #[doc = " [Ln 441, Col 5]"]
    StrokePaint,
    #[doc = " [Ln 444, Col 5]"]
    Result(String),
}
#[doc = " [Ln 448, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeOut {
    Position,
    Named(String),
}
#[doc = " [Ln 451, Col 1]"]
#[doc = " [Ln 452, Col 1]"]
#[doc = " [Ln 453, Col 1]"]
#[doc = " [Ln 454, Col 1]"]
#[doc = " [Ln 455, Col 1]"]
#[doc = " [Ln 456, Col 1]"]
#[doc = " [Ln 457, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeBlendMode {
    #[doc = " [Ln 459, Col 5]"]
    Normal,
    #[doc = " [Ln 461, Col 5]"]
    Multiply,
    #[doc = " [Ln 463, Col 5]"]
    Screen,
    #[doc = " [Ln 465, Col 5]"]
    Darken,
    #[doc = " [Ln 467, Col 5]"]
    Lighten,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextLengthAdjust {
    #[doc = " [Ln 473, Col 5]"]
    Spacing,
    #[doc = " [Ln 476, Col 5]"]
    #[doc = " [Ln 477, Col 5]"]
    SpacingAndGlyphs,
}
#[doc = " [Ln 481, Col 1]"]
#[doc = " [Ln 482, Col 1]"]
#[doc = " [Ln 483, Col 1]"]
#[doc = " [Ln 484, Col 1]"]
#[doc = " [Ln 485, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WritingMode {
    #[doc = " [Ln 487, Col 5]"]
    #[doc = " [Ln 488, Col 5]"]
    #[doc = " [Ln 489, Col 5]"]
    #[doc = " [Ln 490, Col 5]"]
    LrTb,
    #[doc = " [Ln 492, Col 5]"]
    #[doc = " [Ln 493, Col 5]"]
    RlTb,
    #[doc = " [Ln 495, Col 5]"]
    #[doc = " [Ln 496, Col 5]"]
    #[doc = " [Ln 497, Col 5]"]
    #[doc = " [Ln 498, Col 5]"]
    TbRl,
    #[doc = " [Ln 500, Col 5]"]
    Lr,
    #[doc = " [Ln 502, Col 5]"]
    Rl,
    #[doc = " [Ln 504, Col 5]"]
    Tb,
}
#[doc = " [Ln 508, Col 1]"]
#[doc = " [Ln 509, Col 1]"]
#[doc = " [Ln 510, Col 1]"]
#[doc = " [Ln 511, Col 1]"]
#[doc = " [Ln 512, Col 1]"]
#[doc = " [Ln 513, Col 1]"]
#[doc = " [Ln 514, Col 1]"]
#[doc = " [Ln 515, Col 1]"]
#[doc = " [Ln 516, Col 1]"]
#[doc = " [Ln 517, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextDirection {
    Ltr,
    Rtl,
}
#[doc = " [Ln 523, Col 1]"]
#[doc = " [Ln 524, Col 1]"]
#[doc = " [Ln 525, Col 1]"]
#[doc = " [Ln 526, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnicodeBidi {
    Normal,
    Embed,
    BidiOverride,
}
#[doc = " [Ln 533, Col 1]"]
#[doc = " [Ln 534, Col 1]"]
#[doc = " [Ln 535, Col 1]"]
#[doc = " [Ln 536, Col 1]"]
#[doc = " [Ln 537, Col 1]"]
#[doc = " [Ln 538, Col 1]"]
#[doc = " [Ln 539, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextAnchor {
    #[doc = " [Ln 541, Col 5]"]
    #[doc = " [Ln 542, Col 5]"]
    #[doc = " [Ln 543, Col 5]"]
    #[doc = " [Ln 544, Col 5]"]
    #[doc = " [Ln 545, Col 5]"]
    Start,
    #[doc = " [Ln 547, Col 5]"]
    #[doc = " [Ln 548, Col 5]"]
    Middle,
    #[doc = " [Ln 550, Col 5]"]
    #[doc = " [Ln 551, Col 5]"]
    #[doc = " [Ln 552, Col 5]"]
    #[doc = " [Ln 553, Col 5]"]
    #[doc = " [Ln 554, Col 5]"]
    End,
}
#[doc = " [Ln 558, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DominantBaseline {
    #[doc = " [Ln 560, Col 5]"]
    #[doc = " [Ln 561, Col 5]"]
    #[doc = " [Ln 562, Col 5]"]
    #[doc = " [Ln 563, Col 5]"]
    #[doc = " [Ln 564, Col 5]"]
    #[doc = " [Ln 565, Col 5]"]
    #[doc = " [Ln 566, Col 5]"]
    #[doc = " [Ln 567, Col 5]"]
    #[doc = " [Ln 568, Col 5]"]
    #[doc = " [Ln 569, Col 5]"]
    Auto,
    #[doc = " [Ln 571, Col 5]"]
    #[doc = " [Ln 572, Col 5]"]
    #[doc = " [Ln 573, Col 5]"]
    #[doc = " [Ln 574, Col 5]"]
    #[doc = " [Ln 575, Col 5]"]
    UseScript,
    #[doc = " [Ln 577, Col 5]"]
    #[doc = " [Ln 578, Col 5]"]
    NoChange,
    #[doc = " [Ln 580, Col 5]"]
    #[doc = " [Ln 581, Col 5]"]
    ResetSize,
    #[doc = " [Ln 583, Col 5]"]
    #[doc = " [Ln 584, Col 5]"]
    #[doc = " [Ln 585, Col 5]"]
    Ideographic,
    #[doc = " [Ln 587, Col 5]"]
    #[doc = " [Ln 588, Col 5]"]
    #[doc = " [Ln 589, Col 5]"]
    Alphabetic,
    #[doc = " [Ln 591, Col 5]"]
    #[doc = " [Ln 592, Col 5]"]
    #[doc = " [Ln 593, Col 5]"]
    Hanging,
    #[doc = " [Ln 595, Col 5]"]
    #[doc = " [Ln 596, Col 5]"]
    #[doc = " [Ln 597, Col 5]"]
    Mathematical,
    #[doc = " [Ln 599, Col 5]"]
    #[doc = " [Ln 600, Col 5]"]
    #[doc = " [Ln 601, Col 5]"]
    #[doc = " [Ln 602, Col 5]"]
    Central,
    #[doc = " [Ln 604, Col 5]"]
    #[doc = " [Ln 605, Col 5]"]
    #[doc = " [Ln 606, Col 5]"]
    #[doc = " [Ln 607, Col 5]"]
    Middle,
    #[doc = " [Ln 609, Col 5]"]
    #[doc = " [Ln 610, Col 5]"]
    #[doc = " [Ln 611, Col 5]"]
    #[doc = " [Ln 612, Col 5]"]
    #[doc = " [Ln 613, Col 5]"]
    #[doc = " [Ln 614, Col 5]"]
    #[doc = " [Ln 615, Col 5]"]
    TextAfterEdge,
    #[doc = " [Ln 617, Col 5]"]
    #[doc = " [Ln 618, Col 5]"]
    #[doc = " [Ln 619, Col 5]"]
    #[doc = " [Ln 620, Col 5]"]
    #[doc = " [Ln 621, Col 5]"]
    #[doc = " [Ln 622, Col 5]"]
    #[doc = " [Ln 623, Col 5]"]
    TextBeforeEdge,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlignmentBaseline {
    #[doc = " [Ln 628, Col 5]"]
    #[doc = " [Ln 629, Col 5]"]
    Auto,
    #[doc = " [Ln 631, Col 5]"]
    #[doc = " [Ln 632, Col 5]"]
    Baseline,
    #[doc = " [Ln 634, Col 5]"]
    #[doc = " [Ln 635, Col 5]"]
    BeforeEdge,
    #[doc = " [Ln 637, Col 5]"]
    #[doc = " [Ln 638, Col 5]"]
    TextBeforeEdge,
    #[doc = " [Ln 640, Col 5]"]
    Middle,
    #[doc = " [Ln 642, Col 5]"]
    Central,
    #[doc = " [Ln 644, Col 5]"]
    AfterEdge,
    #[doc = " [Ln 646, Col 5]"]
    TextAfterEdge,
    #[doc = " [Ln 648, Col 5]"]
    Ideographic,
    #[doc = " [Ln 650, Col 5]"]
    Alphabetic,
    #[doc = " [Ln 652, Col 5]"]
    Hanging,
    #[doc = " [Ln 654, Col 5]"]
    Mathematical,
}
#[doc = " [Ln 658, Col 1]"]
#[doc = " [Ln 659, Col 1]"]
#[doc = " [Ln 660, Col 1]"]
#[doc = " [Ln 661, Col 1]"]
#[doc = " [Ln 662, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BaselineShift {
    #[doc = " [Ln 664, Col 5]"]
    Baseline,
    #[doc = " [Ln 666, Col 5]"]
    #[doc = " [Ln 667, Col 5]"]
    #[doc = " [Ln 668, Col 5]"]
    #[doc = " [Ln 669, Col 5]"]
    #[doc = " [Ln 670, Col 5]"]
    #[doc = " [Ln 671, Col 5]"]
    #[doc = " [Ln 672, Col 5]"]
    #[doc = " [Ln 673, Col 5]"]
    Sub,
    #[doc = " [Ln 675, Col 5]"]
    #[doc = " [Ln 676, Col 5]"]
    #[doc = " [Ln 677, Col 5]"]
    #[doc = " [Ln 678, Col 5]"]
    #[doc = " [Ln 679, Col 5]"]
    #[doc = " [Ln 680, Col 5]"]
    #[doc = " [Ln 681, Col 5]"]
    #[doc = " [Ln 682, Col 5]"]
    Super,
    #[doc = " [Ln 684, Col 5]"]
    #[doc = " [Ln 685, Col 5]"]
    #[doc = " [Ln 686, Col 5]"]
    Value(Length),
}
#[doc = " [Ln 690, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextDecoration {
    Underline,
    Overline,
    LineThrough,
    Blink,
}
#[doc = " [Ln 698, Col 1]"]
#[doc = " [Ln 699, Col 1]"]
#[doc = " [Ln 700, Col 1]"]
#[doc = " [Ln 701, Col 1]"]
#[doc = " [Ln 702, Col 1]"]
#[doc = " [Ln 703, Col 1]"]
#[doc = " [Ln 704, Col 1]"]
#[doc = " [Ln 705, Col 1]"]
#[doc = " [Ln 706, Col 1]"]
#[doc = " [Ln 707, Col 1]"]
#[doc = " [Ln 708, Col 1]"]
#[doc = " [Ln 709, Col 1]"]
#[doc = " [Ln 710, Col 1]"]
#[doc = " [Ln 711, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextPathMethod {
    Align,
    Stretch,
}
#[doc = " [Ln 717, Col 1]"]
#[doc = " [Ln 718, Col 1]"]
#[doc = " [Ln 719, Col 1]"]
#[doc = " [Ln 720, Col 1]"]
#[doc = " [Ln 721, Col 1]"]
#[doc = " [Ln 722, Col 1]"]
#[doc = " [Ln 723, Col 1]"]
#[doc = " [Ln 724, Col 1]"]
#[doc = " [Ln 725, Col 1]"]
#[doc = " [Ln 726, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextPathSpacing {
    Auto,
    Exact,
}
#[doc = " [Ln 732, Col 1]"]
#[doc = " [Ln 733, Col 1]"]
#[doc = " [Ln 734, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LetterSpacing {
    Normal,
    Length(Length),
}
#[doc = " [Ln 740, Col 1]"]
#[doc = " [Ln 741, Col 1]"]
#[doc = " [Ln 742, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WordSpacing {
    Normal,
    Length(Length),
}
#[doc = " [Ln 748, Col 1]"]
#[doc = " [Ln 749, Col 1]"]
#[doc = " [Ln 750, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeetOrSlice {
    Meet,
    Slice,
}
#[doc = " [Ln 756, Col 1]"]
#[doc = " [Ln 757, Col 1]"]
#[doc = " [Ln 758, Col 1]"]
#[doc = " [Ln 759, Col 1]"]
#[doc = " [Ln 760, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PreserveAspectRatio {
    None,
    #[doc = " [Ln 763, Col 5]"]
    #[doc = " [Ln 764, Col 5]"]
    #[doc = " [Ln 765, Col 5]"]
    #[doc = " [Ln 766, Col 5]"]
    XMinYMin(MeetOrSlice),
    #[doc = " [Ln 768, Col 5]"]
    #[doc = " [Ln 769, Col 5]"]
    #[doc = " [Ln 770, Col 5]"]
    #[doc = " [Ln 771, Col 5]"]
    XMidYMin(MeetOrSlice),
    #[doc = " [Ln 773, Col 5]"]
    #[doc = " [Ln 774, Col 5]"]
    #[doc = " [Ln 775, Col 5]"]
    #[doc = " [Ln 776, Col 5]"]
    XMaxYMin(MeetOrSlice),
    #[doc = " [Ln 778, Col 5]"]
    #[doc = " [Ln 779, Col 5]"]
    #[doc = " [Ln 780, Col 5]"]
    #[doc = " [Ln 781, Col 5]"]
    XMinYMid(MeetOrSlice),
    #[doc = " [Ln 783, Col 5]"]
    #[doc = " [Ln 784, Col 5]"]
    #[doc = " [Ln 785, Col 5]"]
    #[doc = " [Ln 786, Col 5]"]
    XMidYMid(MeetOrSlice),
    #[doc = " [Ln 788, Col 5]"]
    #[doc = " [Ln 789, Col 5]"]
    #[doc = " [Ln 790, Col 5]"]
    #[doc = " [Ln 791, Col 5]"]
    XMaxYMid(MeetOrSlice),
    #[doc = " [Ln 793, Col 5]"]
    #[doc = " [Ln 794, Col 5]"]
    #[doc = " [Ln 795, Col 5]"]
    #[doc = " [Ln 796, Col 5]"]
    XMinYMax(MeetOrSlice),
    #[doc = " [Ln 798, Col 5]"]
    #[doc = " [Ln 799, Col 5]"]
    #[doc = " [Ln 800, Col 5]"]
    #[doc = " [Ln 801, Col 5]"]
    XMidYMax(MeetOrSlice),
    #[doc = " [Ln 803, Col 5]"]
    #[doc = " [Ln 804, Col 5]"]
    #[doc = " [Ln 805, Col 5]"]
    #[doc = " [Ln 806, Col 5]"]
    XMaxYMax(MeetOrSlice),
}
#[doc = " [Ln 811, Col 1]"]
#[doc = " [Ln 812, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextLayout {
    #[doc = " [Ln 814, Col 5]"]
    pub write_mode: Option<WritingMode>,
    #[doc = " [Ln 817, Col 5]"]
    pub direction: Option<TextDirection>,
    #[doc = " [Ln 821, Col 5]"]
    pub unicode_bidi: Option<UnicodeBidi>,
    #[doc = " [Ln 825, Col 5]"]
    pub anchor: Option<variable::Variable<TextAnchor>>,
    #[doc = " [Ln 829, Col 5]"]
    pub dominant_baseline: Option<variable::Variable<DominantBaseline>>,
    #[doc = " [Ln 833, Col 5]"]
    pub alignment_baseline: Option<variable::Variable<AlignmentBaseline>>,
    #[doc = " [Ln 837, Col 5]"]
    pub baseline_shift: Option<variable::Variable<BaselineShift>>,
    #[doc = " [Ln 841, Col 5]"]
    pub decoration: Option<variable::Variable<TextDecoration>>,
    #[doc = " [Ln 845, Col 5]"]
    pub letter_spacing: Option<variable::Variable<LetterSpacing>>,
    #[doc = " [Ln 849, Col 5]"]
    pub word_spacing: Option<variable::Variable<WordSpacing>>,
}
#[doc = " [Ln 854, Col 1]"]
#[doc = " [Ln 855, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithTransform(pub Vec<Transform>);
#[doc = " [Ln 861, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Id(pub String);
#[doc = " [Ln 864, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fill {
    #[doc = " [Ln 866, Col 5]"]
    #[doc = " [Ln 867, Col 5]"]
    #[doc = " [Ln 868, Col 5]"]
    pub paint: Option<Paint>,
    #[doc = " [Ln 872, Col 5]"]
    #[doc = " [Ln 873, Col 5]"]
    #[doc = " [Ln 874, Col 5]"]
    pub rule: Option<FillRule>,
    #[doc = " [Ln 878, Col 5]"]
    pub opacity: Option<f32>,
}
#[doc = " [Ln 884, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stroke {
    #[doc = " [Ln 886, Col 5]"]
    #[doc = " [Ln 887, Col 5]"]
    #[doc = " [Ln 888, Col 5]"]
    pub paint: Option<variable::Variable<Paint>>,
    #[doc = " [Ln 891, Col 5]"]
    #[doc = " [Ln 892, Col 5]"]
    #[doc = " [Ln 893, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 897, Col 5]"]
    #[doc = " [Ln 898, Col 5]"]
    #[doc = " [Ln 899, Col 5]"]
    pub linecap: Option<variable::Variable<StrokeLineCap>>,
    #[doc = " [Ln 903, Col 5]"]
    #[doc = " [Ln 904, Col 5]"]
    #[doc = " [Ln 905, Col 5]"]
    pub linejoin: Option<variable::Variable<StrokeLineJoin>>,
    #[doc = " [Ln 909, Col 5]"]
    #[doc = " [Ln 910, Col 5]"]
    #[doc = " [Ln 911, Col 5]"]
    #[doc = " [Ln 912, Col 5]"]
    #[doc = " [Ln 913, Col 5]"]
    #[doc = " [Ln 914, Col 5]"]
    pub dasharray: Option<variable::Variable<Vec<Length>>>,
    #[doc = " [Ln 917, Col 5]"]
    #[doc = " [Ln 918, Col 5]"]
    #[doc = " [Ln 919, Col 5]"]
    pub dashoffset: Option<variable::Variable<Length>>,
    #[doc = " [Ln 923, Col 5]"]
    pub opacity: Option<variable::Variable<f32>>,
}
#[doc = " [Ln 929, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Font {
    #[doc = " [Ln 931, Col 5]"]
    pub family: Option<variable::Variable<Vec<FontFamily>>>,
    #[doc = " [Ln 934, Col 5]"]
    pub style: Option<variable::Variable<FontStyle>>,
    #[doc = " [Ln 937, Col 5]"]
    pub variant: Option<variable::Variable<FontVariant>>,
    #[doc = " [Ln 940, Col 5]"]
    pub weight: Option<variable::Variable<FontWeight>>,
    #[doc = " [Ln 943, Col 5]"]
    #[doc = " [Ln 944, Col 5]"]
    pub size: Option<variable::Variable<Length>>,
    #[doc = " [Ln 947, Col 5]"]
    pub stretch: Option<variable::Variable<FontStretch>>,
}
#[doc = " [Ln 953, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnableBackground(pub Background);
#[doc = " [Ln 956, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithFilter(pub String);
#[doc = " [Ln 959, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithClipPath(pub FuncIri);
#[doc = " [Ln 963, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithMask(pub FuncIri);
#[doc = " [Ln 967, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Opacity(pub f32);
#[doc = " [Ln 971, Col 1]"]
#[doc = " [Ln 972, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewBox {
    #[doc = " [Ln 975, Col 5]"]
    pub minx: variable::Variable<f32>,
    #[doc = " [Ln 978, Col 5]"]
    pub miny: variable::Variable<f32>,
    #[doc = " [Ln 981, Col 5]"]
    pub width: variable::Variable<f32>,
    #[doc = " [Ln 984, Col 5]"]
    pub height: variable::Variable<f32>,
    #[doc = " [Ln 987, Col 5]"]
    pub aspect: Option<variable::Variable<PreserveAspectRatio>>,
}
#[doc = " [Ln 1035, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Canvas {
    #[doc = " [Ln 1038, Col 5]"]
    pub width: variable::Variable<Length>,
    #[doc = " [Ln 1041, Col 5]"]
    pub height: variable::Variable<Length>,
}
#[doc = " [Ln 1046, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mask {
    #[doc = " [Ln 1048, Col 5]"]
    #[doc = " [Ln 1049, Col 5]"]
    #[doc = " [Ln 1050, Col 5]"]
    #[doc = " [Ln 1051, Col 5]"]
    #[doc = " [Ln 1052, Col 5]"]
    #[doc = " [Ln 1053, Col 5]"]
    #[doc = " [Ln 1054, Col 5]"]
    #[doc = " [Ln 1055, Col 5]"]
    #[doc = " [Ln 1056, Col 5]"]
    #[doc = " [Ln 1057, Col 5]"]
    pub units: Option<variable::Variable<Coords>>,
    #[doc = " [Ln 1061, Col 5]"]
    #[doc = " [Ln 1062, Col 5]"]
    #[doc = " [Ln 1063, Col 5]"]
    #[doc = " [Ln 1064, Col 5]"]
    #[doc = " [Ln 1065, Col 5]"]
    #[doc = " [Ln 1066, Col 5]"]
    #[doc = " [Ln 1067, Col 5]"]
    #[doc = " [Ln 1068, Col 5]"]
    #[doc = " [Ln 1069, Col 5]"]
    #[doc = " [Ln 1070, Col 5]"]
    pub content_units: Option<variable::Variable<Coords>>,
    #[doc = " [Ln 1074, Col 5]"]
    #[doc = " [Ln 1075, Col 5]"]
    #[doc = " [Ln 1076, Col 5]"]
    #[doc = " [Ln 1077, Col 5]"]
    #[doc = " [Ln 1078, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1082, Col 5]"]
    #[doc = " [Ln 1083, Col 5]"]
    #[doc = " [Ln 1084, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1088, Col 5]"]
    #[doc = " [Ln 1089, Col 5]"]
    #[doc = " [Ln 1090, Col 5]"]
    #[doc = " [Ln 1091, Col 5]"]
    #[doc = " [Ln 1092, Col 5]"]
    #[doc = " [Ln 1093, Col 5]"]
    #[doc = " [Ln 1094, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1098, Col 5]"]
    #[doc = " [Ln 1099, Col 5]"]
    #[doc = " [Ln 1100, Col 5]"]
    #[doc = " [Ln 1101, Col 5]"]
    #[doc = " [Ln 1102, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
}
#[doc = " [Ln 1107, Col 1]"]
#[doc = " [Ln 1108, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClipPath(
    #[doc = " [Ln 1110, Col 5]"]
    #[doc = " [Ln 1111, Col 5]"]
    #[doc = " [Ln 1112, Col 5]"]
    #[doc = " [Ln 1113, Col 5]"]
    #[doc = " [Ln 1114, Col 5]"]
    #[doc = " [Ln 1115, Col 5]"]
    #[doc = " [Ln 1116, Col 5]"]
    #[doc = " [Ln 1117, Col 5]"]
    #[doc = " [Ln 1118, Col 5]"]
    #[doc = " [Ln 1119, Col 5]"]
    #[doc = " [Ln 1120, Col 5]"]
    pub Option<variable::Variable<Coords>>,
);
#[doc = " [Ln 1125, Col 1]"]
#[doc = " [Ln 1126, Col 1]"]
#[doc = " [Ln 1127, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Filter {
    #[doc = " [Ln 1129, Col 5]"]
    #[doc = " [Ln 1130, Col 5]"]
    #[doc = " [Ln 1131, Col 5]"]
    #[doc = " [Ln 1132, Col 5]"]
    #[doc = " [Ln 1133, Col 5]"]
    #[doc = " [Ln 1134, Col 5]"]
    #[doc = " [Ln 1135, Col 5]"]
    #[doc = " [Ln 1136, Col 5]"]
    #[doc = " [Ln 1137, Col 5]"]
    #[doc = " [Ln 1138, Col 5]"]
    #[doc = " [Ln 1139, Col 5]"]
    pub units: Option<variable::Variable<Coords>>,
    #[doc = " [Ln 1143, Col 5]"]
    #[doc = " [Ln 1144, Col 5]"]
    #[doc = " [Ln 1145, Col 5]"]
    #[doc = " [Ln 1146, Col 5]"]
    #[doc = " [Ln 1147, Col 5]"]
    #[doc = " [Ln 1148, Col 5]"]
    #[doc = " [Ln 1149, Col 5]"]
    #[doc = " [Ln 1150, Col 5]"]
    #[doc = " [Ln 1151, Col 5]"]
    #[doc = " [Ln 1152, Col 5]"]
    #[doc = " [Ln 1153, Col 5]"]
    #[doc = " [Ln 1154, Col 5]"]
    #[doc = " [Ln 1155, Col 5]"]
    pub primitive_units: Option<variable::Variable<Coords>>,
    #[doc = " [Ln 1159, Col 5]"]
    #[doc = " [Ln 1160, Col 5]"]
    #[doc = " [Ln 1161, Col 5]"]
    #[doc = " [Ln 1162, Col 5]"]
    #[doc = " [Ln 1163, Col 5]"]
    #[doc = " [Ln 1164, Col 5]"]
    #[doc = " [Ln 1165, Col 5]"]
    #[doc = " [Ln 1166, Col 5]"]
    #[doc = " [Ln 1167, Col 5]"]
    #[doc = " [Ln 1168, Col 5]"]
    #[doc = " [Ln 1169, Col 5]"]
    #[doc = " [Ln 1170, Col 5]"]
    #[doc = " [Ln 1171, Col 5]"]
    #[doc = " [Ln 1172, Col 5]"]
    #[doc = " [Ln 1173, Col 5]"]
    #[doc = " [Ln 1174, Col 5]"]
    #[doc = " [Ln 1175, Col 5]"]
    #[doc = " [Ln 1176, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1180, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1184, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1188, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1192, Col 5]"]
    #[doc = " [Ln 1193, Col 5]"]
    #[doc = " [Ln 1194, Col 5]"]
    #[doc = " [Ln 1195, Col 5]"]
    #[doc = " [Ln 1196, Col 5]"]
    #[doc = " [Ln 1197, Col 5]"]
    #[doc = " [Ln 1198, Col 5]"]
    #[doc = " [Ln 1199, Col 5]"]
    #[doc = " [Ln 1200, Col 5]"]
    #[doc = " [Ln 1201, Col 5]"]
    #[doc = " [Ln 1202, Col 5]"]
    #[doc = " [Ln 1203, Col 5]"]
    pub res: Option<variable::Variable<NumberOptNumber>>,
}
#[doc = " [Ln 1209, Col 1]"]
#[doc = " [Ln 1210, Col 1]"]
#[doc = " [Ln 1211, Col 1]"]
#[doc = " [Ln 1212, Col 1]"]
#[doc = " [Ln 1213, Col 1]"]
#[doc = " [Ln 1214, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDistantLight {
    #[doc = " [Ln 1216, Col 5]"]
    #[doc = " [Ln 1217, Col 5]"]
    #[doc = " [Ln 1218, Col 5]"]
    pub azimuth: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1222, Col 5]"]
    #[doc = " [Ln 1223, Col 5]"]
    #[doc = " [Ln 1224, Col 5]"]
    pub elevation: Option<variable::Variable<f32>>,
}
#[doc = " [Ln 1229, Col 1]"]
#[doc = " [Ln 1230, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FePointLight {
    #[doc = " [Ln 1232, Col 5]"]
    #[doc = " [Ln 1233, Col 5]"]
    #[doc = " [Ln 1234, Col 5]"]
    pub x: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1238, Col 5]"]
    #[doc = " [Ln 1239, Col 5]"]
    #[doc = " [Ln 1240, Col 5]"]
    pub y: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1244, Col 5]"]
    #[doc = " [Ln 1245, Col 5]"]
    #[doc = " [Ln 1246, Col 5]"]
    #[doc = " [Ln 1247, Col 5]"]
    #[doc = " [Ln 1248, Col 5]"]
    pub z: Option<variable::Variable<f32>>,
}
#[doc = " [Ln 1254, Col 1]"]
#[doc = " [Ln 1255, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeSpotLight {
    #[doc = " [Ln 1257, Col 5]"]
    #[doc = " [Ln 1258, Col 5]"]
    #[doc = " [Ln 1259, Col 5]"]
    pub x: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1263, Col 5]"]
    #[doc = " [Ln 1264, Col 5]"]
    #[doc = " [Ln 1265, Col 5]"]
    pub y: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1269, Col 5]"]
    #[doc = " [Ln 1270, Col 5]"]
    #[doc = " [Ln 1271, Col 5]"]
    #[doc = " [Ln 1272, Col 5]"]
    #[doc = " [Ln 1273, Col 5]"]
    pub z: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1277, Col 5]"]
    #[doc = " [Ln 1278, Col 5]"]
    #[doc = " [Ln 1279, Col 5]"]
    #[doc = " [Ln 1280, Col 5]"]
    pub point_at_x: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1284, Col 5]"]
    #[doc = " [Ln 1285, Col 5]"]
    #[doc = " [Ln 1286, Col 5]"]
    #[doc = " [Ln 1287, Col 5]"]
    pub point_at_y: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1291, Col 5]"]
    #[doc = " [Ln 1292, Col 5]"]
    #[doc = " [Ln 1293, Col 5]"]
    #[doc = " [Ln 1294, Col 5]"]
    #[doc = " [Ln 1295, Col 5]"]
    pub point_at_z: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1299, Col 5]"]
    #[doc = " [Ln 1300, Col 5]"]
    #[doc = " [Ln 1301, Col 5]"]
    pub specular_exponent: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1305, Col 5]"]
    #[doc = " [Ln 1306, Col 5]"]
    #[doc = " [Ln 1307, Col 5]"]
    #[doc = " [Ln 1308, Col 5]"]
    #[doc = " [Ln 1309, Col 5]"]
    #[doc = " [Ln 1310, Col 5]"]
    pub limiting_cone_angle: Option<variable::Variable<f32>>,
}
#[doc = " [Ln 1315, Col 1]"]
#[doc = " [Ln 1316, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeBlend {
    #[doc = " [Ln 1318, Col 5]"]
    pub mode: Option<variable::Variable<FeBlendMode>>,
    #[doc = " [Ln 1322, Col 5]"]
    pub r#in: Option<variable::Variable<FeIn>>,
    #[doc = " [Ln 1326, Col 5]"]
    pub in2: Option<variable::Variable<FeIn>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1332, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeColorMatrixValues {
    #[doc = " [Ln 1334, Col 5]"]
    Matrix([f32; 20usize]),
    #[doc = " [Ln 1337, Col 5]"]
    #[doc = " [Ln 1338, Col 5]"]
    #[doc = " [Ln 1339, Col 5]"]
    Saturate(f32),
    #[doc = " [Ln 1342, Col 5]"]
    #[doc = " [Ln 1343, Col 5]"]
    #[doc = " [Ln 1344, Col 5]"]
    HueRotate(f32),
    #[doc = " [Ln 1347, Col 5]"]
    #[doc = " [Ln 1348, Col 5]"]
    #[doc = " [Ln 1349, Col 5]"]
    LuminanceToAlpha,
}
#[doc = " [Ln 1354, Col 1]"]
#[doc = " [Ln 1355, Col 1]"]
#[doc = " [Ln 1356, Col 1]"]
#[doc = " [Ln 1357, Col 1]"]
#[doc = " [Ln 1358, Col 1]"]
#[doc = " [Ln 1359, Col 1]"]
#[doc = " [Ln 1360, Col 1]"]
#[doc = " [Ln 1361, Col 1]"]
#[doc = " [Ln 1362, Col 1]"]
#[doc = " [Ln 1363, Col 1]"]
#[doc = " [Ln 1364, Col 1]"]
#[doc = " [Ln 1365, Col 1]"]
#[doc = " [Ln 1366, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeColorMatrix {
    #[doc = " [Ln 1368, Col 5]"]
    pub r#in: variable::Variable<FeIn>,
    #[doc = " [Ln 1372, Col 5]"]
    pub values: variable::Variable<FeColorMatrixValues>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1378, Col 1]"]
#[doc = " [Ln 1379, Col 1]"]
#[doc = " [Ln 1380, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeFunc {
    #[doc = " [Ln 1382, Col 5]"]
    Identity,
    #[doc = " [Ln 1385, Col 5]"]
    #[doc = " [Ln 1386, Col 5]"]
    #[doc = " [Ln 1387, Col 5]"]
    #[doc = " [Ln 1388, Col 5]"]
    #[doc = " [Ln 1389, Col 5]"]
    #[doc = " [Ln 1390, Col 5]"]
    #[doc = " [Ln 1391, Col 5]"]
    #[doc = " [Ln 1392, Col 5]"]
    #[doc = " [Ln 1393, Col 5]"]
    #[doc = " [Ln 1394, Col 5]"]
    #[doc = " [Ln 1395, Col 5]"]
    #[doc = " [Ln 1396, Col 5]"]
    #[doc = " [Ln 1397, Col 5]"]
    #[doc = " [Ln 1398, Col 5]"]
    #[doc = " [Ln 1399, Col 5]"]
    Table(Vec<f32>),
    #[doc = " [Ln 1402, Col 5]"]
    #[doc = " [Ln 1403, Col 5]"]
    #[doc = " [Ln 1404, Col 5]"]
    #[doc = " [Ln 1405, Col 5]"]
    #[doc = " [Ln 1406, Col 5]"]
    #[doc = " [Ln 1407, Col 5]"]
    #[doc = " [Ln 1408, Col 5]"]
    #[doc = " [Ln 1409, Col 5]"]
    #[doc = " [Ln 1410, Col 5]"]
    #[doc = " [Ln 1411, Col 5]"]
    #[doc = " [Ln 1412, Col 5]"]
    #[doc = " [Ln 1413, Col 5]"]
    #[doc = " [Ln 1414, Col 5]"]
    #[doc = " [Ln 1415, Col 5]"]
    #[doc = " [Ln 1416, Col 5]"]
    Discrete(Vec<f32>),
    #[doc = " [Ln 1419, Col 5]"]
    #[doc = " [Ln 1420, Col 5]"]
    #[doc = " [Ln 1421, Col 5]"]
    Linear {
        #[doc = " [Ln 1423, Col 9]"]
        slope: f32,
        #[doc = " [Ln 1425, Col 9]"]
        intercept: f32,
    },
    #[doc = " [Ln 1429, Col 5]"]
    #[doc = " [Ln 1430, Col 5]"]
    #[doc = " [Ln 1431, Col 5]"]
    Gamma {
        #[doc = " [Ln 1433, Col 9]"]
        #[doc = " [Ln 1434, Col 9]"]
        amplitude: f32,
        #[doc = " [Ln 1436, Col 9]"]
        #[doc = " [Ln 1437, Col 9]"]
        exponent: f32,
        #[doc = " [Ln 1439, Col 9]"]
        #[doc = " [Ln 1440, Col 9]"]
        offset: f32,
    },
}
#[doc = " [Ln 1445, Col 1]"]
#[doc = " [Ln 1446, Col 1]"]
#[doc = " [Ln 1447, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeCompositeOperator {
    Over,
    In,
    Out,
    Atop,
    Xor,
    Arithmetic {
        #[doc = " [Ln 1455, Col 9]"]
        #[doc = " [Ln 1456, Col 9]"]
        k1: f32,
        #[doc = " [Ln 1459, Col 9]"]
        #[doc = " [Ln 1460, Col 9]"]
        k2: f32,
        #[doc = " [Ln 1463, Col 9]"]
        #[doc = " [Ln 1464, Col 9]"]
        k3: f32,
        #[doc = " [Ln 1467, Col 9]"]
        #[doc = " [Ln 1468, Col 9]"]
        k4: f32,
    },
}
#[doc = " [Ln 1473, Col 1]"]
#[doc = " [Ln 1474, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeConvolveMatrixEdgeMode {
    Duplicate,
    Wrap,
    None,
}
#[doc = " [Ln 1481, Col 1]"]
#[doc = " [Ln 1482, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeMorphologyOperator {
    Erode,
    Dilate,
}
#[doc = " [Ln 1489, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeStitchTiles {
    #[doc = " [Ln 1491, Col 5]"]
    #[doc = " [Ln 1492, Col 5]"]
    #[doc = " [Ln 1493, Col 5]"]
    #[doc = " [Ln 1494, Col 5]"]
    #[doc = " [Ln 1495, Col 5]"]
    #[doc = " [Ln 1496, Col 5]"]
    #[doc = " [Ln 1497, Col 5]"]
    Stitch,
    #[doc = " [Ln 1499, Col 5]"]
    #[doc = " [Ln 1500, Col 5]"]
    NoStitch,
}
#[doc = " [Ln 1504, Col 1]"]
#[doc = " [Ln 1505, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeTurbulenceType {
    FractalNoise,
    Turbulence,
}
#[doc = " [Ln 1512, Col 1]"]
#[doc = " [Ln 1513, Col 1]"]
#[doc = " [Ln 1514, Col 1]"]
#[doc = " [Ln 1515, Col 1]"]
#[doc = " [Ln 1516, Col 1]"]
#[doc = " [Ln 1517, Col 1]"]
#[doc = " [Ln 1518, Col 1]"]
#[doc = " [Ln 1519, Col 1]"]
#[doc = " [Ln 1520, Col 1]"]
#[doc = " [Ln 1521, Col 1]"]
#[doc = " [Ln 1522, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeComponentTransfer(pub Option<variable::Variable<FeIn>>);
#[doc = " [Ln 1525, Col 1]"]
#[doc = " [Ln 1526, Col 1]"]
#[doc = " [Ln 1527, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncA(pub FeFunc);
#[doc = " [Ln 1530, Col 1]"]
#[doc = " [Ln 1531, Col 1]"]
#[doc = " [Ln 1532, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncR(pub FeFunc);
#[doc = " [Ln 1535, Col 1]"]
#[doc = " [Ln 1536, Col 1]"]
#[doc = " [Ln 1537, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncG(pub FeFunc);
#[doc = " [Ln 1540, Col 1]"]
#[doc = " [Ln 1541, Col 1]"]
#[doc = " [Ln 1542, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncB(pub FeFunc);
#[doc = " [Ln 1545, Col 1]"]
#[doc = " [Ln 1546, Col 1]"]
#[doc = " [Ln 1547, Col 1]"]
#[doc = " [Ln 1548, Col 1]"]
#[doc = " [Ln 1549, Col 1]"]
#[doc = " [Ln 1550, Col 1]"]
#[doc = " [Ln 1551, Col 1]"]
#[doc = " [Ln 1552, Col 1]"]
#[doc = " [Ln 1553, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeComposite {
    #[doc = " [Ln 1555, Col 5]"]
    pub r#in: Option<variable::Variable<FeIn>>,
    #[doc = " [Ln 1559, Col 5]"]
    pub in2: variable::Variable<FeIn>,
    #[doc = " [Ln 1563, Col 5]"]
    pub operator: Option<variable::Variable<FeCompositeOperator>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1569, Col 1]"]
#[doc = " [Ln 1570, Col 1]"]
#[doc = " [Ln 1571, Col 1]"]
#[doc = " [Ln 1572, Col 1]"]
#[doc = " [Ln 1573, Col 1]"]
#[doc = " [Ln 1574, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeConvolveMatrix {
    #[doc = " [Ln 1576, Col 5]"]
    pub r#in: Option<variable::Variable<FeIn>>,
    #[doc = " [Ln 1580, Col 5]"]
    #[doc = " [Ln 1581, Col 5]"]
    #[doc = " [Ln 1582, Col 5]"]
    #[doc = " [Ln 1583, Col 5]"]
    #[doc = " [Ln 1584, Col 5]"]
    #[doc = " [Ln 1585, Col 5]"]
    #[doc = " [Ln 1586, Col 5]"]
    #[doc = " [Ln 1587, Col 5]"]
    pub order: Option<variable::Variable<NumberOptNumber>>,
    #[doc = " [Ln 1591, Col 5]"]
    #[doc = " [Ln 1592, Col 5]"]
    pub kernel: variable::Variable<Vec<f32>>,
    #[doc = " [Ln 1596, Col 5]"]
    #[doc = " [Ln 1597, Col 5]"]
    #[doc = " [Ln 1598, Col 5]"]
    #[doc = " [Ln 1599, Col 5]"]
    #[doc = " [Ln 1600, Col 5]"]
    pub divisor: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1604, Col 5]"]
    #[doc = " [Ln 1605, Col 5]"]
    #[doc = " [Ln 1606, Col 5]"]
    #[doc = " [Ln 1607, Col 5]"]
    #[doc = " [Ln 1608, Col 5]"]
    pub bias: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1612, Col 5]"]
    #[doc = " [Ln 1613, Col 5]"]
    #[doc = " [Ln 1614, Col 5]"]
    #[doc = " [Ln 1615, Col 5]"]
    #[doc = " [Ln 1616, Col 5]"]
    pub target_x: Option<variable::Variable<i32>>,
    #[doc = " [Ln 1620, Col 5]"]
    #[doc = " [Ln 1621, Col 5]"]
    #[doc = " [Ln 1622, Col 5]"]
    pub target_y: Option<variable::Variable<i32>>,
    #[doc = " [Ln 1626, Col 5]"]
    #[doc = " [Ln 1627, Col 5]"]
    pub edge_mode: variable::Variable<FeConvolveMatrixEdgeMode>,
    #[doc = " [Ln 1631, Col 5]"]
    #[doc = " [Ln 1632, Col 5]"]
    #[doc = " [Ln 1633, Col 5]"]
    #[doc = " [Ln 1634, Col 5]"]
    #[doc = " [Ln 1635, Col 5]"]
    #[doc = " [Ln 1636, Col 5]"]
    #[doc = " [Ln 1637, Col 5]"]
    #[doc = " [Ln 1638, Col 5]"]
    #[doc = " [Ln 1639, Col 5]"]
    #[doc = " [Ln 1640, Col 5]"]
    #[doc = " [Ln 1641, Col 5]"]
    #[doc = " [Ln 1642, Col 5]"]
    pub kernel_unit_len: Option<variable::Variable<NumberOptNumber>>,
    #[doc = " [Ln 1646, Col 5]"]
    #[doc = " [Ln 1647, Col 5]"]
    #[doc = " [Ln 1648, Col 5]"]
    pub preserve_alpha: variable::Variable<bool>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1653, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDiffuseLighting {
    #[doc = " [Ln 1655, Col 5]"]
    pub r#in: variable::Variable<FeIn>,
    #[doc = " [Ln 1659, Col 5]"]
    #[doc = " [Ln 1660, Col 5]"]
    #[doc = " [Ln 1661, Col 5]"]
    pub surface_scale: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1665, Col 5]"]
    #[doc = " [Ln 1666, Col 5]"]
    #[doc = " [Ln 1667, Col 5]"]
    pub diffuse_constant: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1671, Col 5]"]
    #[doc = " [Ln 1672, Col 5]"]
    #[doc = " [Ln 1673, Col 5]"]
    #[doc = " [Ln 1674, Col 5]"]
    #[doc = " [Ln 1675, Col 5]"]
    #[doc = " [Ln 1676, Col 5]"]
    #[doc = " [Ln 1677, Col 5]"]
    #[doc = " [Ln 1678, Col 5]"]
    #[doc = " [Ln 1679, Col 5]"]
    #[doc = " [Ln 1680, Col 5]"]
    #[doc = " [Ln 1681, Col 5]"]
    #[doc = " [Ln 1682, Col 5]"]
    pub kernel_unit_len: Option<variable::Variable<NumberOptNumber>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1688, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDisplacementMap {
    #[doc = " [Ln 1690, Col 5]"]
    pub r#in: Option<variable::Variable<FeIn>>,
    #[doc = " [Ln 1694, Col 5]"]
    pub in2: variable::Variable<FeIn>,
    #[doc = " [Ln 1698, Col 5]"]
    #[doc = " [Ln 1699, Col 5]"]
    #[doc = " [Ln 1700, Col 5]"]
    #[doc = " [Ln 1701, Col 5]"]
    #[doc = " [Ln 1702, Col 5]"]
    #[doc = " [Ln 1703, Col 5]"]
    pub scale: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1707, Col 5]"]
    #[doc = " [Ln 1708, Col 5]"]
    #[doc = " [Ln 1709, Col 5]"]
    pub x_channel_selector: Option<variable::Variable<Channel>>,
    #[doc = " [Ln 1713, Col 5]"]
    #[doc = " [Ln 1714, Col 5]"]
    #[doc = " [Ln 1715, Col 5]"]
    pub y_channel_selector: Option<variable::Variable<Channel>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1721, Col 1]"]
#[doc = " [Ln 1722, Col 1]"]
#[doc = " [Ln 1723, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFlood {
    #[doc = " [Ln 1725, Col 5]"]
    pub color: Option<variable::Variable<Rgb>>,
    #[doc = " [Ln 1728, Col 5]"]
    pub opacity: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1734, Col 1]"]
#[doc = " [Ln 1735, Col 1]"]
#[doc = " [Ln 1736, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeGaussianBlur {
    #[doc = " [Ln 1738, Col 5]"]
    pub r#in: Option<variable::Variable<FeIn>>,
    #[doc = " [Ln 1742, Col 5]"]
    #[doc = " [Ln 1743, Col 5]"]
    #[doc = " [Ln 1744, Col 5]"]
    #[doc = " [Ln 1745, Col 5]"]
    #[doc = " [Ln 1746, Col 5]"]
    #[doc = " [Ln 1747, Col 5]"]
    #[doc = " [Ln 1748, Col 5]"]
    #[doc = " [Ln 1749, Col 5]"]
    pub std_deviation: Option<variable::Variable<NumberOptNumber>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1754, Col 1]"]
#[doc = " [Ln 1755, Col 1]"]
#[doc = " [Ln 1756, Col 1]"]
#[doc = " [Ln 1757, Col 1]"]
#[doc = " [Ln 1758, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMerge {
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1761, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMergeNode(pub variable::Variable<FeIn>);
#[doc = " [Ln 1764, Col 1]"]
#[doc = " [Ln 1765, Col 1]"]
#[doc = " [Ln 1766, Col 1]"]
#[doc = " [Ln 1767, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeImage {
    #[doc = " [Ln 1769, Col 5]"]
    pub href: variable::Variable<FuncIri>,
    #[doc = " [Ln 1773, Col 5]"]
    pub aspect: Option<variable::Variable<PreserveAspectRatio>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1778, Col 1]"]
#[doc = " [Ln 1779, Col 1]"]
#[doc = " [Ln 1780, Col 1]"]
#[doc = " [Ln 1781, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMorphology {
    #[doc = " [Ln 1783, Col 5]"]
    pub r#in: Option<variable::Variable<FeIn>>,
    #[doc = " [Ln 1787, Col 5]"]
    pub mode: Option<variable::Variable<FeMorphologyOperator>>,
    #[doc = " [Ln 1791, Col 5]"]
    #[doc = " [Ln 1792, Col 5]"]
    #[doc = " [Ln 1793, Col 5]"]
    #[doc = " [Ln 1794, Col 5]"]
    #[doc = " [Ln 1795, Col 5]"]
    #[doc = " [Ln 1796, Col 5]"]
    #[doc = " [Ln 1797, Col 5]"]
    #[doc = " [Ln 1798, Col 5]"]
    #[doc = " [Ln 1799, Col 5]"]
    pub radius: Option<variable::Variable<NumberOptNumber>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1804, Col 1]"]
#[doc = " [Ln 1805, Col 1]"]
#[doc = " [Ln 1806, Col 1]"]
#[doc = " [Ln 1807, Col 1]"]
#[doc = " [Ln 1808, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeOffset {
    #[doc = " [Ln 1810, Col 5]"]
    pub r#in: Option<variable::Variable<FeIn>>,
    #[doc = " [Ln 1814, Col 5]"]
    #[doc = " [Ln 1815, Col 5]"]
    #[doc = " [Ln 1816, Col 5]"]
    #[doc = " [Ln 1817, Col 5]"]
    pub dx: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1821, Col 5]"]
    #[doc = " [Ln 1822, Col 5]"]
    #[doc = " [Ln 1823, Col 5]"]
    #[doc = " [Ln 1824, Col 5]"]
    pub dy: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1829, Col 1]"]
#[doc = " [Ln 1830, Col 1]"]
#[doc = " [Ln 1831, Col 1]"]
#[doc = " [Ln 1832, Col 1]"]
#[doc = " [Ln 1833, Col 1]"]
#[doc = " [Ln 1834, Col 1]"]
#[doc = " [Ln 1835, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeSpecularLighting {
    #[doc = " [Ln 1837, Col 5]"]
    pub r#in: Option<variable::Variable<FeIn>>,
    #[doc = " [Ln 1841, Col 5]"]
    #[doc = " [Ln 1842, Col 5]"]
    #[doc = " [Ln 1843, Col 5]"]
    pub surface_scale: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1847, Col 5]"]
    #[doc = " [Ln 1848, Col 5]"]
    #[doc = " [Ln 1849, Col 5]"]
    pub specular_constant: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1853, Col 5]"]
    #[doc = " [Ln 1854, Col 5]"]
    #[doc = " [Ln 1855, Col 5]"]
    pub specular_exponent: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1859, Col 5]"]
    #[doc = " [Ln 1860, Col 5]"]
    #[doc = " [Ln 1861, Col 5]"]
    #[doc = " [Ln 1862, Col 5]"]
    #[doc = " [Ln 1863, Col 5]"]
    #[doc = " [Ln 1864, Col 5]"]
    #[doc = " [Ln 1865, Col 5]"]
    #[doc = " [Ln 1866, Col 5]"]
    #[doc = " [Ln 1867, Col 5]"]
    #[doc = " [Ln 1868, Col 5]"]
    #[doc = " [Ln 1869, Col 5]"]
    #[doc = " [Ln 1870, Col 5]"]
    pub kernel_unit_len: Option<variable::Variable<NumberOptNumber>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1875, Col 1]"]
#[doc = " [Ln 1876, Col 1]"]
#[doc = " [Ln 1877, Col 1]"]
#[doc = " [Ln 1878, Col 1]"]
#[doc = " [Ln 1879, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeTile {
    #[doc = " [Ln 1881, Col 5]"]
    pub r#in: Option<variable::Variable<FeIn>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1886, Col 1]"]
#[doc = " [Ln 1887, Col 1]"]
#[doc = " [Ln 1888, Col 1]"]
#[doc = " [Ln 1889, Col 1]"]
#[doc = " [Ln 1890, Col 1]"]
#[doc = " [Ln 1891, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeTurbulence {
    #[doc = " [Ln 1893, Col 5]"]
    #[doc = " [Ln 1894, Col 5]"]
    #[doc = " [Ln 1895, Col 5]"]
    #[doc = " [Ln 1896, Col 5]"]
    #[doc = " [Ln 1897, Col 5]"]
    #[doc = " [Ln 1898, Col 5]"]
    #[doc = " [Ln 1899, Col 5]"]
    pub base_frequency: Option<variable::Variable<NumberOptNumber>>,
    #[doc = " [Ln 1903, Col 5]"]
    #[doc = " [Ln 1904, Col 5]"]
    #[doc = " [Ln 1905, Col 5]"]
    pub num_octaves: Option<variable::Variable<i32>>,
    #[doc = " [Ln 1909, Col 5]"]
    #[doc = " [Ln 1910, Col 5]"]
    #[doc = " [Ln 1911, Col 5]"]
    #[doc = " [Ln 1912, Col 5]"]
    #[doc = " [Ln 1913, Col 5]"]
    pub seed: Option<variable::Variable<f32>>,
    #[doc = " [Ln 1917, Col 5]"]
    pub stitch_tiles: Option<variable::Variable<FeStitchTiles>>,
    #[doc = " [Ln 1921, Col 5]"]
    pub r#type: Option<variable::Variable<FeTurbulenceType>>,
    #[doc = " [Ln 1006, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1010, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1014, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1018, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1022, Col 5]"]
    #[doc = " [Ln 1023, Col 5]"]
    #[doc = " [Ln 1024, Col 5]"]
    #[doc = " [Ln 1025, Col 5]"]
    #[doc = " [Ln 1026, Col 5]"]
    #[doc = " [Ln 1027, Col 5]"]
    #[doc = " [Ln 1028, Col 5]"]
    #[doc = " [Ln 1029, Col 5]"]
    #[doc = " [Ln 1030, Col 5]"]
    pub result: Option<variable::Variable<String>>,
}
#[doc = " [Ln 1927, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinearGradient {
    #[doc = " [Ln 1929, Col 5]"]
    pub units: Option<variable::Variable<Coords>>,
    #[doc = " [Ln 1933, Col 5]"]
    #[doc = " [Ln 1934, Col 5]"]
    #[doc = " [Ln 1935, Col 5]"]
    #[doc = " [Ln 1936, Col 5]"]
    #[doc = " [Ln 1937, Col 5]"]
    #[doc = " [Ln 1938, Col 5]"]
    #[doc = " [Ln 1939, Col 5]"]
    #[doc = " [Ln 1940, Col 5]"]
    #[doc = " [Ln 1941, Col 5]"]
    pub transform: Option<variable::Variable<Transform>>,
    #[doc = " [Ln 1945, Col 5]"]
    #[doc = " [Ln 1946, Col 5]"]
    #[doc = " [Ln 1947, Col 5]"]
    #[doc = " [Ln 1948, Col 5]"]
    #[doc = " [Ln 1949, Col 5]"]
    #[doc = " [Ln 1950, Col 5]"]
    #[doc = " [Ln 1951, Col 5]"]
    pub x1: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1955, Col 5]"]
    pub y1: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1959, Col 5]"]
    pub x2: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1963, Col 5]"]
    pub y2: Option<variable::Variable<Length>>,
    #[doc = " [Ln 1967, Col 5]"]
    pub spread: Option<variable::Variable<SpreadMethod>>,
}
#[doc = " [Ln 1972, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RadialGradient {
    #[doc = " [Ln 1974, Col 5]"]
    pub unit: Option<variable::Variable<Coords>>,
    #[doc = " [Ln 1978, Col 5]"]
    #[doc = " [Ln 1979, Col 5]"]
    #[doc = " [Ln 1980, Col 5]"]
    #[doc = " [Ln 1981, Col 5]"]
    #[doc = " [Ln 1982, Col 5]"]
    #[doc = " [Ln 1983, Col 5]"]
    #[doc = " [Ln 1984, Col 5]"]
    #[doc = " [Ln 1985, Col 5]"]
    #[doc = " [Ln 1986, Col 5]"]
    pub transform: Option<variable::Variable<Transform>>,
    #[doc = " [Ln 1990, Col 5]"]
    #[doc = " [Ln 1991, Col 5]"]
    #[doc = " [Ln 1992, Col 5]"]
    #[doc = " [Ln 1993, Col 5]"]
    #[doc = " [Ln 1994, Col 5]"]
    #[doc = " [Ln 1995, Col 5]"]
    #[doc = " [Ln 1996, Col 5]"]
    pub cx: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2000, Col 5]"]
    #[doc = " [Ln 2001, Col 5]"]
    pub cy: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2005, Col 5]"]
    #[doc = " [Ln 2006, Col 5]"]
    #[doc = " [Ln 2007, Col 5]"]
    #[doc = " [Ln 2008, Col 5]"]
    #[doc = " [Ln 2009, Col 5]"]
    #[doc = " [Ln 2010, Col 5]"]
    #[doc = " [Ln 2011, Col 5]"]
    #[doc = " [Ln 2012, Col 5]"]
    pub r: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2016, Col 5]"]
    #[doc = " [Ln 2017, Col 5]"]
    #[doc = " [Ln 2018, Col 5]"]
    #[doc = " [Ln 2019, Col 5]"]
    #[doc = " [Ln 2020, Col 5]"]
    #[doc = " [Ln 2021, Col 5]"]
    #[doc = " [Ln 2022, Col 5]"]
    #[doc = " [Ln 2023, Col 5]"]
    pub fx: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2027, Col 5]"]
    #[doc = " [Ln 2028, Col 5]"]
    #[doc = " [Ln 2029, Col 5]"]
    #[doc = " [Ln 2030, Col 5]"]
    #[doc = " [Ln 2031, Col 5]"]
    #[doc = " [Ln 2032, Col 5]"]
    #[doc = " [Ln 2033, Col 5]"]
    pub fy: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2037, Col 5]"]
    pub spread: Option<variable::Variable<SpreadMethod>>,
}
#[doc = " [Ln 2042, Col 1]"]
#[doc = " [Ln 2043, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GradientStop {
    #[doc = " [Ln 2045, Col 5]"]
    #[doc = " [Ln 2046, Col 5]"]
    #[doc = " [Ln 2047, Col 5]"]
    #[doc = " [Ln 2048, Col 5]"]
    #[doc = " [Ln 2049, Col 5]"]
    #[doc = " [Ln 2050, Col 5]"]
    #[doc = " [Ln 2051, Col 5]"]
    pub offset: variable::Variable<f32>,
    #[doc = " [Ln 2055, Col 5]"]
    pub color: Option<variable::Variable<Rgb>>,
    #[doc = " [Ln 2059, Col 5]"]
    pub opacity: Option<variable::Variable<f32>>,
}
#[doc = " [Ln 2064, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Group;
#[doc = " [Ln 2068, Col 1]"]
#[doc = " [Ln 2069, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Path {
    #[doc = " [Ln 2072, Col 5]"]
    pub events: variable::Variable<Vec<PathEvent>>,
    #[doc = " [Ln 2076, Col 5]"]
    #[doc = " [Ln 2077, Col 5]"]
    #[doc = " [Ln 2078, Col 5]"]
    #[doc = " [Ln 2079, Col 5]"]
    #[doc = " [Ln 2080, Col 5]"]
    #[doc = " [Ln 2081, Col 5]"]
    #[doc = " [Ln 2082, Col 5]"]
    #[doc = " [Ln 2083, Col 5]"]
    #[doc = " [Ln 2084, Col 5]"]
    pub length: variable::Variable<Length>,
}
#[doc = " [Ln 2089, Col 1]"]
#[doc = " [Ln 2090, Col 1]"]
#[doc = " [Ln 2091, Col 1]"]
#[doc = " [Ln 2092, Col 1]"]
#[doc = " [Ln 2093, Col 1]"]
#[doc = " [Ln 2094, Col 1]"]
#[doc = " [Ln 2095, Col 1]"]
#[doc = " [Ln 2096, Col 1]"]
#[doc = " [Ln 2097, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pattern {
    #[doc = " [Ln 2099, Col 5]"]
    #[doc = " [Ln 2100, Col 5]"]
    #[doc = " [Ln 2101, Col 5]"]
    #[doc = " [Ln 2102, Col 5]"]
    #[doc = " [Ln 2103, Col 5]"]
    #[doc = " [Ln 2104, Col 5]"]
    #[doc = " [Ln 2105, Col 5]"]
    #[doc = " [Ln 2106, Col 5]"]
    #[doc = " [Ln 2107, Col 5]"]
    #[doc = " [Ln 2108, Col 5]"]
    #[doc = " [Ln 2109, Col 5]"]
    #[doc = " [Ln 2110, Col 5]"]
    pub units: Option<variable::Variable<Coords>>,
    #[doc = " [Ln 2113, Col 5]"]
    #[doc = " [Ln 2114, Col 5]"]
    #[doc = " [Ln 2115, Col 5]"]
    #[doc = " [Ln 2116, Col 5]"]
    #[doc = " [Ln 2117, Col 5]"]
    #[doc = " [Ln 2118, Col 5]"]
    #[doc = " [Ln 2119, Col 5]"]
    #[doc = " [Ln 2120, Col 5]"]
    #[doc = " [Ln 2121, Col 5]"]
    #[doc = " [Ln 2122, Col 5]"]
    #[doc = " [Ln 2123, Col 5]"]
    #[doc = " [Ln 2124, Col 5]"]
    #[doc = " [Ln 2125, Col 5]"]
    #[doc = " [Ln 2126, Col 5]"]
    #[doc = " [Ln 2127, Col 5]"]
    pub content_units: Option<variable::Variable<Coords>>,
    #[doc = " [Ln 2131, Col 5]"]
    #[doc = " [Ln 2132, Col 5]"]
    #[doc = " [Ln 2133, Col 5]"]
    #[doc = " [Ln 2134, Col 5]"]
    #[doc = " [Ln 2135, Col 5]"]
    #[doc = " [Ln 2136, Col 5]"]
    #[doc = " [Ln 2137, Col 5]"]
    pub transform: Option<variable::Variable<Transform>>,
    #[doc = " [Ln 2141, Col 5]"]
    #[doc = " [Ln 2142, Col 5]"]
    #[doc = " [Ln 2143, Col 5]"]
    #[doc = " [Ln 2144, Col 5]"]
    #[doc = " [Ln 2145, Col 5]"]
    #[doc = " [Ln 2146, Col 5]"]
    #[doc = " [Ln 2147, Col 5]"]
    pub x: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2151, Col 5]"]
    #[doc = " [Ln 2152, Col 5]"]
    #[doc = " [Ln 2153, Col 5]"]
    #[doc = " [Ln 2154, Col 5]"]
    #[doc = " [Ln 2155, Col 5]"]
    pub y: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2159, Col 5]"]
    #[doc = " [Ln 2160, Col 5]"]
    #[doc = " [Ln 2161, Col 5]"]
    #[doc = " [Ln 2162, Col 5]"]
    #[doc = " [Ln 2163, Col 5]"]
    pub width: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2167, Col 5]"]
    #[doc = " [Ln 2168, Col 5]"]
    #[doc = " [Ln 2169, Col 5]"]
    #[doc = " [Ln 2170, Col 5]"]
    #[doc = " [Ln 2171, Col 5]"]
    pub height: Option<variable::Variable<Length>>,
}
#[doc = " [Ln 2176, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Use(pub variable::Variable<Iri>);
#[doc = " [Ln 2180, Col 1]"]
#[doc = " [Ln 2181, Col 1]"]
#[doc = " [Ln 2182, Col 1]"]
#[doc = " [Ln 2183, Col 1]"]
#[doc = " [Ln 2184, Col 1]"]
#[doc = " [Ln 2185, Col 1]"]
#[doc = " [Ln 2186, Col 1]"]
#[doc = " [Ln 2187, Col 1]"]
#[doc = " [Ln 2188, Col 1]"]
#[doc = " [Ln 2189, Col 1]"]
#[doc = " [Ln 2190, Col 1]"]
#[doc = " [Ln 2191, Col 1]"]
#[doc = " [Ln 2192, Col 1]"]
#[doc = " [Ln 2193, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    #[doc = " [Ln 2195, Col 5]"]
    #[doc = " [Ln 2196, Col 5]"]
    #[doc = " [Ln 2197, Col 5]"]
    #[doc = " [Ln 2198, Col 5]"]
    pub x: variable::Variable<Length>,
    #[doc = " [Ln 2202, Col 5]"]
    #[doc = " [Ln 2203, Col 5]"]
    #[doc = " [Ln 2204, Col 5]"]
    #[doc = " [Ln 2205, Col 5]"]
    pub y: variable::Variable<Length>,
    #[doc = " [Ln 2209, Col 5]"]
    #[doc = " [Ln 2210, Col 5]"]
    #[doc = " [Ln 2211, Col 5]"]
    #[doc = " [Ln 2212, Col 5]"]
    pub width: variable::Variable<Length>,
    #[doc = " [Ln 2216, Col 5]"]
    #[doc = " [Ln 2217, Col 5]"]
    #[doc = " [Ln 2218, Col 5]"]
    #[doc = " [Ln 2219, Col 5]"]
    pub height: variable::Variable<Length>,
    #[doc = " [Ln 2223, Col 5]"]
    #[doc = " [Ln 2224, Col 5]"]
    #[doc = " [Ln 2225, Col 5]"]
    #[doc = " [Ln 2226, Col 5]"]
    pub rx: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2230, Col 5]"]
    #[doc = " [Ln 2231, Col 5]"]
    #[doc = " [Ln 2232, Col 5]"]
    #[doc = " [Ln 2233, Col 5]"]
    pub ry: Option<variable::Variable<Length>>,
}
#[doc = " [Ln 2239, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle {
    #[doc = " [Ln 2241, Col 5]"]
    #[doc = " [Ln 2242, Col 5]"]
    #[doc = " [Ln 2243, Col 5]"]
    #[doc = " [Ln 2244, Col 5]"]
    pub cx: variable::Variable<Length>,
    #[doc = " [Ln 2248, Col 5]"]
    #[doc = " [Ln 2249, Col 5]"]
    #[doc = " [Ln 2250, Col 5]"]
    #[doc = " [Ln 2251, Col 5]"]
    pub cy: variable::Variable<Length>,
    #[doc = " [Ln 2255, Col 5]"]
    #[doc = " [Ln 2256, Col 5]"]
    #[doc = " [Ln 2257, Col 5]"]
    #[doc = " [Ln 2258, Col 5]"]
    pub r: variable::Variable<Length>,
}
#[doc = " [Ln 2263, Col 1]"]
#[doc = " [Ln 2264, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ellipse {
    #[doc = " [Ln 2266, Col 5]"]
    #[doc = " [Ln 2267, Col 5]"]
    #[doc = " [Ln 2268, Col 5]"]
    #[doc = " [Ln 2269, Col 5]"]
    pub cx: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2273, Col 5]"]
    #[doc = " [Ln 2274, Col 5]"]
    #[doc = " [Ln 2275, Col 5]"]
    #[doc = " [Ln 2276, Col 5]"]
    pub cy: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2280, Col 5]"]
    #[doc = " [Ln 2281, Col 5]"]
    #[doc = " [Ln 2282, Col 5]"]
    #[doc = " [Ln 2283, Col 5]"]
    pub rx: variable::Variable<Length>,
    #[doc = " [Ln 2287, Col 5]"]
    #[doc = " [Ln 2288, Col 5]"]
    #[doc = " [Ln 2289, Col 5]"]
    #[doc = " [Ln 2290, Col 5]"]
    pub ry: variable::Variable<Length>,
}
#[doc = " [Ln 2295, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line {
    #[doc = " [Ln 2297, Col 5]"]
    #[doc = " [Ln 2298, Col 5]"]
    #[doc = " [Ln 2299, Col 5]"]
    #[doc = " [Ln 2300, Col 5]"]
    #[doc = " [Ln 2301, Col 5]"]
    pub x1: variable::Variable<Length>,
    #[doc = " [Ln 2305, Col 5]"]
    #[doc = " [Ln 2306, Col 5]"]
    #[doc = " [Ln 2307, Col 5]"]
    #[doc = " [Ln 2308, Col 5]"]
    #[doc = " [Ln 2309, Col 5]"]
    pub y1: variable::Variable<Length>,
    #[doc = " [Ln 2313, Col 5]"]
    #[doc = " [Ln 2314, Col 5]"]
    #[doc = " [Ln 2315, Col 5]"]
    #[doc = " [Ln 2316, Col 5]"]
    #[doc = " [Ln 2317, Col 5]"]
    pub x2: variable::Variable<Length>,
    #[doc = " [Ln 2321, Col 5]"]
    #[doc = " [Ln 2322, Col 5]"]
    #[doc = " [Ln 2323, Col 5]"]
    #[doc = " [Ln 2324, Col 5]"]
    #[doc = " [Ln 2325, Col 5]"]
    pub y2: variable::Variable<Length>,
}
#[doc = " [Ln 2330, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polyline(
    #[doc = " [Ln 2332, Col 5]"]
    #[doc = " [Ln 2333, Col 5]"]
    #[doc = " [Ln 2334, Col 5]"]
    pub variable::Variable<Vec<Point>>,
);
#[doc = " [Ln 2339, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polygon(
    #[doc = " [Ln 2341, Col 5]"]
    #[doc = " [Ln 2342, Col 5]"]
    #[doc = " [Ln 2343, Col 5]"]
    pub variable::Variable<Vec<Point>>,
);
#[doc = " [Ln 2436, Col 1]"]
#[doc = " [Ln 2437, Col 1]"]
#[doc = " [Ln 2438, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text {
    #[doc = " [Ln 2354, Col 5]"]
    #[doc = " [Ln 2355, Col 5]"]
    #[doc = " [Ln 2356, Col 5]"]
    #[doc = " [Ln 2357, Col 5]"]
    #[doc = " [Ln 2358, Col 5]"]
    #[doc = " [Ln 2359, Col 5]"]
    #[doc = " [Ln 2360, Col 5]"]
    #[doc = " [Ln 2361, Col 5]"]
    #[doc = " [Ln 2362, Col 5]"]
    #[doc = " [Ln 2363, Col 5]"]
    #[doc = " [Ln 2364, Col 5]"]
    pub x: Option<variable::Variable<Vec<Length>>>,
    #[doc = " [Ln 2368, Col 5]"]
    #[doc = " [Ln 2369, Col 5]"]
    #[doc = " [Ln 2370, Col 5]"]
    #[doc = " [Ln 2371, Col 5]"]
    pub y: Option<variable::Variable<Vec<Length>>>,
    #[doc = " [Ln 2375, Col 5]"]
    #[doc = " [Ln 2376, Col 5]"]
    #[doc = " [Ln 2377, Col 5]"]
    #[doc = " [Ln 2378, Col 5]"]
    #[doc = " [Ln 2379, Col 5]"]
    #[doc = " [Ln 2380, Col 5]"]
    pub dx: Option<variable::Variable<Vec<Length>>>,
    #[doc = " [Ln 2384, Col 5]"]
    #[doc = " [Ln 2385, Col 5]"]
    #[doc = " [Ln 2386, Col 5]"]
    #[doc = " [Ln 2387, Col 5]"]
    #[doc = " [Ln 2388, Col 5]"]
    #[doc = " [Ln 2389, Col 5]"]
    pub dy: Option<variable::Variable<Vec<Length>>>,
    #[doc = " [Ln 2393, Col 5]"]
    #[doc = " [Ln 2394, Col 5]"]
    #[doc = " [Ln 2395, Col 5]"]
    #[doc = " [Ln 2396, Col 5]"]
    #[doc = " [Ln 2397, Col 5]"]
    #[doc = " [Ln 2398, Col 5]"]
    pub rotate: Option<variable::Variable<Vec<Angle>>>,
    #[doc = " [Ln 2402, Col 5]"]
    #[doc = " [Ln 2403, Col 5]"]
    #[doc = " [Ln 2404, Col 5]"]
    #[doc = " [Ln 2405, Col 5]"]
    #[doc = " [Ln 2406, Col 5]"]
    #[doc = " [Ln 2407, Col 5]"]
    #[doc = " [Ln 2408, Col 5]"]
    #[doc = " [Ln 2409, Col 5]"]
    #[doc = " [Ln 2410, Col 5]"]
    #[doc = " [Ln 2411, Col 5]"]
    #[doc = " [Ln 2412, Col 5]"]
    #[doc = " [Ln 2413, Col 5]"]
    #[doc = " [Ln 2414, Col 5]"]
    #[doc = " [Ln 2415, Col 5]"]
    #[doc = " [Ln 2416, Col 5]"]
    pub text_length: Option<variable::Variable<Vec<Length>>>,
    #[doc = " [Ln 2420, Col 5]"]
    #[doc = " [Ln 2421, Col 5]"]
    #[doc = " [Ln 2422, Col 5]"]
    #[doc = " [Ln 2423, Col 5]"]
    #[doc = " [Ln 2424, Col 5]"]
    #[doc = " [Ln 2425, Col 5]"]
    #[doc = " [Ln 2426, Col 5]"]
    #[doc = " [Ln 2427, Col 5]"]
    #[doc = " [Ln 2428, Col 5]"]
    #[doc = " [Ln 2429, Col 5]"]
    #[doc = " [Ln 2430, Col 5]"]
    #[doc = " [Ln 2431, Col 5]"]
    pub length_adjust: Option<variable::Variable<TextLengthAdjust>>,
}
#[doc = " [Ln 2441, Col 1]"]
#[doc = " [Ln 2442, Col 1]"]
#[doc = " [Ln 2443, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextSpan {
    #[doc = " [Ln 2354, Col 5]"]
    #[doc = " [Ln 2355, Col 5]"]
    #[doc = " [Ln 2356, Col 5]"]
    #[doc = " [Ln 2357, Col 5]"]
    #[doc = " [Ln 2358, Col 5]"]
    #[doc = " [Ln 2359, Col 5]"]
    #[doc = " [Ln 2360, Col 5]"]
    #[doc = " [Ln 2361, Col 5]"]
    #[doc = " [Ln 2362, Col 5]"]
    #[doc = " [Ln 2363, Col 5]"]
    #[doc = " [Ln 2364, Col 5]"]
    pub x: Option<variable::Variable<Vec<Length>>>,
    #[doc = " [Ln 2368, Col 5]"]
    #[doc = " [Ln 2369, Col 5]"]
    #[doc = " [Ln 2370, Col 5]"]
    #[doc = " [Ln 2371, Col 5]"]
    pub y: Option<variable::Variable<Vec<Length>>>,
    #[doc = " [Ln 2375, Col 5]"]
    #[doc = " [Ln 2376, Col 5]"]
    #[doc = " [Ln 2377, Col 5]"]
    #[doc = " [Ln 2378, Col 5]"]
    #[doc = " [Ln 2379, Col 5]"]
    #[doc = " [Ln 2380, Col 5]"]
    pub dx: Option<variable::Variable<Vec<Length>>>,
    #[doc = " [Ln 2384, Col 5]"]
    #[doc = " [Ln 2385, Col 5]"]
    #[doc = " [Ln 2386, Col 5]"]
    #[doc = " [Ln 2387, Col 5]"]
    #[doc = " [Ln 2388, Col 5]"]
    #[doc = " [Ln 2389, Col 5]"]
    pub dy: Option<variable::Variable<Vec<Length>>>,
    #[doc = " [Ln 2393, Col 5]"]
    #[doc = " [Ln 2394, Col 5]"]
    #[doc = " [Ln 2395, Col 5]"]
    #[doc = " [Ln 2396, Col 5]"]
    #[doc = " [Ln 2397, Col 5]"]
    #[doc = " [Ln 2398, Col 5]"]
    pub rotate: Option<variable::Variable<Vec<Angle>>>,
    #[doc = " [Ln 2402, Col 5]"]
    #[doc = " [Ln 2403, Col 5]"]
    #[doc = " [Ln 2404, Col 5]"]
    #[doc = " [Ln 2405, Col 5]"]
    #[doc = " [Ln 2406, Col 5]"]
    #[doc = " [Ln 2407, Col 5]"]
    #[doc = " [Ln 2408, Col 5]"]
    #[doc = " [Ln 2409, Col 5]"]
    #[doc = " [Ln 2410, Col 5]"]
    #[doc = " [Ln 2411, Col 5]"]
    #[doc = " [Ln 2412, Col 5]"]
    #[doc = " [Ln 2413, Col 5]"]
    #[doc = " [Ln 2414, Col 5]"]
    #[doc = " [Ln 2415, Col 5]"]
    #[doc = " [Ln 2416, Col 5]"]
    pub text_length: Option<variable::Variable<Vec<Length>>>,
    #[doc = " [Ln 2420, Col 5]"]
    #[doc = " [Ln 2421, Col 5]"]
    #[doc = " [Ln 2422, Col 5]"]
    #[doc = " [Ln 2423, Col 5]"]
    #[doc = " [Ln 2424, Col 5]"]
    #[doc = " [Ln 2425, Col 5]"]
    #[doc = " [Ln 2426, Col 5]"]
    #[doc = " [Ln 2427, Col 5]"]
    #[doc = " [Ln 2428, Col 5]"]
    #[doc = " [Ln 2429, Col 5]"]
    #[doc = " [Ln 2430, Col 5]"]
    #[doc = " [Ln 2431, Col 5]"]
    pub length_adjust: Option<variable::Variable<TextLengthAdjust>>,
}
#[doc = " [Ln 2447, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Characters(pub String);
#[doc = " [Ln 2450, Col 1]"]
#[doc = " [Ln 2451, Col 1]"]
#[doc = " [Ln 2452, Col 1]"]
#[doc = " [Ln 2453, Col 1]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextPath {
    #[doc = " [Ln 2455, Col 5]"]
    #[doc = " [Ln 2456, Col 5]"]
    #[doc = " [Ln 2457, Col 5]"]
    #[doc = " [Ln 2458, Col 5]"]
    #[doc = " [Ln 2459, Col 5]"]
    #[doc = " [Ln 2460, Col 5]"]
    #[doc = " [Ln 2461, Col 5]"]
    #[doc = " [Ln 2462, Col 5]"]
    #[doc = " [Ln 2463, Col 5]"]
    #[doc = " [Ln 2464, Col 5]"]
    #[doc = " [Ln 2465, Col 5]"]
    #[doc = " [Ln 2466, Col 5]"]
    #[doc = " [Ln 2467, Col 5]"]
    pub start_offset: Option<variable::Variable<Length>>,
    #[doc = " [Ln 2471, Col 5]"]
    pub method: Option<variable::Variable<TextPathMethod>>,
    #[doc = " [Ln 2475, Col 5]"]
    pub spacing: Option<variable::Variable<TextPathSpacing>>,
    #[doc = " [Ln 2479, Col 5]"]
    #[doc = " [Ln 2480, Col 5]"]
    #[doc = " [Ln 2481, Col 5]"]
    pub href: variable::Variable<Iri>,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Data {
    Bool(bool),
    ListOfBool(Box<Vec<bool>>),
    String(String),
    ListOfString(Box<Vec<String>>),
    Byte(i8),
    ListOfByte(Box<Vec<i8>>),
    Ubyte(u8),
    ListOfUbyte(Box<Vec<u8>>),
    Short(i16),
    ListOfShort(Box<Vec<i16>>),
    Ushort(u16),
    ListOfUshort(Box<Vec<u16>>),
    Int(i32),
    ListOfInt(Box<Vec<i32>>),
    Uint(u32),
    ListOfUint(Box<Vec<u32>>),
    Long(i64),
    ListOfLong(Box<Vec<i64>>),
    Ulong(u64),
    ListOfUlong(Box<Vec<u64>>),
    Float(f32),
    ListOfFloat(Box<Vec<f32>>),
    Double(f64),
    ListOfDouble(Box<Vec<f64>>),
    Angle(Box<Angle>),
    ListOfAngle(Box<Vec<Angle>>),
    Length(Box<Length>),
    ListOfLength(Box<Vec<Length>>),
    Color(Box<Color>),
    ListOfColor(Box<Vec<Color>>),
    Rgb(Box<Rgb>),
    ListOfRgb(Box<Vec<Rgb>>),
    Iri(Box<Iri>),
    ListOfIri(Box<Vec<Iri>>),
    FuncIri(Box<FuncIri>),
    ListOfFuncIri(Box<Vec<FuncIri>>),
    Point(Box<Point>),
    ListOfPoint(Box<Vec<Point>>),
    Percent(Box<Percent>),
    ListOfPercent(Box<Vec<Percent>>),
    Paint(Box<Paint>),
    ListOfPaint(Box<Vec<Paint>>),
    NumberOptNumber(Box<NumberOptNumber>),
    ListOfNumberOptNumber(Box<Vec<NumberOptNumber>>),
    Coords(Box<Coords>),
    ListOfCoords(Box<Vec<Coords>>),
    Transform(Box<Transform>),
    ListOfTransform(Box<Vec<Transform>>),
    Channel(Box<Channel>),
    ListOfChannel(Box<Vec<Channel>>),
    ClipRule(Box<ClipRule>),
    ListOfClipRule(Box<Vec<ClipRule>>),
    PathEvent(Box<PathEvent>),
    ListOfPathEvent(Box<Vec<PathEvent>>),
    FillRule(Box<FillRule>),
    ListOfFillRule(Box<Vec<FillRule>>),
    StrokeLineCap(Box<StrokeLineCap>),
    ListOfStrokeLineCap(Box<Vec<StrokeLineCap>>),
    StrokeLineJoin(Box<StrokeLineJoin>),
    ListOfStrokeLineJoin(Box<Vec<StrokeLineJoin>>),
    SpreadMethod(Box<SpreadMethod>),
    ListOfSpreadMethod(Box<Vec<SpreadMethod>>),
    FontStyle(Box<FontStyle>),
    ListOfFontStyle(Box<Vec<FontStyle>>),
    FontVariant(Box<FontVariant>),
    ListOfFontVariant(Box<Vec<FontVariant>>),
    FontWeight(Box<FontWeight>),
    ListOfFontWeight(Box<Vec<FontWeight>>),
    FontFamily(Box<FontFamily>),
    ListOfFontFamily(Box<Vec<FontFamily>>),
    FontStretch(Box<FontStretch>),
    ListOfFontStretch(Box<Vec<FontStretch>>),
    Background(Box<Background>),
    ListOfBackground(Box<Vec<Background>>),
    FeIn(Box<FeIn>),
    ListOfFeIn(Box<Vec<FeIn>>),
    FeOut(Box<FeOut>),
    ListOfFeOut(Box<Vec<FeOut>>),
    FeBlendMode(Box<FeBlendMode>),
    ListOfFeBlendMode(Box<Vec<FeBlendMode>>),
    TextLengthAdjust(Box<TextLengthAdjust>),
    ListOfTextLengthAdjust(Box<Vec<TextLengthAdjust>>),
    WritingMode(Box<WritingMode>),
    ListOfWritingMode(Box<Vec<WritingMode>>),
    TextDirection(Box<TextDirection>),
    ListOfTextDirection(Box<Vec<TextDirection>>),
    UnicodeBidi(Box<UnicodeBidi>),
    ListOfUnicodeBidi(Box<Vec<UnicodeBidi>>),
    TextAnchor(Box<TextAnchor>),
    ListOfTextAnchor(Box<Vec<TextAnchor>>),
    DominantBaseline(Box<DominantBaseline>),
    ListOfDominantBaseline(Box<Vec<DominantBaseline>>),
    AlignmentBaseline(Box<AlignmentBaseline>),
    ListOfAlignmentBaseline(Box<Vec<AlignmentBaseline>>),
    BaselineShift(Box<BaselineShift>),
    ListOfBaselineShift(Box<Vec<BaselineShift>>),
    TextDecoration(Box<TextDecoration>),
    ListOfTextDecoration(Box<Vec<TextDecoration>>),
    TextPathMethod(Box<TextPathMethod>),
    ListOfTextPathMethod(Box<Vec<TextPathMethod>>),
    TextPathSpacing(Box<TextPathSpacing>),
    ListOfTextPathSpacing(Box<Vec<TextPathSpacing>>),
    LetterSpacing(Box<LetterSpacing>),
    ListOfLetterSpacing(Box<Vec<LetterSpacing>>),
    WordSpacing(Box<WordSpacing>),
    ListOfWordSpacing(Box<Vec<WordSpacing>>),
    MeetOrSlice(Box<MeetOrSlice>),
    ListOfMeetOrSlice(Box<Vec<MeetOrSlice>>),
    PreserveAspectRatio(Box<PreserveAspectRatio>),
    ListOfPreserveAspectRatio(Box<Vec<PreserveAspectRatio>>),
    FeColorMatrixValues(Box<FeColorMatrixValues>),
    ListOfFeColorMatrixValues(Box<Vec<FeColorMatrixValues>>),
    FeFunc(Box<FeFunc>),
    ListOfFeFunc(Box<Vec<FeFunc>>),
    FeCompositeOperator(Box<FeCompositeOperator>),
    ListOfFeCompositeOperator(Box<Vec<FeCompositeOperator>>),
    FeConvolveMatrixEdgeMode(Box<FeConvolveMatrixEdgeMode>),
    ListOfFeConvolveMatrixEdgeMode(Box<Vec<FeConvolveMatrixEdgeMode>>),
    FeMorphologyOperator(Box<FeMorphologyOperator>),
    ListOfFeMorphologyOperator(Box<Vec<FeMorphologyOperator>>),
    FeStitchTiles(Box<FeStitchTiles>),
    ListOfFeStitchTiles(Box<Vec<FeStitchTiles>>),
    FeTurbulenceType(Box<FeTurbulenceType>),
    ListOfFeTurbulenceType(Box<Vec<FeTurbulenceType>>),
}
impl From<bool> for Data {
    fn from(value: bool) -> Self {
        Data::Bool(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a bool {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Bool(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<bool>> for Data {
    fn from(value: Vec<bool>) -> Self {
        Data::ListOfBool(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<bool> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfBool(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<String> for Data {
    fn from(value: String) -> Self {
        Data::String(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a String {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::String(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<String>> for Data {
    fn from(value: Vec<String>) -> Self {
        Data::ListOfString(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<String> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfString(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<i8> for Data {
    fn from(value: i8) -> Self {
        Data::Byte(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a i8 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Byte(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<i8>> for Data {
    fn from(value: Vec<i8>) -> Self {
        Data::ListOfByte(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<i8> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfByte(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<u8> for Data {
    fn from(value: u8) -> Self {
        Data::Ubyte(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a u8 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Ubyte(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<u8>> for Data {
    fn from(value: Vec<u8>) -> Self {
        Data::ListOfUbyte(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<u8> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfUbyte(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<i16> for Data {
    fn from(value: i16) -> Self {
        Data::Short(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a i16 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Short(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<i16>> for Data {
    fn from(value: Vec<i16>) -> Self {
        Data::ListOfShort(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<i16> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfShort(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<u16> for Data {
    fn from(value: u16) -> Self {
        Data::Ushort(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a u16 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Ushort(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<u16>> for Data {
    fn from(value: Vec<u16>) -> Self {
        Data::ListOfUshort(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<u16> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfUshort(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<i32> for Data {
    fn from(value: i32) -> Self {
        Data::Int(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a i32 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Int(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<i32>> for Data {
    fn from(value: Vec<i32>) -> Self {
        Data::ListOfInt(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<i32> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfInt(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<u32> for Data {
    fn from(value: u32) -> Self {
        Data::Uint(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a u32 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Uint(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<u32>> for Data {
    fn from(value: Vec<u32>) -> Self {
        Data::ListOfUint(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<u32> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfUint(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<i64> for Data {
    fn from(value: i64) -> Self {
        Data::Long(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a i64 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Long(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<i64>> for Data {
    fn from(value: Vec<i64>) -> Self {
        Data::ListOfLong(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<i64> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfLong(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<u64> for Data {
    fn from(value: u64) -> Self {
        Data::Ulong(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a u64 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Ulong(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<u64>> for Data {
    fn from(value: Vec<u64>) -> Self {
        Data::ListOfUlong(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<u64> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfUlong(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<f32> for Data {
    fn from(value: f32) -> Self {
        Data::Float(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a f32 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Float(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<f32>> for Data {
    fn from(value: Vec<f32>) -> Self {
        Data::ListOfFloat(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<f32> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFloat(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<f64> for Data {
    fn from(value: f64) -> Self {
        Data::Double(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a f64 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Double(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<f64>> for Data {
    fn from(value: Vec<f64>) -> Self {
        Data::ListOfDouble(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<f64> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfDouble(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Angle> for Data {
    fn from(value: Angle) -> Self {
        Data::Angle(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Angle {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Angle(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Angle>> for Data {
    fn from(value: Vec<Angle>) -> Self {
        Data::ListOfAngle(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Angle> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfAngle(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Length> for Data {
    fn from(value: Length) -> Self {
        Data::Length(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Length {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Length(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Length>> for Data {
    fn from(value: Vec<Length>) -> Self {
        Data::ListOfLength(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Length> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfLength(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Color> for Data {
    fn from(value: Color) -> Self {
        Data::Color(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Color {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Color(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Color>> for Data {
    fn from(value: Vec<Color>) -> Self {
        Data::ListOfColor(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Color> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfColor(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Rgb> for Data {
    fn from(value: Rgb) -> Self {
        Data::Rgb(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Rgb {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Rgb(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Rgb>> for Data {
    fn from(value: Vec<Rgb>) -> Self {
        Data::ListOfRgb(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Rgb> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfRgb(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Iri> for Data {
    fn from(value: Iri) -> Self {
        Data::Iri(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Iri {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Iri(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Iri>> for Data {
    fn from(value: Vec<Iri>) -> Self {
        Data::ListOfIri(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Iri> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfIri(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FuncIri> for Data {
    fn from(value: FuncIri) -> Self {
        Data::FuncIri(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FuncIri {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FuncIri(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FuncIri>> for Data {
    fn from(value: Vec<FuncIri>) -> Self {
        Data::ListOfFuncIri(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FuncIri> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFuncIri(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Point> for Data {
    fn from(value: Point) -> Self {
        Data::Point(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Point {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Point(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Point>> for Data {
    fn from(value: Vec<Point>) -> Self {
        Data::ListOfPoint(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Point> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfPoint(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Percent> for Data {
    fn from(value: Percent) -> Self {
        Data::Percent(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Percent {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Percent(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Percent>> for Data {
    fn from(value: Vec<Percent>) -> Self {
        Data::ListOfPercent(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Percent> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfPercent(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Paint> for Data {
    fn from(value: Paint) -> Self {
        Data::Paint(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Paint {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Paint(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Paint>> for Data {
    fn from(value: Vec<Paint>) -> Self {
        Data::ListOfPaint(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Paint> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfPaint(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<NumberOptNumber> for Data {
    fn from(value: NumberOptNumber) -> Self {
        Data::NumberOptNumber(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a NumberOptNumber {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::NumberOptNumber(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<NumberOptNumber>> for Data {
    fn from(value: Vec<NumberOptNumber>) -> Self {
        Data::ListOfNumberOptNumber(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<NumberOptNumber> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfNumberOptNumber(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Coords> for Data {
    fn from(value: Coords) -> Self {
        Data::Coords(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Coords {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Coords(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Coords>> for Data {
    fn from(value: Vec<Coords>) -> Self {
        Data::ListOfCoords(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Coords> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfCoords(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Transform> for Data {
    fn from(value: Transform) -> Self {
        Data::Transform(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Transform {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Transform(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Transform>> for Data {
    fn from(value: Vec<Transform>) -> Self {
        Data::ListOfTransform(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Transform> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTransform(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Channel> for Data {
    fn from(value: Channel) -> Self {
        Data::Channel(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Channel {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Channel(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Channel>> for Data {
    fn from(value: Vec<Channel>) -> Self {
        Data::ListOfChannel(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Channel> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfChannel(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<ClipRule> for Data {
    fn from(value: ClipRule) -> Self {
        Data::ClipRule(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a ClipRule {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ClipRule(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<ClipRule>> for Data {
    fn from(value: Vec<ClipRule>) -> Self {
        Data::ListOfClipRule(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<ClipRule> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfClipRule(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<PathEvent> for Data {
    fn from(value: PathEvent) -> Self {
        Data::PathEvent(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a PathEvent {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::PathEvent(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<PathEvent>> for Data {
    fn from(value: Vec<PathEvent>) -> Self {
        Data::ListOfPathEvent(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<PathEvent> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfPathEvent(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FillRule> for Data {
    fn from(value: FillRule) -> Self {
        Data::FillRule(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FillRule {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FillRule(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FillRule>> for Data {
    fn from(value: Vec<FillRule>) -> Self {
        Data::ListOfFillRule(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FillRule> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFillRule(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<StrokeLineCap> for Data {
    fn from(value: StrokeLineCap) -> Self {
        Data::StrokeLineCap(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a StrokeLineCap {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::StrokeLineCap(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<StrokeLineCap>> for Data {
    fn from(value: Vec<StrokeLineCap>) -> Self {
        Data::ListOfStrokeLineCap(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<StrokeLineCap> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfStrokeLineCap(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<StrokeLineJoin> for Data {
    fn from(value: StrokeLineJoin) -> Self {
        Data::StrokeLineJoin(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a StrokeLineJoin {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::StrokeLineJoin(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<StrokeLineJoin>> for Data {
    fn from(value: Vec<StrokeLineJoin>) -> Self {
        Data::ListOfStrokeLineJoin(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<StrokeLineJoin> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfStrokeLineJoin(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<SpreadMethod> for Data {
    fn from(value: SpreadMethod) -> Self {
        Data::SpreadMethod(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a SpreadMethod {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::SpreadMethod(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<SpreadMethod>> for Data {
    fn from(value: Vec<SpreadMethod>) -> Self {
        Data::ListOfSpreadMethod(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<SpreadMethod> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfSpreadMethod(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FontStyle> for Data {
    fn from(value: FontStyle) -> Self {
        Data::FontStyle(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FontStyle {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FontStyle(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FontStyle>> for Data {
    fn from(value: Vec<FontStyle>) -> Self {
        Data::ListOfFontStyle(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FontStyle> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFontStyle(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FontVariant> for Data {
    fn from(value: FontVariant) -> Self {
        Data::FontVariant(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FontVariant {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FontVariant(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FontVariant>> for Data {
    fn from(value: Vec<FontVariant>) -> Self {
        Data::ListOfFontVariant(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FontVariant> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFontVariant(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FontWeight> for Data {
    fn from(value: FontWeight) -> Self {
        Data::FontWeight(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FontWeight {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FontWeight(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FontWeight>> for Data {
    fn from(value: Vec<FontWeight>) -> Self {
        Data::ListOfFontWeight(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FontWeight> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFontWeight(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FontFamily> for Data {
    fn from(value: FontFamily) -> Self {
        Data::FontFamily(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FontFamily {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FontFamily(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FontFamily>> for Data {
    fn from(value: Vec<FontFamily>) -> Self {
        Data::ListOfFontFamily(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FontFamily> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFontFamily(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FontStretch> for Data {
    fn from(value: FontStretch) -> Self {
        Data::FontStretch(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FontStretch {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FontStretch(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FontStretch>> for Data {
    fn from(value: Vec<FontStretch>) -> Self {
        Data::ListOfFontStretch(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FontStretch> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFontStretch(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Background> for Data {
    fn from(value: Background) -> Self {
        Data::Background(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Background {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Background(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Background>> for Data {
    fn from(value: Vec<Background>) -> Self {
        Data::ListOfBackground(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Background> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfBackground(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeIn> for Data {
    fn from(value: FeIn) -> Self {
        Data::FeIn(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeIn {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeIn(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeIn>> for Data {
    fn from(value: Vec<FeIn>) -> Self {
        Data::ListOfFeIn(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeIn> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeIn(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeOut> for Data {
    fn from(value: FeOut) -> Self {
        Data::FeOut(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeOut {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeOut(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeOut>> for Data {
    fn from(value: Vec<FeOut>) -> Self {
        Data::ListOfFeOut(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeOut> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeOut(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeBlendMode> for Data {
    fn from(value: FeBlendMode) -> Self {
        Data::FeBlendMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeBlendMode {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeBlendMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeBlendMode>> for Data {
    fn from(value: Vec<FeBlendMode>) -> Self {
        Data::ListOfFeBlendMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeBlendMode> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeBlendMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextLengthAdjust> for Data {
    fn from(value: TextLengthAdjust) -> Self {
        Data::TextLengthAdjust(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextLengthAdjust {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextLengthAdjust(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextLengthAdjust>> for Data {
    fn from(value: Vec<TextLengthAdjust>) -> Self {
        Data::ListOfTextLengthAdjust(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextLengthAdjust> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextLengthAdjust(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<WritingMode> for Data {
    fn from(value: WritingMode) -> Self {
        Data::WritingMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a WritingMode {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::WritingMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<WritingMode>> for Data {
    fn from(value: Vec<WritingMode>) -> Self {
        Data::ListOfWritingMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<WritingMode> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfWritingMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextDirection> for Data {
    fn from(value: TextDirection) -> Self {
        Data::TextDirection(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextDirection {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextDirection(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextDirection>> for Data {
    fn from(value: Vec<TextDirection>) -> Self {
        Data::ListOfTextDirection(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextDirection> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextDirection(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<UnicodeBidi> for Data {
    fn from(value: UnicodeBidi) -> Self {
        Data::UnicodeBidi(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a UnicodeBidi {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::UnicodeBidi(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<UnicodeBidi>> for Data {
    fn from(value: Vec<UnicodeBidi>) -> Self {
        Data::ListOfUnicodeBidi(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<UnicodeBidi> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfUnicodeBidi(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextAnchor> for Data {
    fn from(value: TextAnchor) -> Self {
        Data::TextAnchor(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextAnchor {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextAnchor(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextAnchor>> for Data {
    fn from(value: Vec<TextAnchor>) -> Self {
        Data::ListOfTextAnchor(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextAnchor> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextAnchor(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<DominantBaseline> for Data {
    fn from(value: DominantBaseline) -> Self {
        Data::DominantBaseline(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a DominantBaseline {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::DominantBaseline(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<DominantBaseline>> for Data {
    fn from(value: Vec<DominantBaseline>) -> Self {
        Data::ListOfDominantBaseline(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<DominantBaseline> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfDominantBaseline(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<AlignmentBaseline> for Data {
    fn from(value: AlignmentBaseline) -> Self {
        Data::AlignmentBaseline(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a AlignmentBaseline {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::AlignmentBaseline(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<AlignmentBaseline>> for Data {
    fn from(value: Vec<AlignmentBaseline>) -> Self {
        Data::ListOfAlignmentBaseline(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<AlignmentBaseline> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfAlignmentBaseline(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<BaselineShift> for Data {
    fn from(value: BaselineShift) -> Self {
        Data::BaselineShift(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a BaselineShift {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::BaselineShift(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<BaselineShift>> for Data {
    fn from(value: Vec<BaselineShift>) -> Self {
        Data::ListOfBaselineShift(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<BaselineShift> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfBaselineShift(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextDecoration> for Data {
    fn from(value: TextDecoration) -> Self {
        Data::TextDecoration(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextDecoration {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextDecoration(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextDecoration>> for Data {
    fn from(value: Vec<TextDecoration>) -> Self {
        Data::ListOfTextDecoration(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextDecoration> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextDecoration(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextPathMethod> for Data {
    fn from(value: TextPathMethod) -> Self {
        Data::TextPathMethod(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextPathMethod {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextPathMethod(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextPathMethod>> for Data {
    fn from(value: Vec<TextPathMethod>) -> Self {
        Data::ListOfTextPathMethod(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextPathMethod> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextPathMethod(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextPathSpacing> for Data {
    fn from(value: TextPathSpacing) -> Self {
        Data::TextPathSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextPathSpacing {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextPathSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextPathSpacing>> for Data {
    fn from(value: Vec<TextPathSpacing>) -> Self {
        Data::ListOfTextPathSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextPathSpacing> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextPathSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<LetterSpacing> for Data {
    fn from(value: LetterSpacing) -> Self {
        Data::LetterSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a LetterSpacing {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::LetterSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<LetterSpacing>> for Data {
    fn from(value: Vec<LetterSpacing>) -> Self {
        Data::ListOfLetterSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<LetterSpacing> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfLetterSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<WordSpacing> for Data {
    fn from(value: WordSpacing) -> Self {
        Data::WordSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a WordSpacing {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::WordSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<WordSpacing>> for Data {
    fn from(value: Vec<WordSpacing>) -> Self {
        Data::ListOfWordSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<WordSpacing> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfWordSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<MeetOrSlice> for Data {
    fn from(value: MeetOrSlice) -> Self {
        Data::MeetOrSlice(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a MeetOrSlice {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::MeetOrSlice(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<MeetOrSlice>> for Data {
    fn from(value: Vec<MeetOrSlice>) -> Self {
        Data::ListOfMeetOrSlice(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<MeetOrSlice> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfMeetOrSlice(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<PreserveAspectRatio> for Data {
    fn from(value: PreserveAspectRatio) -> Self {
        Data::PreserveAspectRatio(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a PreserveAspectRatio {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::PreserveAspectRatio(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<PreserveAspectRatio>> for Data {
    fn from(value: Vec<PreserveAspectRatio>) -> Self {
        Data::ListOfPreserveAspectRatio(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<PreserveAspectRatio> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfPreserveAspectRatio(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeColorMatrixValues> for Data {
    fn from(value: FeColorMatrixValues) -> Self {
        Data::FeColorMatrixValues(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeColorMatrixValues {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeColorMatrixValues(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeColorMatrixValues>> for Data {
    fn from(value: Vec<FeColorMatrixValues>) -> Self {
        Data::ListOfFeColorMatrixValues(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeColorMatrixValues> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeColorMatrixValues(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeFunc> for Data {
    fn from(value: FeFunc) -> Self {
        Data::FeFunc(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeFunc {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeFunc(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeFunc>> for Data {
    fn from(value: Vec<FeFunc>) -> Self {
        Data::ListOfFeFunc(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeFunc> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeFunc(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeCompositeOperator> for Data {
    fn from(value: FeCompositeOperator) -> Self {
        Data::FeCompositeOperator(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeCompositeOperator {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeCompositeOperator(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeCompositeOperator>> for Data {
    fn from(value: Vec<FeCompositeOperator>) -> Self {
        Data::ListOfFeCompositeOperator(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeCompositeOperator> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeCompositeOperator(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeConvolveMatrixEdgeMode> for Data {
    fn from(value: FeConvolveMatrixEdgeMode) -> Self {
        Data::FeConvolveMatrixEdgeMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeConvolveMatrixEdgeMode {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeConvolveMatrixEdgeMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeConvolveMatrixEdgeMode>> for Data {
    fn from(value: Vec<FeConvolveMatrixEdgeMode>) -> Self {
        Data::ListOfFeConvolveMatrixEdgeMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeConvolveMatrixEdgeMode> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeConvolveMatrixEdgeMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeMorphologyOperator> for Data {
    fn from(value: FeMorphologyOperator) -> Self {
        Data::FeMorphologyOperator(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeMorphologyOperator {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeMorphologyOperator(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeMorphologyOperator>> for Data {
    fn from(value: Vec<FeMorphologyOperator>) -> Self {
        Data::ListOfFeMorphologyOperator(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeMorphologyOperator> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeMorphologyOperator(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeStitchTiles> for Data {
    fn from(value: FeStitchTiles) -> Self {
        Data::FeStitchTiles(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeStitchTiles {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeStitchTiles(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeStitchTiles>> for Data {
    fn from(value: Vec<FeStitchTiles>) -> Self {
        Data::ListOfFeStitchTiles(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeStitchTiles> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeStitchTiles(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeTurbulenceType> for Data {
    fn from(value: FeTurbulenceType) -> Self {
        Data::FeTurbulenceType(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeTurbulenceType {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeTurbulenceType(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeTurbulenceType>> for Data {
    fn from(value: Vec<FeTurbulenceType>) -> Self {
        Data::ListOfFeTurbulenceType(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeTurbulenceType> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeTurbulenceType(v) => Ok(v),
            _ => Err(()),
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Attr {
    TextLayout(Box<TextLayout>),
    WithTransform(Box<WithTransform>),
    Id(Box<Id>),
    Fill(Box<Fill>),
    Stroke(Box<Stroke>),
    Font(Box<Font>),
    EnableBackground(Box<EnableBackground>),
    WithFilter(Box<WithFilter>),
    WithClipPath(Box<WithClipPath>),
    WithMask(Box<WithMask>),
    Opacity(Box<Opacity>),
    ViewBox(Box<ViewBox>),
}
impl From<TextLayout> for Attr {
    fn from(value: TextLayout) -> Self {
        Self::TextLayout(Box::new(value))
    }
}
impl From<WithTransform> for Attr {
    fn from(value: WithTransform) -> Self {
        Self::WithTransform(Box::new(value))
    }
}
impl From<Id> for Attr {
    fn from(value: Id) -> Self {
        Self::Id(Box::new(value))
    }
}
impl From<Fill> for Attr {
    fn from(value: Fill) -> Self {
        Self::Fill(Box::new(value))
    }
}
impl From<Stroke> for Attr {
    fn from(value: Stroke) -> Self {
        Self::Stroke(Box::new(value))
    }
}
impl From<Font> for Attr {
    fn from(value: Font) -> Self {
        Self::Font(Box::new(value))
    }
}
impl From<EnableBackground> for Attr {
    fn from(value: EnableBackground) -> Self {
        Self::EnableBackground(Box::new(value))
    }
}
impl From<WithFilter> for Attr {
    fn from(value: WithFilter) -> Self {
        Self::WithFilter(Box::new(value))
    }
}
impl From<WithClipPath> for Attr {
    fn from(value: WithClipPath) -> Self {
        Self::WithClipPath(Box::new(value))
    }
}
impl From<WithMask> for Attr {
    fn from(value: WithMask) -> Self {
        Self::WithMask(Box::new(value))
    }
}
impl From<Opacity> for Attr {
    fn from(value: Opacity) -> Self {
        Self::Opacity(Box::new(value))
    }
}
impl From<ViewBox> for Attr {
    fn from(value: ViewBox) -> Self {
        Self::ViewBox(Box::new(value))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Element {
    Canvas(Box<Canvas>),
    Mask(Box<Mask>),
    ClipPath(Box<ClipPath>),
    Filter(Box<Filter>),
    FeComponentTransfer(Box<FeComponentTransfer>),
    FeDiffuseLighting(Box<FeDiffuseLighting>),
    FeMerge(Box<FeMerge>),
    FeSpecularLighting(Box<FeSpecularLighting>),
    LinearGradient(Box<LinearGradient>),
    RadialGradient(Box<RadialGradient>),
    Group(Box<Group>),
    Pattern(Box<Pattern>),
    Text(Box<Text>),
    TextSpan(Box<TextSpan>),
    TextPath(Box<TextPath>),
}
impl From<Canvas> for Element {
    fn from(value: Canvas) -> Self {
        Self::Canvas(Box::new(value))
    }
}
impl From<Mask> for Element {
    fn from(value: Mask) -> Self {
        Self::Mask(Box::new(value))
    }
}
impl From<ClipPath> for Element {
    fn from(value: ClipPath) -> Self {
        Self::ClipPath(Box::new(value))
    }
}
impl From<Filter> for Element {
    fn from(value: Filter) -> Self {
        Self::Filter(Box::new(value))
    }
}
impl From<FeComponentTransfer> for Element {
    fn from(value: FeComponentTransfer) -> Self {
        Self::FeComponentTransfer(Box::new(value))
    }
}
impl From<FeDiffuseLighting> for Element {
    fn from(value: FeDiffuseLighting) -> Self {
        Self::FeDiffuseLighting(Box::new(value))
    }
}
impl From<FeMerge> for Element {
    fn from(value: FeMerge) -> Self {
        Self::FeMerge(Box::new(value))
    }
}
impl From<FeSpecularLighting> for Element {
    fn from(value: FeSpecularLighting) -> Self {
        Self::FeSpecularLighting(Box::new(value))
    }
}
impl From<LinearGradient> for Element {
    fn from(value: LinearGradient) -> Self {
        Self::LinearGradient(Box::new(value))
    }
}
impl From<RadialGradient> for Element {
    fn from(value: RadialGradient) -> Self {
        Self::RadialGradient(Box::new(value))
    }
}
impl From<Group> for Element {
    fn from(value: Group) -> Self {
        Self::Group(Box::new(value))
    }
}
impl From<Pattern> for Element {
    fn from(value: Pattern) -> Self {
        Self::Pattern(Box::new(value))
    }
}
impl From<Text> for Element {
    fn from(value: Text) -> Self {
        Self::Text(Box::new(value))
    }
}
impl From<TextSpan> for Element {
    fn from(value: TextSpan) -> Self {
        Self::TextSpan(Box::new(value))
    }
}
impl From<TextPath> for Element {
    fn from(value: TextPath) -> Self {
        Self::TextPath(Box::new(value))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Leaf {
    FeDistantLight(Box<FeDistantLight>),
    FePointLight(Box<FePointLight>),
    FeSpotLight(Box<FeSpotLight>),
    FeBlend(Box<FeBlend>),
    FeColorMatrix(Box<FeColorMatrix>),
    FeFuncA(Box<FeFuncA>),
    FeFuncR(Box<FeFuncR>),
    FeFuncG(Box<FeFuncG>),
    FeFuncB(Box<FeFuncB>),
    FeComposite(Box<FeComposite>),
    FeConvolveMatrix(Box<FeConvolveMatrix>),
    FeDisplacementMap(Box<FeDisplacementMap>),
    FeFlood(Box<FeFlood>),
    FeGaussianBlur(Box<FeGaussianBlur>),
    FeMergeNode(Box<FeMergeNode>),
    FeImage(Box<FeImage>),
    FeMorphology(Box<FeMorphology>),
    FeOffset(Box<FeOffset>),
    FeTile(Box<FeTile>),
    FeTurbulence(Box<FeTurbulence>),
    GradientStop(Box<GradientStop>),
    Path(Box<Path>),
    Use(Box<Use>),
    Rect(Box<Rect>),
    Circle(Box<Circle>),
    Ellipse(Box<Ellipse>),
    Line(Box<Line>),
    Polyline(Box<Polyline>),
    Polygon(Box<Polygon>),
    Characters(Box<Characters>),
}
impl From<FeDistantLight> for Leaf {
    fn from(value: FeDistantLight) -> Self {
        Self::FeDistantLight(Box::new(value))
    }
}
impl From<FePointLight> for Leaf {
    fn from(value: FePointLight) -> Self {
        Self::FePointLight(Box::new(value))
    }
}
impl From<FeSpotLight> for Leaf {
    fn from(value: FeSpotLight) -> Self {
        Self::FeSpotLight(Box::new(value))
    }
}
impl From<FeBlend> for Leaf {
    fn from(value: FeBlend) -> Self {
        Self::FeBlend(Box::new(value))
    }
}
impl From<FeColorMatrix> for Leaf {
    fn from(value: FeColorMatrix) -> Self {
        Self::FeColorMatrix(Box::new(value))
    }
}
impl From<FeFuncA> for Leaf {
    fn from(value: FeFuncA) -> Self {
        Self::FeFuncA(Box::new(value))
    }
}
impl From<FeFuncR> for Leaf {
    fn from(value: FeFuncR) -> Self {
        Self::FeFuncR(Box::new(value))
    }
}
impl From<FeFuncG> for Leaf {
    fn from(value: FeFuncG) -> Self {
        Self::FeFuncG(Box::new(value))
    }
}
impl From<FeFuncB> for Leaf {
    fn from(value: FeFuncB) -> Self {
        Self::FeFuncB(Box::new(value))
    }
}
impl From<FeComposite> for Leaf {
    fn from(value: FeComposite) -> Self {
        Self::FeComposite(Box::new(value))
    }
}
impl From<FeConvolveMatrix> for Leaf {
    fn from(value: FeConvolveMatrix) -> Self {
        Self::FeConvolveMatrix(Box::new(value))
    }
}
impl From<FeDisplacementMap> for Leaf {
    fn from(value: FeDisplacementMap) -> Self {
        Self::FeDisplacementMap(Box::new(value))
    }
}
impl From<FeFlood> for Leaf {
    fn from(value: FeFlood) -> Self {
        Self::FeFlood(Box::new(value))
    }
}
impl From<FeGaussianBlur> for Leaf {
    fn from(value: FeGaussianBlur) -> Self {
        Self::FeGaussianBlur(Box::new(value))
    }
}
impl From<FeMergeNode> for Leaf {
    fn from(value: FeMergeNode) -> Self {
        Self::FeMergeNode(Box::new(value))
    }
}
impl From<FeImage> for Leaf {
    fn from(value: FeImage) -> Self {
        Self::FeImage(Box::new(value))
    }
}
impl From<FeMorphology> for Leaf {
    fn from(value: FeMorphology) -> Self {
        Self::FeMorphology(Box::new(value))
    }
}
impl From<FeOffset> for Leaf {
    fn from(value: FeOffset) -> Self {
        Self::FeOffset(Box::new(value))
    }
}
impl From<FeTile> for Leaf {
    fn from(value: FeTile) -> Self {
        Self::FeTile(Box::new(value))
    }
}
impl From<FeTurbulence> for Leaf {
    fn from(value: FeTurbulence) -> Self {
        Self::FeTurbulence(Box::new(value))
    }
}
impl From<GradientStop> for Leaf {
    fn from(value: GradientStop) -> Self {
        Self::GradientStop(Box::new(value))
    }
}
impl From<Path> for Leaf {
    fn from(value: Path) -> Self {
        Self::Path(Box::new(value))
    }
}
impl From<Use> for Leaf {
    fn from(value: Use) -> Self {
        Self::Use(Box::new(value))
    }
}
impl From<Rect> for Leaf {
    fn from(value: Rect) -> Self {
        Self::Rect(Box::new(value))
    }
}
impl From<Circle> for Leaf {
    fn from(value: Circle) -> Self {
        Self::Circle(Box::new(value))
    }
}
impl From<Ellipse> for Leaf {
    fn from(value: Ellipse) -> Self {
        Self::Ellipse(Box::new(value))
    }
}
impl From<Line> for Leaf {
    fn from(value: Line) -> Self {
        Self::Line(Box::new(value))
    }
}
impl From<Polyline> for Leaf {
    fn from(value: Polyline) -> Self {
        Self::Polyline(Box::new(value))
    }
}
impl From<Polygon> for Leaf {
    fn from(value: Polygon) -> Self {
        Self::Polygon(Box::new(value))
    }
}
impl From<Characters> for Leaf {
    fn from(value: Characters) -> Self {
        Self::Characters(Box::new(value))
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    Apply(Attr),
    Element(Element),
    Pop,
    Leaf(Leaf),
}
impl From<Attr> for Opcode {
    fn from(value: Attr) -> Self {
        Self::Apply(value)
    }
}
impl From<Element> for Opcode {
    fn from(value: Element) -> Self {
        Self::Element(value)
    }
}
impl From<Leaf> for Opcode {
    fn from(value: Leaf) -> Self {
        Self::Leaf(value)
    }
}
pub mod variable {
    #[doc = r" The path used by [`Variable`] is used to point to [`Target`]."]
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum Path {
        #[doc = r" Reference by name."]
        Named(String),
        #[doc = r" Reference by optimized position."]
        Index(usize),
    }
    impl From<String> for Path {
        fn from(value: String) -> Self {
            Self::Named(value)
        }
    }
    impl From<&str> for Path {
        fn from(value: &str) -> Self {
            Self::Named(value.to_owned())
        }
    }
    impl From<usize> for Path {
        fn from(value: usize) -> Self {
            Self::Index(value)
        }
    }
    #[doc = r" The type of variable pointed to by [`Path`]."]
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum Target {
        #[doc = r" Target is animation register."]
        Register,
        #[doc = r" Target is `item` variable returns by `foreach` iterator."]
        ForeachItem,
        #[doc = r" Target is `index` variable returns by `foreach` iterator."]
        ForeachIndex,
        #[doc = r" Target is `index` variable returns by `for range` iterator."]
        Range,
    }
    #[doc = r" Variable used by property fields."]
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum Variable<T>
    where
        super::Data: From<T>,
    {
        #[doc = r" A literal constant value."]
        Constant(T),
        Reference {
            path: Path,
            target: Target,
        },
    }
    impl<T> From<T> for Variable<T>
    where
        super::Data: From<T>,
    {
        fn from(value: T) -> Self {
            Self::Constant(value)
        }
    }
    impl<T> Default for Variable<T>
    where
        T: Default,
        super::Data: From<T>,
    {
        fn default() -> Self {
            Self::Constant(T::default())
        }
    }
    impl<P, T> From<(P, Target)> for Variable<T>
    where
        super::Data: From<T>,
        Path: From<P>,
    {
        fn from(value: (P, Target)) -> Self {
            Self::Reference {
                path: value.0.into(),
                target: value.1,
            }
        }
    }
}
