pub use super::ml::sexpr::*;
pub use vglang_derive::rgb;

use crate::opcode::{Angle, Color, FeIn, FontFamily, FuncIri, Length, Paint, Rgb};

impl<T> From<T> for Length
where
    Number: From<T>,
{
    fn from(value: T) -> Self {
        Self::Px(Number::from(value).0)
    }
}

impl<T> From<T> for Angle
where
    Number: From<T>,
{
    fn from(value: T) -> Self {
        Self::Deg(Number::from(value).0)
    }
}

impl Rgb {
    /// Create a new `rgb` instance from (red,green,blue) channels.
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(red, green, blue)
    }
}

impl From<Color> for Rgb {
    fn from(value: Color) -> Self {
        match value {
            Color::Aliceblue => Rgb::rgb(240, 248, 255),
            Color::Antiquewhite => Rgb::rgb(250, 235, 215),
            Color::Aqua => Rgb::rgb(0, 255, 255),
            Color::Aquamarine => Rgb::rgb(127, 255, 212),
            Color::Azure => Rgb::rgb(240, 255, 255),
            Color::Beige => Rgb::rgb(245, 245, 220),
            Color::Bisque => Rgb::rgb(255, 228, 196),
            Color::Black => Rgb::rgb(0, 0, 0),
            Color::Blanchedalmond => Rgb::rgb(255, 235, 205),
            Color::Blue => Rgb::rgb(0, 0, 255),
            Color::Blueviolet => Rgb::rgb(138, 43, 226),
            Color::Brown => Rgb::rgb(165, 42, 42),
            Color::Burlywood => Rgb::rgb(222, 184, 135),
            Color::Cadetblue => Rgb::rgb(95, 158, 160),
            Color::Chartreuse => Rgb::rgb(127, 255, 0),
            Color::Chocolate => Rgb::rgb(210, 105, 30),
            Color::Coral => Rgb::rgb(255, 127, 80),
            Color::Cornflowerblue => Rgb::rgb(100, 149, 237),
            Color::Cornsilk => Rgb::rgb(255, 248, 220),
            Color::Crimson => Rgb::rgb(220, 20, 60),
            Color::Cyan => Rgb::rgb(0, 255, 255),
            Color::Darkblue => Rgb::rgb(0, 0, 139),
            Color::Darkcyan => Rgb::rgb(0, 139, 139),
            Color::Darkgoldenrod => Rgb::rgb(184, 134, 11),
            Color::Darkgray => Rgb::rgb(169, 169, 169),
            Color::Darkgreen => Rgb::rgb(0, 100, 0),
            Color::Darkgrey => Rgb::rgb(169, 169, 169),
            Color::Darkkhaki => Rgb::rgb(189, 183, 107),
            Color::Darkmagenta => Rgb::rgb(139, 0, 139),
            Color::Darkolivegreen => Rgb::rgb(85, 107, 47),
            Color::Darkorange => Rgb::rgb(255, 140, 0),
            Color::Darkorchid => Rgb::rgb(153, 50, 204),
            Color::Darkred => Rgb::rgb(139, 0, 0),
            Color::Darksalmon => Rgb::rgb(233, 150, 122),
            Color::Darkseagreen => Rgb::rgb(143, 188, 143),
            Color::Darkslateblue => Rgb::rgb(72, 61, 139),
            Color::Darkslategray => Rgb::rgb(47, 79, 79),
            Color::Darkslategrey => Rgb::rgb(47, 79, 79),
            Color::Darkturquoise => Rgb::rgb(0, 206, 209),
            Color::Darkviolet => Rgb::rgb(148, 0, 211),
            Color::Deeppink => Rgb::rgb(255, 20, 147),
            Color::Deepskyblue => Rgb::rgb(0, 191, 255),
            Color::Dimgray => Rgb::rgb(105, 105, 105),
            Color::Dimgrey => Rgb::rgb(105, 105, 105),
            Color::Dodgerblue => Rgb::rgb(30, 144, 255),
            Color::Firebrick => Rgb::rgb(178, 34, 34),
            Color::Floralwhite => Rgb::rgb(255, 250, 240),
            Color::Forestgreen => Rgb::rgb(34, 139, 34),
            Color::Fuchsia => Rgb::rgb(255, 0, 255),
            Color::Gainsboro => Rgb::rgb(220, 220, 220),
            Color::Ghostwhite => Rgb::rgb(248, 248, 255),
            Color::Gold => Rgb::rgb(255, 215, 0),
            Color::Goldenrod => Rgb::rgb(218, 165, 32),
            Color::Gray => Rgb::rgb(128, 128, 128),
            Color::Grey => Rgb::rgb(128, 128, 128),
            Color::Green => Rgb::rgb(0, 128, 0),
            Color::Greenyellow => Rgb::rgb(173, 255, 47),
            Color::Honeydew => Rgb::rgb(240, 255, 240),
            Color::Hotpink => Rgb::rgb(255, 105, 180),
            Color::Indianred => Rgb::rgb(205, 92, 92),
            Color::Indigo => Rgb::rgb(75, 0, 130),
            Color::Ivory => Rgb::rgb(255, 255, 240),
            Color::Khaki => Rgb::rgb(240, 230, 140),
            Color::Lavender => Rgb::rgb(230, 230, 250),
            Color::Lavenderblush => Rgb::rgb(255, 240, 245),
            Color::Lawngreen => Rgb::rgb(124, 252, 0),
            Color::Lemonchiffon => Rgb::rgb(255, 250, 205),
            Color::Lightblue => Rgb::rgb(173, 216, 230),
            Color::Lightcoral => Rgb::rgb(240, 128, 128),
            Color::Lightcyan => Rgb::rgb(224, 255, 255),
            Color::Lightgoldenrodyellow => Rgb::rgb(250, 250, 210),
            Color::Lightgray => Rgb::rgb(211, 211, 211),
            Color::Lightgreen => Rgb::rgb(144, 238, 144),
            Color::Lightgrey => Rgb::rgb(211, 211, 211),
            Color::Lightpink => Rgb::rgb(255, 182, 193),
            Color::Lightsalmon => Rgb::rgb(255, 160, 122),
            Color::Lightseagreen => Rgb::rgb(32, 178, 170),
            Color::Lightskyblue => Rgb::rgb(135, 206, 250),
            Color::Lightslategray => Rgb::rgb(119, 136, 153),
            Color::Lightslategrey => Rgb::rgb(119, 136, 153),
            Color::Lightsteelblue => Rgb::rgb(176, 196, 222),
            Color::Lightyellow => Rgb::rgb(255, 255, 224),
            Color::Lime => Rgb::rgb(0, 255, 0),
            Color::Limegreen => Rgb::rgb(50, 205, 50),
            Color::Linen => Rgb::rgb(250, 240, 230),
            Color::Magenta => Rgb::rgb(255, 0, 255),
            Color::Maroon => Rgb::rgb(128, 0, 0),
            Color::Mediumaquamarine => Rgb::rgb(102, 205, 170),
            Color::Mediumblue => Rgb::rgb(0, 0, 205),
            Color::Mediumorchid => Rgb::rgb(186, 85, 211),
            Color::Mediumpurple => Rgb::rgb(147, 112, 219),
            Color::Mediumseagreen => Rgb::rgb(60, 179, 113),
            Color::Mediumslateblue => Rgb::rgb(123, 104, 238),
            Color::Mediumspringgreen => Rgb::rgb(0, 250, 154),
            Color::Mediumturquoise => Rgb::rgb(72, 209, 204),
            Color::Mediumvioletred => Rgb::rgb(199, 21, 133),
            Color::Midnightblue => Rgb::rgb(25, 25, 112),
            Color::Mintcream => Rgb::rgb(245, 255, 250),
            Color::Mistyrose => Rgb::rgb(255, 228, 225),
            Color::Moccasin => Rgb::rgb(255, 228, 181),
            Color::Navajowhite => Rgb::rgb(255, 222, 173),
            Color::Navy => Rgb::rgb(0, 0, 128),
            Color::Oldlace => Rgb::rgb(253, 245, 230),
            Color::Olive => Rgb::rgb(128, 128, 0),
            Color::Olivedrab => Rgb::rgb(107, 142, 35),
            Color::Orange => Rgb::rgb(255, 165, 0),
            Color::Orangered => Rgb::rgb(255, 69, 0),
            Color::Orchid => Rgb::rgb(218, 112, 214),
            Color::Palegoldenrod => Rgb::rgb(238, 232, 170),
            Color::Palegreen => Rgb::rgb(152, 251, 152),
            Color::Paleturquoise => Rgb::rgb(175, 238, 238),
            Color::Palevioletred => Rgb::rgb(219, 112, 147),
            Color::Papayawhip => Rgb::rgb(255, 239, 213),
            Color::Peachpuff => Rgb::rgb(255, 218, 185),
            Color::Peru => Rgb::rgb(205, 133, 63),
            Color::Pink => Rgb::rgb(255, 192, 203),
            Color::Plum => Rgb::rgb(221, 160, 221),
            Color::Powderblue => Rgb::rgb(176, 224, 230),
            Color::Purple => Rgb::rgb(128, 0, 128),
            Color::Red => Rgb::rgb(255, 0, 0),
            Color::Rosybrown => Rgb::rgb(188, 143, 143),
            Color::Royalblue => Rgb::rgb(65, 105, 225),
            Color::Saddlebrown => Rgb::rgb(139, 69, 19),
            Color::Salmon => Rgb::rgb(250, 128, 114),
            Color::Sandybrown => Rgb::rgb(244, 164, 96),
            Color::Seagreen => Rgb::rgb(46, 139, 87),
            Color::Seashell => Rgb::rgb(255, 245, 238),
            Color::Sienna => Rgb::rgb(160, 82, 45),
            Color::Silver => Rgb::rgb(192, 192, 192),
            Color::Skyblue => Rgb::rgb(135, 206, 235),
            Color::Slateblue => Rgb::rgb(106, 90, 205),
            Color::Slategray => Rgb::rgb(112, 128, 144),
            Color::Slategrey => Rgb::rgb(112, 128, 144),
            Color::Snow => Rgb::rgb(255, 250, 250),
            Color::Springgreen => Rgb::rgb(0, 255, 127),
            Color::Steelblue => Rgb::rgb(70, 130, 180),
            Color::Tan => Rgb::rgb(210, 180, 140),
            Color::Teal => Rgb::rgb(0, 128, 128),
            Color::Thistle => Rgb::rgb(216, 191, 216),
            Color::Tomato => Rgb::rgb(255, 99, 71),
            Color::Turquoise => Rgb::rgb(64, 224, 208),
            Color::Violet => Rgb::rgb(238, 130, 238),
            Color::Wheat => Rgb::rgb(245, 222, 179),
            Color::White => Rgb::rgb(255, 255, 255),
            Color::Whitesmoke => Rgb::rgb(245, 245, 245),
            Color::Yellow => Rgb::rgb(255, 255, 0),
            Color::Yellowgreen => Rgb::rgb(154, 205, 50),
        }
    }
}

impl<T> From<T> for Paint
where
    FuncIri: From<T>,
{
    fn from(value: T) -> Self {
        Self::Server(value.into())
    }
}

impl From<Color> for Paint {
    fn from(value: Color) -> Self {
        Self::Color(value.into())
    }
}

impl From<Rgb> for Paint {
    fn from(value: Rgb) -> Self {
        Self::Color(value)
    }
}

impl From<&str> for FontFamily {
    fn from(value: &str) -> Self {
        Self::Generic(value.into())
    }
}
impl From<String> for FontFamily {
    fn from(value: String) -> Self {
        Self::Generic(value)
    }
}

impl From<&str> for FeIn {
    fn from(value: &str) -> Self {
        Self::Result(value.into())
    }
}

impl From<String> for FeIn {
    fn from(value: String) -> Self {
        Self::Result(value)
    }
}
