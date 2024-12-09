use super::FrameVariable;

/// A color structure repesents as RGBA, the storage value is normalized.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgba(pub f32, pub f32, pub f32, pub f32);

/// Rgba can be used as context variant type.
impl FrameVariable for Rgba {}

impl From<Rgba> for [f32; 4] {
    fn from(value: Rgba) -> Self {
        [value.0, value.1, value.2, value.3]
    }
}

impl From<u32> for Rgba {
    fn from(hex: u32) -> Self {
        Rgba::rgb(
            ((hex >> 16) & 0xff) as u8,
            ((hex >> 8) & 0xff) as u8,
            (hex & 0xff) as u8,
        )
    }
}

impl From<(u32, u8)> for Rgba {
    fn from(value: (u32, u8)) -> Self {
        let (hex, opacity) = value;

        Rgba::new(
            ((hex >> 16) & 0xff) as u8,
            ((hex >> 8) & 0xff) as u8,
            (hex & 0xff) as u8,
            opacity,
        )
    }
}
impl From<(u8, u8, u8)> for Rgba {
    fn from(value: (u8, u8, u8)) -> Self {
        let (r, g, b) = value;

        Rgba::rgb(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Rgba {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        let (r, g, b, a) = value;

        Rgba::new(r, g, b, a)
    }
}

impl Rgba {
    /// Create a `Rgba` with opaque alpha channel.
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::new(red, green, blue, 255)
    }

    /// Create a `Rgba` with normalized rgb values.
    pub const fn rgbf(red: f32, green: f32, blue: f32) -> Self {
        Self::newf(red, green, blue, 1.0)
    }

    /// Create a `Rgba` with normalized rgba values.
    pub const fn newf(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self(red, green, blue, alpha)
    }

    /// Create a new `Rgba` isntance.
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self(
            red as f32 / 255f32,
            green as f32 / 255f32,
            blue as f32 / 255f32,
            alpha as f32 / 255f32,
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

impl From<Color> for Rgba {
    fn from(value: Color) -> Self {
        match value {
            Color::aliceblue => Rgba::rgb(240, 248, 255),
            Color::antiquewhite => Rgba::rgb(250, 235, 215),
            Color::aqua => Rgba::rgb(0, 255, 255),
            Color::aquamarine => Rgba::rgb(127, 255, 212),
            Color::azure => Rgba::rgb(240, 255, 255),
            Color::beige => Rgba::rgb(245, 245, 220),
            Color::bisque => Rgba::rgb(255, 228, 196),
            Color::black => Rgba::rgb(0, 0, 0),
            Color::blanchedalmond => Rgba::rgb(255, 235, 205),
            Color::blue => Rgba::rgb(0, 0, 255),
            Color::blueviolet => Rgba::rgb(138, 43, 226),
            Color::brown => Rgba::rgb(165, 42, 42),
            Color::burlywood => Rgba::rgb(222, 184, 135),
            Color::cadetblue => Rgba::rgb(95, 158, 160),
            Color::chartreuse => Rgba::rgb(127, 255, 0),
            Color::chocolate => Rgba::rgb(210, 105, 30),
            Color::coral => Rgba::rgb(255, 127, 80),
            Color::cornflowerblue => Rgba::rgb(100, 149, 237),
            Color::cornsilk => Rgba::rgb(255, 248, 220),
            Color::crimson => Rgba::rgb(220, 20, 60),
            Color::cyan => Rgba::rgb(0, 255, 255),
            Color::darkblue => Rgba::rgb(0, 0, 139),
            Color::darkcyan => Rgba::rgb(0, 139, 139),
            Color::darkgoldenrod => Rgba::rgb(184, 134, 11),
            Color::darkgray => Rgba::rgb(169, 169, 169),
            Color::darkgreen => Rgba::rgb(0, 100, 0),
            Color::darkgrey => Rgba::rgb(169, 169, 169),
            Color::darkkhaki => Rgba::rgb(189, 183, 107),
            Color::darkmagenta => Rgba::rgb(139, 0, 139),
            Color::darkolivegreen => Rgba::rgb(85, 107, 47),
            Color::darkorange => Rgba::rgb(255, 140, 0),
            Color::darkorchid => Rgba::rgb(153, 50, 204),
            Color::darkred => Rgba::rgb(139, 0, 0),
            Color::darksalmon => Rgba::rgb(233, 150, 122),
            Color::darkseagreen => Rgba::rgb(143, 188, 143),
            Color::darkslateblue => Rgba::rgb(72, 61, 139),
            Color::darkslategray => Rgba::rgb(47, 79, 79),
            Color::darkslategrey => Rgba::rgb(47, 79, 79),
            Color::darkturquoise => Rgba::rgb(0, 206, 209),
            Color::darkviolet => Rgba::rgb(148, 0, 211),
            Color::deeppink => Rgba::rgb(255, 20, 147),
            Color::deepskyblue => Rgba::rgb(0, 191, 255),
            Color::dimgray => Rgba::rgb(105, 105, 105),
            Color::dimgrey => Rgba::rgb(105, 105, 105),
            Color::dodgerblue => Rgba::rgb(30, 144, 255),
            Color::firebrick => Rgba::rgb(178, 34, 34),
            Color::floralwhite => Rgba::rgb(255, 250, 240),
            Color::forestgreen => Rgba::rgb(34, 139, 34),
            Color::fuchsia => Rgba::rgb(255, 0, 255),
            Color::gainsboro => Rgba::rgb(220, 220, 220),
            Color::ghostwhite => Rgba::rgb(248, 248, 255),
            Color::gold => Rgba::rgb(255, 215, 0),
            Color::goldenrod => Rgba::rgb(218, 165, 32),
            Color::gray => Rgba::rgb(128, 128, 128),
            Color::grey => Rgba::rgb(128, 128, 128),
            Color::green => Rgba::rgb(0, 128, 0),
            Color::greenyellow => Rgba::rgb(173, 255, 47),
            Color::honeydew => Rgba::rgb(240, 255, 240),
            Color::hotpink => Rgba::rgb(255, 105, 180),
            Color::indianred => Rgba::rgb(205, 92, 92),
            Color::indigo => Rgba::rgb(75, 0, 130),
            Color::ivory => Rgba::rgb(255, 255, 240),
            Color::khaki => Rgba::rgb(240, 230, 140),
            Color::lavender => Rgba::rgb(230, 230, 250),
            Color::lavenderblush => Rgba::rgb(255, 240, 245),
            Color::lawngreen => Rgba::rgb(124, 252, 0),
            Color::lemonchiffon => Rgba::rgb(255, 250, 205),
            Color::lightblue => Rgba::rgb(173, 216, 230),
            Color::lightcoral => Rgba::rgb(240, 128, 128),
            Color::lightcyan => Rgba::rgb(224, 255, 255),
            Color::lightgoldenrodyellow => Rgba::rgb(250, 250, 210),
            Color::lightgray => Rgba::rgb(211, 211, 211),
            Color::lightgreen => Rgba::rgb(144, 238, 144),
            Color::lightgrey => Rgba::rgb(211, 211, 211),
            Color::lightpink => Rgba::rgb(255, 182, 193),
            Color::lightsalmon => Rgba::rgb(255, 160, 122),
            Color::lightseagreen => Rgba::rgb(32, 178, 170),
            Color::lightskyblue => Rgba::rgb(135, 206, 250),
            Color::lightslategray => Rgba::rgb(119, 136, 153),
            Color::lightslategrey => Rgba::rgb(119, 136, 153),
            Color::lightsteelblue => Rgba::rgb(176, 196, 222),
            Color::lightyellow => Rgba::rgb(255, 255, 224),
            Color::lime => Rgba::rgb(0, 255, 0),
            Color::limegreen => Rgba::rgb(50, 205, 50),
            Color::linen => Rgba::rgb(250, 240, 230),
            Color::magenta => Rgba::rgb(255, 0, 255),
            Color::maroon => Rgba::rgb(128, 0, 0),
            Color::mediumaquamarine => Rgba::rgb(102, 205, 170),
            Color::mediumblue => Rgba::rgb(0, 0, 205),
            Color::mediumorchid => Rgba::rgb(186, 85, 211),
            Color::mediumpurple => Rgba::rgb(147, 112, 219),
            Color::mediumseagreen => Rgba::rgb(60, 179, 113),
            Color::mediumslateblue => Rgba::rgb(123, 104, 238),
            Color::mediumspringgreen => Rgba::rgb(0, 250, 154),
            Color::mediumturquoise => Rgba::rgb(72, 209, 204),
            Color::mediumvioletred => Rgba::rgb(199, 21, 133),
            Color::midnightblue => Rgba::rgb(25, 25, 112),
            Color::mintcream => Rgba::rgb(245, 255, 250),
            Color::mistyrose => Rgba::rgb(255, 228, 225),
            Color::moccasin => Rgba::rgb(255, 228, 181),
            Color::navajowhite => Rgba::rgb(255, 222, 173),
            Color::navy => Rgba::rgb(0, 0, 128),
            Color::oldlace => Rgba::rgb(253, 245, 230),
            Color::olive => Rgba::rgb(128, 128, 0),
            Color::olivedrab => Rgba::rgb(107, 142, 35),
            Color::orange => Rgba::rgb(255, 165, 0),
            Color::orangered => Rgba::rgb(255, 69, 0),
            Color::orchid => Rgba::rgb(218, 112, 214),
            Color::palegoldenrod => Rgba::rgb(238, 232, 170),
            Color::palegreen => Rgba::rgb(152, 251, 152),
            Color::paleturquoise => Rgba::rgb(175, 238, 238),
            Color::palevioletred => Rgba::rgb(219, 112, 147),
            Color::papayawhip => Rgba::rgb(255, 239, 213),
            Color::peachpuff => Rgba::rgb(255, 218, 185),
            Color::peru => Rgba::rgb(205, 133, 63),
            Color::pink => Rgba::rgb(255, 192, 203),
            Color::plum => Rgba::rgb(221, 160, 221),
            Color::powderblue => Rgba::rgb(176, 224, 230),
            Color::purple => Rgba::rgb(128, 0, 128),
            Color::red => Rgba::rgb(255, 0, 0),
            Color::rosybrown => Rgba::rgb(188, 143, 143),
            Color::royalblue => Rgba::rgb(65, 105, 225),
            Color::saddlebrown => Rgba::rgb(139, 69, 19),
            Color::salmon => Rgba::rgb(250, 128, 114),
            Color::sandybrown => Rgba::rgb(244, 164, 96),
            Color::seagreen => Rgba::rgb(46, 139, 87),
            Color::seashell => Rgba::rgb(255, 245, 238),
            Color::sienna => Rgba::rgb(160, 82, 45),
            Color::silver => Rgba::rgb(192, 192, 192),
            Color::skyblue => Rgba::rgb(135, 206, 235),
            Color::slateblue => Rgba::rgb(106, 90, 205),
            Color::slategray => Rgba::rgb(112, 128, 144),
            Color::slategrey => Rgba::rgb(112, 128, 144),
            Color::snow => Rgba::rgb(255, 250, 250),
            Color::springgreen => Rgba::rgb(0, 255, 127),
            Color::steelblue => Rgba::rgb(70, 130, 180),
            Color::tan => Rgba::rgb(210, 180, 140),
            Color::teal => Rgba::rgb(0, 128, 128),
            Color::thistle => Rgba::rgb(216, 191, 216),
            Color::tomato => Rgba::rgb(255, 99, 71),
            Color::turquoise => Rgba::rgb(64, 224, 208),
            Color::violet => Rgba::rgb(238, 130, 238),
            Color::wheat => Rgba::rgb(245, 222, 179),
            Color::white => Rgba::rgb(255, 255, 255),
            Color::whitesmoke => Rgba::rgb(245, 245, 245),
            Color::yellow => Rgba::rgb(255, 255, 0),
            Color::yellowgreen => Rgba::rgb(154, 205, 50),
        }
    }
}
