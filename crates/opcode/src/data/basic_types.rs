use std::{f32::consts::PI, ops::Deref};

use super::{impl_data_type, Data, DataType};

/// An `integer` is specified as an optional sign character ("+" or "-") followed by one or more digits "0" to "9":
///
/// ***integer ::= [+-]? [0-9]+***
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Integer(
    /// The wrapped [`i32`] value.
    pub i32,
);

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Deref for Integer {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl_data_type!(Integer);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Number(
    /// The wrapped [`f32`] value.
    pub f32,
);

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self(value as f32)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self(value as f32)
    }
}

impl Deref for Number {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl_data_type!(Number);

/// Angles are specified in one of two ways depending upon
/// whether they are used in CSS property syntax or SVG
/// presentation attribute syntax:
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Angle {
    deg(f32),
    grad(f32),
    rad(f32),
}

impl Default for Angle {
    fn default() -> Self {
        Self::deg(0.0)
    }
}

impl From<i32> for Angle {
    fn from(value: i32) -> Self {
        Self::deg(value as f32)
    }
}

impl From<f32> for Angle {
    fn from(value: f32) -> Self {
        Self::deg(value)
    }
}

impl Angle {
    pub fn as_deg(&self) -> f32 {
        match self {
            Angle::deg(v) => *v,
            Angle::grad(v) => *v * 360.0 / 400.0,
            Angle::rad(v) => *v * 180.0 / PI,
        }
    }
}

impl_data_type!(Angle);
impl_data_type!(listof, Angle);

/// Recognized color keyword names, compliant with svg 1.1.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Color {
    aliceblue,
    antiquewhite,
    aqua,
    aquamarine,
    azure,
    beige,
    bisque,
    black,
    blanchedalmond,
    blue,
    blueviolet,
    brown,
    burlywood,
    cadetblue,
    chartreuse,
    chocolate,
    coral,
    cornflowerblue,
    cornsilk,
    crimson,
    cyan,
    darkblue,
    darkcyan,
    darkgoldenrod,
    darkgray,
    darkgreen,
    darkgrey,
    darkkhaki,
    darkmagenta,
    darkolivegreen,
    darkorange,
    darkorchid,
    darkred,
    darksalmon,
    darkseagreen,
    darkslateblue,
    darkslategray,
    darkslategrey,
    darkturquoise,
    darkviolet,
    deeppink,
    deepskyblue,
    dimgray,
    dimgrey,
    dodgerblue,
    firebrick,
    floralwhite,
    forestgreen,
    fuchsia,
    gainsboro,
    ghostwhite,
    gold,
    goldenrod,
    gray,
    grey,
    green,
    greenyellow,
    honeydew,
    hotpink,
    indianred,
    indigo,
    ivory,
    khaki,
    lavender,
    lavenderblush,
    lawngreen,
    lemonchiffon,
    lightblue,
    lightcoral,
    lightcyan,
    lightgoldenrodyellow,
    lightgray,
    lightgreen,
    lightgrey,
    lightpink,
    lightsalmon,
    lightseagreen,
    lightskyblue,
    lightslategray,
    lightslategrey,
    lightsteelblue,
    lightyellow,
    lime,
    limegreen,
    linen,
    magenta,
    maroon,
    mediumaquamarine,
    mediumblue,
    mediumorchid,
    mediumpurple,
    mediumseagreen,
    mediumslateblue,
    mediumspringgreen,
    mediumturquoise,
    mediumvioletred,
    midnightblue,
    mintcream,
    mistyrose,
    moccasin,
    navajowhite,
    navy,
    oldlace,
    olive,
    olivedrab,
    orange,
    orangered,
    orchid,
    palegoldenrod,
    palegreen,
    paleturquoise,
    palevioletred,
    papayawhip,
    peachpuff,
    peru,
    pink,
    plum,
    powderblue,
    purple,
    red,
    rosybrown,
    royalblue,
    saddlebrown,
    salmon,
    sandybrown,
    seagreen,
    seashell,
    sienna,
    silver,
    skyblue,
    slateblue,
    slategray,
    slategrey,
    snow,
    springgreen,
    steelblue,
    tan,
    teal,
    thistle,
    tomato,
    turquoise,
    violet,
    wheat,
    white,
    whitesmoke,
    yellow,
    yellowgreen,
}

/// A sexpr to create rgb value, the storage value is normalized.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgb(pub u8, pub u8, pub u8);

impl From<(u8, u8, u8)> for Rgb {
    fn from(value: (u8, u8, u8)) -> Self {
        let (r, g, b) = value;

        Rgb::rgb(r, g, b)
    }
}

impl From<Color> for Rgb {
    fn from(value: Color) -> Self {
        match value {
            Color::aliceblue => Rgb::rgb(240, 248, 255),
            Color::antiquewhite => Rgb::rgb(250, 235, 215),
            Color::aqua => Rgb::rgb(0, 255, 255),
            Color::aquamarine => Rgb::rgb(127, 255, 212),
            Color::azure => Rgb::rgb(240, 255, 255),
            Color::beige => Rgb::rgb(245, 245, 220),
            Color::bisque => Rgb::rgb(255, 228, 196),
            Color::black => Rgb::rgb(0, 0, 0),
            Color::blanchedalmond => Rgb::rgb(255, 235, 205),
            Color::blue => Rgb::rgb(0, 0, 255),
            Color::blueviolet => Rgb::rgb(138, 43, 226),
            Color::brown => Rgb::rgb(165, 42, 42),
            Color::burlywood => Rgb::rgb(222, 184, 135),
            Color::cadetblue => Rgb::rgb(95, 158, 160),
            Color::chartreuse => Rgb::rgb(127, 255, 0),
            Color::chocolate => Rgb::rgb(210, 105, 30),
            Color::coral => Rgb::rgb(255, 127, 80),
            Color::cornflowerblue => Rgb::rgb(100, 149, 237),
            Color::cornsilk => Rgb::rgb(255, 248, 220),
            Color::crimson => Rgb::rgb(220, 20, 60),
            Color::cyan => Rgb::rgb(0, 255, 255),
            Color::darkblue => Rgb::rgb(0, 0, 139),
            Color::darkcyan => Rgb::rgb(0, 139, 139),
            Color::darkgoldenrod => Rgb::rgb(184, 134, 11),
            Color::darkgray => Rgb::rgb(169, 169, 169),
            Color::darkgreen => Rgb::rgb(0, 100, 0),
            Color::darkgrey => Rgb::rgb(169, 169, 169),
            Color::darkkhaki => Rgb::rgb(189, 183, 107),
            Color::darkmagenta => Rgb::rgb(139, 0, 139),
            Color::darkolivegreen => Rgb::rgb(85, 107, 47),
            Color::darkorange => Rgb::rgb(255, 140, 0),
            Color::darkorchid => Rgb::rgb(153, 50, 204),
            Color::darkred => Rgb::rgb(139, 0, 0),
            Color::darksalmon => Rgb::rgb(233, 150, 122),
            Color::darkseagreen => Rgb::rgb(143, 188, 143),
            Color::darkslateblue => Rgb::rgb(72, 61, 139),
            Color::darkslategray => Rgb::rgb(47, 79, 79),
            Color::darkslategrey => Rgb::rgb(47, 79, 79),
            Color::darkturquoise => Rgb::rgb(0, 206, 209),
            Color::darkviolet => Rgb::rgb(148, 0, 211),
            Color::deeppink => Rgb::rgb(255, 20, 147),
            Color::deepskyblue => Rgb::rgb(0, 191, 255),
            Color::dimgray => Rgb::rgb(105, 105, 105),
            Color::dimgrey => Rgb::rgb(105, 105, 105),
            Color::dodgerblue => Rgb::rgb(30, 144, 255),
            Color::firebrick => Rgb::rgb(178, 34, 34),
            Color::floralwhite => Rgb::rgb(255, 250, 240),
            Color::forestgreen => Rgb::rgb(34, 139, 34),
            Color::fuchsia => Rgb::rgb(255, 0, 255),
            Color::gainsboro => Rgb::rgb(220, 220, 220),
            Color::ghostwhite => Rgb::rgb(248, 248, 255),
            Color::gold => Rgb::rgb(255, 215, 0),
            Color::goldenrod => Rgb::rgb(218, 165, 32),
            Color::gray => Rgb::rgb(128, 128, 128),
            Color::grey => Rgb::rgb(128, 128, 128),
            Color::green => Rgb::rgb(0, 128, 0),
            Color::greenyellow => Rgb::rgb(173, 255, 47),
            Color::honeydew => Rgb::rgb(240, 255, 240),
            Color::hotpink => Rgb::rgb(255, 105, 180),
            Color::indianred => Rgb::rgb(205, 92, 92),
            Color::indigo => Rgb::rgb(75, 0, 130),
            Color::ivory => Rgb::rgb(255, 255, 240),
            Color::khaki => Rgb::rgb(240, 230, 140),
            Color::lavender => Rgb::rgb(230, 230, 250),
            Color::lavenderblush => Rgb::rgb(255, 240, 245),
            Color::lawngreen => Rgb::rgb(124, 252, 0),
            Color::lemonchiffon => Rgb::rgb(255, 250, 205),
            Color::lightblue => Rgb::rgb(173, 216, 230),
            Color::lightcoral => Rgb::rgb(240, 128, 128),
            Color::lightcyan => Rgb::rgb(224, 255, 255),
            Color::lightgoldenrodyellow => Rgb::rgb(250, 250, 210),
            Color::lightgray => Rgb::rgb(211, 211, 211),
            Color::lightgreen => Rgb::rgb(144, 238, 144),
            Color::lightgrey => Rgb::rgb(211, 211, 211),
            Color::lightpink => Rgb::rgb(255, 182, 193),
            Color::lightsalmon => Rgb::rgb(255, 160, 122),
            Color::lightseagreen => Rgb::rgb(32, 178, 170),
            Color::lightskyblue => Rgb::rgb(135, 206, 250),
            Color::lightslategray => Rgb::rgb(119, 136, 153),
            Color::lightslategrey => Rgb::rgb(119, 136, 153),
            Color::lightsteelblue => Rgb::rgb(176, 196, 222),
            Color::lightyellow => Rgb::rgb(255, 255, 224),
            Color::lime => Rgb::rgb(0, 255, 0),
            Color::limegreen => Rgb::rgb(50, 205, 50),
            Color::linen => Rgb::rgb(250, 240, 230),
            Color::magenta => Rgb::rgb(255, 0, 255),
            Color::maroon => Rgb::rgb(128, 0, 0),
            Color::mediumaquamarine => Rgb::rgb(102, 205, 170),
            Color::mediumblue => Rgb::rgb(0, 0, 205),
            Color::mediumorchid => Rgb::rgb(186, 85, 211),
            Color::mediumpurple => Rgb::rgb(147, 112, 219),
            Color::mediumseagreen => Rgb::rgb(60, 179, 113),
            Color::mediumslateblue => Rgb::rgb(123, 104, 238),
            Color::mediumspringgreen => Rgb::rgb(0, 250, 154),
            Color::mediumturquoise => Rgb::rgb(72, 209, 204),
            Color::mediumvioletred => Rgb::rgb(199, 21, 133),
            Color::midnightblue => Rgb::rgb(25, 25, 112),
            Color::mintcream => Rgb::rgb(245, 255, 250),
            Color::mistyrose => Rgb::rgb(255, 228, 225),
            Color::moccasin => Rgb::rgb(255, 228, 181),
            Color::navajowhite => Rgb::rgb(255, 222, 173),
            Color::navy => Rgb::rgb(0, 0, 128),
            Color::oldlace => Rgb::rgb(253, 245, 230),
            Color::olive => Rgb::rgb(128, 128, 0),
            Color::olivedrab => Rgb::rgb(107, 142, 35),
            Color::orange => Rgb::rgb(255, 165, 0),
            Color::orangered => Rgb::rgb(255, 69, 0),
            Color::orchid => Rgb::rgb(218, 112, 214),
            Color::palegoldenrod => Rgb::rgb(238, 232, 170),
            Color::palegreen => Rgb::rgb(152, 251, 152),
            Color::paleturquoise => Rgb::rgb(175, 238, 238),
            Color::palevioletred => Rgb::rgb(219, 112, 147),
            Color::papayawhip => Rgb::rgb(255, 239, 213),
            Color::peachpuff => Rgb::rgb(255, 218, 185),
            Color::peru => Rgb::rgb(205, 133, 63),
            Color::pink => Rgb::rgb(255, 192, 203),
            Color::plum => Rgb::rgb(221, 160, 221),
            Color::powderblue => Rgb::rgb(176, 224, 230),
            Color::purple => Rgb::rgb(128, 0, 128),
            Color::red => Rgb::rgb(255, 0, 0),
            Color::rosybrown => Rgb::rgb(188, 143, 143),
            Color::royalblue => Rgb::rgb(65, 105, 225),
            Color::saddlebrown => Rgb::rgb(139, 69, 19),
            Color::salmon => Rgb::rgb(250, 128, 114),
            Color::sandybrown => Rgb::rgb(244, 164, 96),
            Color::seagreen => Rgb::rgb(46, 139, 87),
            Color::seashell => Rgb::rgb(255, 245, 238),
            Color::sienna => Rgb::rgb(160, 82, 45),
            Color::silver => Rgb::rgb(192, 192, 192),
            Color::skyblue => Rgb::rgb(135, 206, 235),
            Color::slateblue => Rgb::rgb(106, 90, 205),
            Color::slategray => Rgb::rgb(112, 128, 144),
            Color::slategrey => Rgb::rgb(112, 128, 144),
            Color::snow => Rgb::rgb(255, 250, 250),
            Color::springgreen => Rgb::rgb(0, 255, 127),
            Color::steelblue => Rgb::rgb(70, 130, 180),
            Color::tan => Rgb::rgb(210, 180, 140),
            Color::teal => Rgb::rgb(0, 128, 128),
            Color::thistle => Rgb::rgb(216, 191, 216),
            Color::tomato => Rgb::rgb(255, 99, 71),
            Color::turquoise => Rgb::rgb(64, 224, 208),
            Color::violet => Rgb::rgb(238, 130, 238),
            Color::wheat => Rgb::rgb(245, 222, 179),
            Color::white => Rgb::rgb(255, 255, 255),
            Color::whitesmoke => Rgb::rgb(245, 245, 245),
            Color::yellow => Rgb::rgb(255, 255, 0),
            Color::yellowgreen => Rgb::rgb(154, 205, 50),
        }
    }
}

impl Rgb {
    /// Create a new `rgb` instance from (red,green,blue) channels.
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(red, green, blue)
    }
}

impl_data_type!(Rgb);

/// A length is a distance Length, given as a number along with a unit which may be optional.
///
/// See [`length`](https://www.w3.org/TR/SVG11/types.html#DataTypeLength)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Length {
    /// Represents the calculated font-size of the element. If used on the font-size property itself,
    /// it represents the inherited font-size of the element.
    em(f32),
    /// Represents the x-height of the element's font. In fonts with the x letter, this is generally
    /// the height of lowercase letters in the font; 1ex ≈ 0.5em in many fonts.
    ex(f32),
    /// Pixels
    px(f32),
    /// Inches
    r#in(f32),
    /// Centimeters
    cm(f32),
    /// Millimeters
    mm(f32),
    /// Points, 1pt = 1/72nd of 1in
    pt(f32),
    /// Picas, 	1pc = 1/6th of 1in
    pc(f32),
    /// A percentage value
    percent(f32),
}

impl Default for Length {
    fn default() -> Self {
        Self::px(0.0)
    }
}

impl From<f32> for Length {
    fn from(value: f32) -> Self {
        Self::px(value)
    }
}

impl From<i32> for Length {
    fn from(value: i32) -> Self {
        Self::px(value as f32)
    }
}

impl_data_type!(Length);
impl_data_type!(listof, Length);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Iri(
    /// wrapped [`String`] value.
    pub String,
);

impl Deref for Iri {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Iri {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Iri {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl_data_type!(box, Iri);

/// Functional notation for a reference. The syntax for this reference is the same as the [`CSS URI`].
///
/// [`CSS URI`]: https://developer.mozilla.org/en-US/docs/Web/CSS/url_value
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FuncIRI {
    Iri(Iri),
    Path(String),
}

impl From<String> for FuncIRI {
    fn from(value: String) -> Self {
        Self::Path(value)
    }
}

impl From<&str> for FuncIRI {
    fn from(value: &str) -> Self {
        Self::Path(value.to_owned())
    }
}

impl From<Iri> for FuncIRI {
    fn from(value: Iri) -> Self {
        Self::Iri(value)
    }
}

impl_data_type!(box, FuncIRI);

/// This property specifies a prioritized font family names and/or generic family names.
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

impl From<String> for FontFamily {
    fn from(value: String) -> Self {
        Self::Generic(value)
    }
}

impl_data_type!(box, FontFamily);
impl_data_type!(listof, FontFamily);

/// A 2d coordinate point.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Create a point from (f32,f32) with default unit `px`.
impl<X, Y> From<(X, Y)> for Point
where
    Number: From<X> + From<Y>,
{
    fn from(value: (X, Y)) -> Self {
        Self {
            x: Number::from(value.0).0,
            y: Number::from(value.1).0,
        }
    }
}

impl_data_type!(Point);
impl_data_type!(listof, Point);

/// Percentages are specified as a number followed by a "%" character:
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Percentage(pub f32);

impl<X> From<X> for Percentage
where
    Number: From<X>,
{
    fn from(value: X) -> Self {
        Self(Number::from(value).0)
    }
}

impl_data_type!(Percentage);

/// ‘fill’ and ‘stroke’ take on a value of type [`Paint`], which is specified as follows:
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Paint {
    /// the explicit color to be used to paint the current object
    Color(Rgb),
    /// A reference to a paint server.
    Server(FuncIRI),
}

impl_data_type!(box, Paint);

/// A pair of `number`s, where the second `number` is optional.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberOptNumber(
    /// The first number,
    pub f32,
    /// The second optional number.
    pub Option<f32>,
);

impl<X> From<X> for NumberOptNumber
where
    Number: From<X>,
{
    fn from(value: X) -> Self {
        Self(Number::from(value).0, None)
    }
}

impl<X, Y> From<(X, Y)> for NumberOptNumber
where
    Number: From<X> + From<Y>,
{
    fn from(value: (X, Y)) -> Self {
        Self(Number::from(value.0).0, Some(Number::from(value.1).0))
    }
}

impl_data_type!(NumberOptNumber);
