use std::fmt::Display;

/// A sexpr to create rgb value, the storage value is normalized.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgb(pub f32, pub f32, pub f32);

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "rgb({},{},{})",
            (self.0 * 255.0) as u8,
            (self.1 * 255.0) as u8,
            (self.2 * 255.0) as u8
        )
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from(value: (u8, u8, u8)) -> Self {
        let (r, g, b) = value;

        Rgb::rgb(r, g, b)
    }
}

impl Rgb {
    /// Create a new `Srgb` instance from (red,green,blue) components.
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(
            red as f32 / 255f32,
            green as f32 / 255f32,
            blue as f32 / 255f32,
        )
    }
}

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
