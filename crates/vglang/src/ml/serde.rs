impl mlang::rt::serde::ser::Serialize for super::opcode::Angle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Deg(p0) => {
                let mut serializer =
                    serializer.serialize_enum(0usize, "Angle", "Deg", 0usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Grad(p0) => {
                let mut serializer =
                    serializer.serialize_enum(0usize, "Angle", "Grad", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Rad(p0) => {
                let mut serializer =
                    serializer.serialize_enum(0usize, "Angle", "Rad", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Length {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Em(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Em", 0usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Ex(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Ex", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Px(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Px", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Inch(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Inch", 3usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Cm(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Cm", 4usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Mm(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Mm", 5usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Pt(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Pt", 6usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Pc(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Pc", 7usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Percent(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Percent", 8usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Color {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Aliceblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Aliceblue", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Antiquewhite => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Antiquewhite", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::Aqua => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Aqua", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Aquamarine => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Aquamarine", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::Azure => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Azure", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::Beige => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Beige", 5usize, 0usize)?;
                serializer.finish()
            }
            Self::Bisque => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Bisque", 6usize, 0usize)?;
                serializer.finish()
            }
            Self::Black => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Black", 7usize, 0usize)?;
                serializer.finish()
            }
            Self::Blanchedalmond => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Blanchedalmond", 8usize, 0usize)?;
                serializer.finish()
            }
            Self::Blue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Blue", 9usize, 0usize)?;
                serializer.finish()
            }
            Self::Blueviolet => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Blueviolet", 10usize, 0usize)?;
                serializer.finish()
            }
            Self::Brown => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Brown", 11usize, 0usize)?;
                serializer.finish()
            }
            Self::Burlywood => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Burlywood", 12usize, 0usize)?;
                serializer.finish()
            }
            Self::Cadetblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Cadetblue", 13usize, 0usize)?;
                serializer.finish()
            }
            Self::Chartreuse => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Chartreuse", 14usize, 0usize)?;
                serializer.finish()
            }
            Self::Chocolate => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Chocolate", 15usize, 0usize)?;
                serializer.finish()
            }
            Self::Coral => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Coral", 16usize, 0usize)?;
                serializer.finish()
            }
            Self::Cornflowerblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Cornflowerblue",
                    17usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Cornsilk => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Cornsilk", 18usize, 0usize)?;
                serializer.finish()
            }
            Self::Crimson => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Crimson", 19usize, 0usize)?;
                serializer.finish()
            }
            Self::Cyan => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Cyan", 20usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkblue", 21usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkcyan => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkcyan", 22usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkgoldenrod => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgoldenrod", 23usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkgray => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgray", 24usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkgreen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgreen", 25usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkgrey => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgrey", 26usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkkhaki => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkkhaki", 27usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkmagenta => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkmagenta", 28usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkolivegreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Darkolivegreen",
                    29usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkorange => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkorange", 30usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkorchid => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkorchid", 31usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkred => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkred", 32usize, 0usize)?;
                serializer.finish()
            }
            Self::Darksalmon => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darksalmon", 33usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkseagreen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkseagreen", 34usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkslateblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkslateblue", 35usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkslategray => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkslategray", 36usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkslategrey => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkslategrey", 37usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkturquoise => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkturquoise", 38usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkviolet => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkviolet", 39usize, 0usize)?;
                serializer.finish()
            }
            Self::Deeppink => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Deeppink", 40usize, 0usize)?;
                serializer.finish()
            }
            Self::Deepskyblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Deepskyblue", 41usize, 0usize)?;
                serializer.finish()
            }
            Self::Dimgray => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Dimgray", 42usize, 0usize)?;
                serializer.finish()
            }
            Self::Dimgrey => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Dimgrey", 43usize, 0usize)?;
                serializer.finish()
            }
            Self::Dodgerblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Dodgerblue", 44usize, 0usize)?;
                serializer.finish()
            }
            Self::Firebrick => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Firebrick", 45usize, 0usize)?;
                serializer.finish()
            }
            Self::Floralwhite => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Floralwhite", 46usize, 0usize)?;
                serializer.finish()
            }
            Self::Forestgreen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Forestgreen", 47usize, 0usize)?;
                serializer.finish()
            }
            Self::Fuchsia => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Fuchsia", 48usize, 0usize)?;
                serializer.finish()
            }
            Self::Gainsboro => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Gainsboro", 49usize, 0usize)?;
                serializer.finish()
            }
            Self::Ghostwhite => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Ghostwhite", 50usize, 0usize)?;
                serializer.finish()
            }
            Self::Gold => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Gold", 51usize, 0usize)?;
                serializer.finish()
            }
            Self::Goldenrod => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Goldenrod", 52usize, 0usize)?;
                serializer.finish()
            }
            Self::Gray => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Gray", 53usize, 0usize)?;
                serializer.finish()
            }
            Self::Grey => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Grey", 54usize, 0usize)?;
                serializer.finish()
            }
            Self::Green => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Green", 55usize, 0usize)?;
                serializer.finish()
            }
            Self::Greenyellow => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Greenyellow", 56usize, 0usize)?;
                serializer.finish()
            }
            Self::Honeydew => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Honeydew", 57usize, 0usize)?;
                serializer.finish()
            }
            Self::Hotpink => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Hotpink", 58usize, 0usize)?;
                serializer.finish()
            }
            Self::Indianred => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Indianred", 59usize, 0usize)?;
                serializer.finish()
            }
            Self::Indigo => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Indigo", 60usize, 0usize)?;
                serializer.finish()
            }
            Self::Ivory => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Ivory", 61usize, 0usize)?;
                serializer.finish()
            }
            Self::Khaki => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Khaki", 62usize, 0usize)?;
                serializer.finish()
            }
            Self::Lavender => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lavender", 63usize, 0usize)?;
                serializer.finish()
            }
            Self::Lavenderblush => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lavenderblush", 64usize, 0usize)?;
                serializer.finish()
            }
            Self::Lawngreen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lawngreen", 65usize, 0usize)?;
                serializer.finish()
            }
            Self::Lemonchiffon => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lemonchiffon", 66usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightblue", 67usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightcoral => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightcoral", 68usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightcyan => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightcyan", 69usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightgoldenrodyellow => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightgoldenrodyellow",
                    70usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightgray => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightgray", 71usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightgreen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightgreen", 72usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightgrey => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightgrey", 73usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightpink => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightpink", 74usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightsalmon => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightsalmon", 75usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightseagreen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightseagreen", 76usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightskyblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightskyblue", 77usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightslategray => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightslategray",
                    78usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightslategrey => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightslategrey",
                    79usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightsteelblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightsteelblue",
                    80usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightyellow => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightyellow", 81usize, 0usize)?;
                serializer.finish()
            }
            Self::Lime => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Lime", 82usize, 0usize)?;
                serializer.finish()
            }
            Self::Limegreen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Limegreen", 83usize, 0usize)?;
                serializer.finish()
            }
            Self::Linen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Linen", 84usize, 0usize)?;
                serializer.finish()
            }
            Self::Magenta => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Magenta", 85usize, 0usize)?;
                serializer.finish()
            }
            Self::Maroon => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Maroon", 86usize, 0usize)?;
                serializer.finish()
            }
            Self::Mediumaquamarine => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumaquamarine",
                    87usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Mediumblue", 88usize, 0usize)?;
                serializer.finish()
            }
            Self::Mediumorchid => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Mediumorchid", 89usize, 0usize)?;
                serializer.finish()
            }
            Self::Mediumpurple => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Mediumpurple", 90usize, 0usize)?;
                serializer.finish()
            }
            Self::Mediumseagreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumseagreen",
                    91usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumslateblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumslateblue",
                    92usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumspringgreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumspringgreen",
                    93usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumturquoise => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumturquoise",
                    94usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumvioletred => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumvioletred",
                    95usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Midnightblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Midnightblue", 96usize, 0usize)?;
                serializer.finish()
            }
            Self::Mintcream => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Mintcream", 97usize, 0usize)?;
                serializer.finish()
            }
            Self::Mistyrose => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Mistyrose", 98usize, 0usize)?;
                serializer.finish()
            }
            Self::Moccasin => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Moccasin", 99usize, 0usize)?;
                serializer.finish()
            }
            Self::Navajowhite => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Navajowhite", 100usize, 0usize)?;
                serializer.finish()
            }
            Self::Navy => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Navy", 101usize, 0usize)?;
                serializer.finish()
            }
            Self::Oldlace => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Oldlace", 102usize, 0usize)?;
                serializer.finish()
            }
            Self::Olive => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Olive", 103usize, 0usize)?;
                serializer.finish()
            }
            Self::Olivedrab => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Olivedrab", 104usize, 0usize)?;
                serializer.finish()
            }
            Self::Orange => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Orange", 105usize, 0usize)?;
                serializer.finish()
            }
            Self::Orangered => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Orangered", 106usize, 0usize)?;
                serializer.finish()
            }
            Self::Orchid => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Orchid", 107usize, 0usize)?;
                serializer.finish()
            }
            Self::Palegoldenrod => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Palegoldenrod",
                    108usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Palegreen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Palegreen", 109usize, 0usize)?;
                serializer.finish()
            }
            Self::Paleturquoise => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Paleturquoise",
                    110usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Palevioletred => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Palevioletred",
                    111usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Papayawhip => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Papayawhip", 112usize, 0usize)?;
                serializer.finish()
            }
            Self::Peachpuff => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Peachpuff", 113usize, 0usize)?;
                serializer.finish()
            }
            Self::Peru => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Peru", 114usize, 0usize)?;
                serializer.finish()
            }
            Self::Pink => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Pink", 115usize, 0usize)?;
                serializer.finish()
            }
            Self::Plum => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Plum", 116usize, 0usize)?;
                serializer.finish()
            }
            Self::Powderblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Powderblue", 117usize, 0usize)?;
                serializer.finish()
            }
            Self::Purple => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Purple", 118usize, 0usize)?;
                serializer.finish()
            }
            Self::Red => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Red", 119usize, 0usize)?;
                serializer.finish()
            }
            Self::Rosybrown => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Rosybrown", 120usize, 0usize)?;
                serializer.finish()
            }
            Self::Royalblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Royalblue", 121usize, 0usize)?;
                serializer.finish()
            }
            Self::Saddlebrown => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Saddlebrown", 122usize, 0usize)?;
                serializer.finish()
            }
            Self::Salmon => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Salmon", 123usize, 0usize)?;
                serializer.finish()
            }
            Self::Sandybrown => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Sandybrown", 124usize, 0usize)?;
                serializer.finish()
            }
            Self::Seagreen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Seagreen", 125usize, 0usize)?;
                serializer.finish()
            }
            Self::Seashell => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Seashell", 126usize, 0usize)?;
                serializer.finish()
            }
            Self::Sienna => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Sienna", 127usize, 0usize)?;
                serializer.finish()
            }
            Self::Silver => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Silver", 128usize, 0usize)?;
                serializer.finish()
            }
            Self::Skyblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Skyblue", 129usize, 0usize)?;
                serializer.finish()
            }
            Self::Slateblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Slateblue", 130usize, 0usize)?;
                serializer.finish()
            }
            Self::Slategray => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Slategray", 131usize, 0usize)?;
                serializer.finish()
            }
            Self::Slategrey => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Slategrey", 132usize, 0usize)?;
                serializer.finish()
            }
            Self::Snow => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Snow", 133usize, 0usize)?;
                serializer.finish()
            }
            Self::Springgreen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Springgreen", 134usize, 0usize)?;
                serializer.finish()
            }
            Self::Steelblue => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Steelblue", 135usize, 0usize)?;
                serializer.finish()
            }
            Self::Tan => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Tan", 136usize, 0usize)?;
                serializer.finish()
            }
            Self::Teal => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Teal", 137usize, 0usize)?;
                serializer.finish()
            }
            Self::Thistle => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Thistle", 138usize, 0usize)?;
                serializer.finish()
            }
            Self::Tomato => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Tomato", 139usize, 0usize)?;
                serializer.finish()
            }
            Self::Turquoise => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Turquoise", 140usize, 0usize)?;
                serializer.finish()
            }
            Self::Violet => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Violet", 141usize, 0usize)?;
                serializer.finish()
            }
            Self::Wheat => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Wheat", 142usize, 0usize)?;
                serializer.finish()
            }
            Self::White => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "White", 143usize, 0usize)?;
                serializer.finish()
            }
            Self::Whitesmoke => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Whitesmoke", 144usize, 0usize)?;
                serializer.finish()
            }
            Self::Yellow => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Yellow", 145usize, 0usize)?;
                serializer.finish()
            }
            Self::Yellowgreen => {
                let serializer =
                    serializer.serialize_enum(2usize, "Color", "Yellowgreen", 146usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(3usize, "Rgb", 3usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        serializer.serialize_field(2usize, None, &self.2)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Iri {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Local(p0) => {
                let mut serializer =
                    serializer.serialize_enum(4usize, "Iri", "Local", 0usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Path(p0) => {
                let mut serializer =
                    serializer.serialize_enum(4usize, "Iri", "Path", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FuncIri {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(5usize, "FuncIri", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Point {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(6usize, "Point", 2usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Percent {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(7usize, "Percent", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Paint {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::None => {
                let serializer =
                    serializer.serialize_enum(8usize, "Paint", "None", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Color(p0) => {
                let mut serializer =
                    serializer.serialize_enum(8usize, "Paint", "Color", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Server(p0) => {
                let mut serializer =
                    serializer.serialize_enum(8usize, "Paint", "Server", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::NumberOptNumber {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(9usize, "NumberOptNumber", 2usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Coords {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::UserSpaceOnUse => {
                let serializer = serializer.serialize_enum(
                    10usize,
                    "Coords",
                    "UserSpaceOnUse",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ObjectBoundingBox => {
                let serializer = serializer.serialize_enum(
                    10usize,
                    "Coords",
                    "ObjectBoundingBox",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Transform {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Translate(p0, p1) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "Translate", 0usize, 2usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.finish()
            }
            Self::Matrix(p0) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "Matrix", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Scale(p0, p1) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "Scale", 2usize, 2usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.finish()
            }
            Self::Rotate { angle, center } => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "Rotate", 3usize, 2usize)?;
                serializer.serialize_field(0usize, Some("angle"), angle)?;
                serializer.serialize_field(1usize, Some("center"), center)?;
                serializer.finish()
            }
            Self::SkewX(p0) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "SkewX", 4usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::SkewY(p0) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "SkewY", 5usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Channel {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::R => {
                let serializer =
                    serializer.serialize_enum(12usize, "Channel", "R", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::G => {
                let serializer =
                    serializer.serialize_enum(12usize, "Channel", "G", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::B => {
                let serializer =
                    serializer.serialize_enum(12usize, "Channel", "B", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::A => {
                let serializer =
                    serializer.serialize_enum(12usize, "Channel", "A", 3usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::ClipRule {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Nonzero => {
                let serializer =
                    serializer.serialize_enum(13usize, "ClipRule", "Nonzero", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::EvenOdd => {
                let serializer =
                    serializer.serialize_enum(13usize, "ClipRule", "EvenOdd", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::PathEvent {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Close => {
                let serializer =
                    serializer.serialize_enum(14usize, "PathEvent", "Close", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::MoveTo(p0) => {
                let mut serializer =
                    serializer.serialize_enum(14usize, "PathEvent", "MoveTo", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::MoveToRelative(p0) => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "MoveToRelative",
                    2usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::LineTo(p0) => {
                let mut serializer =
                    serializer.serialize_enum(14usize, "PathEvent", "LineTo", 3usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::LineToRelative(p0) => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "LineToRelative",
                    4usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Polyline(p0) => {
                let mut serializer =
                    serializer.serialize_enum(14usize, "PathEvent", "Polyline", 5usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::PolylineRelative(p0) => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "PolylineRelative",
                    6usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::CubicBezier {
                ctrl1,
                ctrl2,
                to_point,
            } => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "CubicBezier",
                    7usize,
                    3usize,
                )?;
                serializer.serialize_field(0usize, Some("ctrl1"), ctrl1)?;
                serializer.serialize_field(1usize, Some("ctrl2"), ctrl2)?;
                serializer.serialize_field(2usize, Some("to_point"), to_point)?;
                serializer.finish()
            }
            Self::CubicBezierRelative {
                ctrl1,
                ctrl2,
                to_point,
            } => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "CubicBezierRelative",
                    8usize,
                    3usize,
                )?;
                serializer.serialize_field(0usize, Some("ctrl1"), ctrl1)?;
                serializer.serialize_field(1usize, Some("ctrl2"), ctrl2)?;
                serializer.serialize_field(2usize, Some("to_point"), to_point)?;
                serializer.finish()
            }
            Self::CubicBezierSmooth { ctrl2, to_point } => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "CubicBezierSmooth",
                    9usize,
                    2usize,
                )?;
                serializer.serialize_field(0usize, Some("ctrl2"), ctrl2)?;
                serializer.serialize_field(1usize, Some("to_point"), to_point)?;
                serializer.finish()
            }
            Self::CubicBezierSmoothRelative { ctrl2, to_point } => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "CubicBezierSmoothRelative",
                    10usize,
                    2usize,
                )?;
                serializer.serialize_field(0usize, Some("ctrl2"), ctrl2)?;
                serializer.serialize_field(1usize, Some("to_point"), to_point)?;
                serializer.finish()
            }
            Self::QuadraticBezier { ctrl, to_point } => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "QuadraticBezier",
                    11usize,
                    2usize,
                )?;
                serializer.serialize_field(0usize, Some("ctrl"), ctrl)?;
                serializer.serialize_field(1usize, Some("to_point"), to_point)?;
                serializer.finish()
            }
            Self::QuadraticBezierRelative { ctrl, to_point } => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "QuadraticBezierRelative",
                    12usize,
                    2usize,
                )?;
                serializer.serialize_field(0usize, Some("ctrl"), ctrl)?;
                serializer.serialize_field(1usize, Some("to_point"), to_point)?;
                serializer.finish()
            }
            Self::QuadraticBezierSmooth(p0) => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "QuadraticBezierSmooth",
                    13usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::QuadraticBezierSmoothRelative(p0) => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "QuadraticBezierSmoothRelative",
                    14usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Arc {
                rx,
                ry,
                x_rotation,
                large_arc,
                sweep,
                to_point,
            } => {
                let mut serializer =
                    serializer.serialize_enum(14usize, "PathEvent", "Arc", 15usize, 6usize)?;
                serializer.serialize_field(0usize, Some("rx"), rx)?;
                serializer.serialize_field(1usize, Some("ry"), ry)?;
                serializer.serialize_field(2usize, Some("x_rotation"), x_rotation)?;
                serializer.serialize_field(3usize, Some("large_arc"), large_arc)?;
                serializer.serialize_field(4usize, Some("sweep"), sweep)?;
                serializer.serialize_field(5usize, Some("to_point"), to_point)?;
                serializer.finish()
            }
            Self::ArcRelative {
                rx,
                ry,
                x_rotation,
                large_arc,
                sweep,
                to_point,
            } => {
                let mut serializer = serializer.serialize_enum(
                    14usize,
                    "PathEvent",
                    "ArcRelative",
                    16usize,
                    6usize,
                )?;
                serializer.serialize_field(0usize, Some("rx"), rx)?;
                serializer.serialize_field(1usize, Some("ry"), ry)?;
                serializer.serialize_field(2usize, Some("x_rotation"), x_rotation)?;
                serializer.serialize_field(3usize, Some("large_arc"), large_arc)?;
                serializer.serialize_field(4usize, Some("sweep"), sweep)?;
                serializer.serialize_field(5usize, Some("to_point"), to_point)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FillRule {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Nonzero => {
                let serializer =
                    serializer.serialize_enum(15usize, "FillRule", "Nonzero", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::EvenOdd => {
                let serializer =
                    serializer.serialize_enum(15usize, "FillRule", "EvenOdd", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::StrokeLineCap {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Butt => {
                let serializer =
                    serializer.serialize_enum(16usize, "stroke-linecap", "Butt", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Round => {
                let serializer = serializer.serialize_enum(
                    16usize,
                    "stroke-linecap",
                    "Round",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Square => {
                let serializer = serializer.serialize_enum(
                    16usize,
                    "stroke-linecap",
                    "Square",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::StrokeLineJoin {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Miter(p0) => {
                let mut serializer = serializer.serialize_enum(
                    17usize,
                    "stroke-linejoin",
                    "Miter",
                    0usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Round => {
                let serializer = serializer.serialize_enum(
                    17usize,
                    "stroke-linejoin",
                    "Round",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Bevel => {
                let serializer = serializer.serialize_enum(
                    17usize,
                    "stroke-linejoin",
                    "Bevel",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::SpreadMethod {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Pad => {
                let serializer =
                    serializer.serialize_enum(18usize, "SpreadMethod", "Pad", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Reflect => {
                let serializer = serializer.serialize_enum(
                    18usize,
                    "SpreadMethod",
                    "Reflect",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Repeat => {
                let serializer =
                    serializer.serialize_enum(18usize, "SpreadMethod", "Repeat", 2usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FontStyle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(19usize, "FontStyle", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Italic => {
                let serializer =
                    serializer.serialize_enum(19usize, "FontStyle", "Italic", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::Oblique => {
                let serializer =
                    serializer.serialize_enum(19usize, "FontStyle", "Oblique", 2usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FontVariant {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(20usize, "FontVariant", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::SmallCaps => {
                let serializer = serializer.serialize_enum(
                    20usize,
                    "FontVariant",
                    "SmallCaps",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FontWeight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Bold => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Bold", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::Bolder => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Bolder", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Lighter => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Lighter", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::W100 => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W100", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::W200 => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W200", 5usize, 0usize)?;
                serializer.finish()
            }
            Self::W300 => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W300", 6usize, 0usize)?;
                serializer.finish()
            }
            Self::W400 => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W400", 7usize, 0usize)?;
                serializer.finish()
            }
            Self::W500 => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W500", 8usize, 0usize)?;
                serializer.finish()
            }
            Self::W600 => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W600", 9usize, 0usize)?;
                serializer.finish()
            }
            Self::W700 => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W700", 10usize, 0usize)?;
                serializer.finish()
            }
            Self::W800 => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W800", 11usize, 0usize)?;
                serializer.finish()
            }
            Self::W900 => {
                let serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W900", 12usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FontFamily {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Serif => {
                let serializer =
                    serializer.serialize_enum(22usize, "FontFamily", "Serif", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::SansSerif => {
                let serializer = serializer.serialize_enum(
                    22usize,
                    "FontFamily",
                    "SansSerif",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Cursive => {
                let serializer =
                    serializer.serialize_enum(22usize, "FontFamily", "Cursive", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Fantasy => {
                let serializer =
                    serializer.serialize_enum(22usize, "FontFamily", "Fantasy", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::Monospace => {
                let serializer = serializer.serialize_enum(
                    22usize,
                    "FontFamily",
                    "Monospace",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Generic(p0) => {
                let mut serializer =
                    serializer.serialize_enum(22usize, "FontFamily", "Generic", 5usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FontStretch {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(23usize, "FontStretch", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Wider => {
                let serializer =
                    serializer.serialize_enum(23usize, "FontStretch", "Wider", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::Narrower => {
                let serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "Narrower",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::UltraCondensed => {
                let serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "UltraCondensed",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ExtraCondensed => {
                let serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "ExtraCondensed",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Condensed => {
                let serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "Condensed",
                    5usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SemiCondensed => {
                let serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "SemiCondensed",
                    6usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SemiExpanded => {
                let serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "SemiExpanded",
                    7usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Expanded => {
                let serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "Expanded",
                    8usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ExtraExpanded => {
                let serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "ExtraExpanded",
                    9usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::UltraExpanded => {
                let serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "UltraExpanded",
                    10usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Background {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Accumulate => {
                let serializer = serializer.serialize_enum(
                    24usize,
                    "Background",
                    "Accumulate",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::New(p0) => {
                let mut serializer =
                    serializer.serialize_enum(24usize, "Background", "New", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::BackgroundNew {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(25usize, "BackgroundNew", 4usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeIn {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::SourceGraphic => {
                let serializer =
                    serializer.serialize_enum(26usize, "FeIn", "SourceGraphic", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::SourceAlpha => {
                let serializer =
                    serializer.serialize_enum(26usize, "FeIn", "SourceAlpha", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::BackgroundImage => {
                let serializer = serializer.serialize_enum(
                    26usize,
                    "FeIn",
                    "BackgroundImage",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::BackgroundAlpha => {
                let serializer = serializer.serialize_enum(
                    26usize,
                    "FeIn",
                    "BackgroundAlpha",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::FillPaint => {
                let serializer =
                    serializer.serialize_enum(26usize, "FeIn", "FillPaint", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::StrokePaint => {
                let serializer =
                    serializer.serialize_enum(26usize, "FeIn", "StrokePaint", 5usize, 0usize)?;
                serializer.finish()
            }
            Self::Result(p0) => {
                let mut serializer =
                    serializer.serialize_enum(26usize, "FeIn", "Result", 6usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeOut {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Position => {
                let serializer =
                    serializer.serialize_enum(27usize, "FeOut", "Position", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Named(p0) => {
                let mut serializer =
                    serializer.serialize_enum(27usize, "FeOut", "Named", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeBlendMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(28usize, "FeBlendMode", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Multiply => {
                let serializer = serializer.serialize_enum(
                    28usize,
                    "FeBlendMode",
                    "Multiply",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Screen => {
                let serializer =
                    serializer.serialize_enum(28usize, "FeBlendMode", "Screen", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Darken => {
                let serializer =
                    serializer.serialize_enum(28usize, "FeBlendMode", "Darken", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::Lighten => {
                let serializer =
                    serializer.serialize_enum(28usize, "FeBlendMode", "Lighten", 4usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::TextLengthAdjust {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Spacing => {
                let serializer = serializer.serialize_enum(
                    29usize,
                    "TextLengthAdjust",
                    "Spacing",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SpacingAndGlyphs => {
                let serializer = serializer.serialize_enum(
                    29usize,
                    "TextLengthAdjust",
                    "SpacingAndGlyphs",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::WritingMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::LrTb => {
                let serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "LrTb", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::RlTb => {
                let serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "RlTb", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::TbRl => {
                let serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "TbRl", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Lr => {
                let serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "Lr", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::Rl => {
                let serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "Rl", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::Tb => {
                let serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "Tb", 5usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::TextDirection {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Ltr => {
                let serializer =
                    serializer.serialize_enum(31usize, "TextDirection", "Ltr", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Rtl => {
                let serializer =
                    serializer.serialize_enum(31usize, "TextDirection", "Rtl", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::UnicodeBidi {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(32usize, "UnicodeBidi", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Embed => {
                let serializer =
                    serializer.serialize_enum(32usize, "UnicodeBidi", "Embed", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::BidiOverride => {
                let serializer = serializer.serialize_enum(
                    32usize,
                    "UnicodeBidi",
                    "BidiOverride",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::TextAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Start => {
                let serializer =
                    serializer.serialize_enum(33usize, "TextAnchor", "Start", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Middle => {
                let serializer =
                    serializer.serialize_enum(33usize, "TextAnchor", "Middle", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::End => {
                let serializer =
                    serializer.serialize_enum(33usize, "TextAnchor", "End", 2usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::DominantBaseline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Auto => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Auto",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::UseScript => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "UseScript",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::NoChange => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "NoChange",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ResetSize => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "ResetSize",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Ideographic => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Ideographic",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Alphabetic => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Alphabetic",
                    5usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Hanging => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Hanging",
                    6usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mathematical => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Mathematical",
                    7usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Central => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Central",
                    8usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Middle => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Middle",
                    9usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextAfterEdge => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "TextAfterEdge",
                    10usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextBeforeEdge => {
                let serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "TextBeforeEdge",
                    11usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::AlignmentBaseline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Auto => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Auto",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Baseline => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Baseline",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::BeforeEdge => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "BeforeEdge",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextBeforeEdge => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "TextBeforeEdge",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Middle => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Middle",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Central => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Central",
                    5usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::AfterEdge => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "AfterEdge",
                    6usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextAfterEdge => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "TextAfterEdge",
                    7usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Ideographic => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Ideographic",
                    8usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Alphabetic => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Alphabetic",
                    9usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Hanging => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Hanging",
                    10usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mathematical => {
                let serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Mathematical",
                    11usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::BaselineShift {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Baseline => {
                let serializer = serializer.serialize_enum(
                    36usize,
                    "BaselineShift",
                    "Baseline",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SubScripts => {
                let serializer = serializer.serialize_enum(
                    36usize,
                    "BaselineShift",
                    "SubScripts",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SuperScripts => {
                let serializer = serializer.serialize_enum(
                    36usize,
                    "BaselineShift",
                    "SuperScripts",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Value(p0) => {
                let mut serializer =
                    serializer.serialize_enum(36usize, "BaselineShift", "Value", 3usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::TextDecoration {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Underline => {
                let serializer = serializer.serialize_enum(
                    37usize,
                    "TextDecoration",
                    "Underline",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Overline => {
                let serializer = serializer.serialize_enum(
                    37usize,
                    "TextDecoration",
                    "Overline",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::LineThrough => {
                let serializer = serializer.serialize_enum(
                    37usize,
                    "TextDecoration",
                    "LineThrough",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Blink => {
                let serializer = serializer.serialize_enum(
                    37usize,
                    "TextDecoration",
                    "Blink",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::TextPathMethod {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Align => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "TextPathMethod",
                    "Align",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Stretch => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "TextPathMethod",
                    "Stretch",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::TextPathSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Auto => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "TextPathSpacing",
                    "Auto",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Exact => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "TextPathSpacing",
                    "Exact",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::LetterSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer = serializer.serialize_enum(
                    40usize,
                    "LetterSpacing",
                    "Normal",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Length(p0) => {
                let mut serializer = serializer.serialize_enum(
                    40usize,
                    "LetterSpacing",
                    "Length",
                    1usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::WordSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(41usize, "WordSpacing", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Length(p0) => {
                let mut serializer =
                    serializer.serialize_enum(41usize, "WordSpacing", "Length", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::MeetOrSlice {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Meet => {
                let serializer =
                    serializer.serialize_enum(42usize, "MeetOrSlice", "Meet", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Slice => {
                let serializer =
                    serializer.serialize_enum(42usize, "MeetOrSlice", "Slice", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::PreserveAspectRatio {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::None => {
                let serializer = serializer.serialize_enum(
                    43usize,
                    "PreserveAspectRatio",
                    "None",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::XMinYMin(p0) => {
                let mut serializer = serializer.serialize_enum(
                    43usize,
                    "PreserveAspectRatio",
                    "XMinYMin",
                    1usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMidYMin(p0) => {
                let mut serializer = serializer.serialize_enum(
                    43usize,
                    "PreserveAspectRatio",
                    "XMidYMin",
                    2usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMaxYMin(p0) => {
                let mut serializer = serializer.serialize_enum(
                    43usize,
                    "PreserveAspectRatio",
                    "XMaxYMin",
                    3usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMinYMid(p0) => {
                let mut serializer = serializer.serialize_enum(
                    43usize,
                    "PreserveAspectRatio",
                    "XMinYMid",
                    4usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMidYMid(p0) => {
                let mut serializer = serializer.serialize_enum(
                    43usize,
                    "PreserveAspectRatio",
                    "XMidYMid",
                    5usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMaxYMid(p0) => {
                let mut serializer = serializer.serialize_enum(
                    43usize,
                    "PreserveAspectRatio",
                    "XMaxYMid",
                    6usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMinYMax(p0) => {
                let mut serializer = serializer.serialize_enum(
                    43usize,
                    "PreserveAspectRatio",
                    "XMinYMax",
                    7usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMidYMax(p0) => {
                let mut serializer = serializer.serialize_enum(
                    43usize,
                    "PreserveAspectRatio",
                    "XMidYMax",
                    8usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMaxYMax(p0) => {
                let mut serializer = serializer.serialize_enum(
                    43usize,
                    "PreserveAspectRatio",
                    "XMaxYMax",
                    9usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::TextLayout {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(44usize, "TextLayout", 10usize)?;
        serializer.serialize_field(0usize, Some("write_mode"), &self.write_mode)?;
        serializer.serialize_field(1usize, Some("direction"), &self.direction)?;
        serializer.serialize_field(2usize, Some("unicode_bidi"), &self.unicode_bidi)?;
        serializer.serialize_field(3usize, Some("text-anchor"), &self.anchor)?;
        serializer.serialize_field(4usize, Some("dominant_baseline"), &self.dominant_baseline)?;
        serializer.serialize_field(5usize, Some("alignment_baseline"), &self.alignment_baseline)?;
        serializer.serialize_field(6usize, Some("baseline_shift"), &self.baseline_shift)?;
        serializer.serialize_field(7usize, Some("text-decoration"), &self.decoration)?;
        serializer.serialize_field(8usize, Some("letter_spacing"), &self.letter_spacing)?;
        serializer.serialize_field(9usize, Some("word_spacing"), &self.word_spacing)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::WithTransform {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(45usize, "WithTransform", 1usize)?;
        serializer.serialize_field(0usize, Some("transform"), &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Id {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(46usize, "Id", 1usize)?;
        serializer.serialize_field(0usize, Some("id"), &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Fill {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(47usize, "Fill", 3usize)?;
        serializer.serialize_field(0usize, Some("fill"), &self.paint)?;
        serializer.serialize_field(1usize, Some("fill-rule"), &self.rule)?;
        serializer.serialize_field(2usize, Some("fill-opacity"), &self.opacity)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Stroke {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(48usize, "Stroke", 7usize)?;
        serializer.serialize_field(0usize, Some("stroke"), &self.paint)?;
        serializer.serialize_field(1usize, Some("stroke-width"), &self.width)?;
        serializer.serialize_field(2usize, Some("stroke-linecap"), &self.linecap)?;
        serializer.serialize_field(3usize, Some("stroke-linejoin"), &self.linejoin)?;
        serializer.serialize_field(4usize, Some("stroke-dasharray"), &self.dasharray)?;
        serializer.serialize_field(5usize, Some("stroke-dashoffset"), &self.dashoffset)?;
        serializer.serialize_field(6usize, Some("stroke-opacity"), &self.opacity)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Font {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(49usize, "Font", 6usize)?;
        serializer.serialize_field(0usize, Some("font-family"), &self.family)?;
        serializer.serialize_field(1usize, Some("font-style"), &self.style)?;
        serializer.serialize_field(2usize, Some("font-variant"), &self.variant)?;
        serializer.serialize_field(3usize, Some("font-weight"), &self.weight)?;
        serializer.serialize_field(4usize, Some("font-size"), &self.size)?;
        serializer.serialize_field(5usize, Some("font-stretch"), &self.stretch)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::EnableBackground {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(50usize, "EnableBackground", 1usize)?;
        serializer.serialize_field(0usize, Some("enable-background"), &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::WithFilter {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(51usize, "WithFilter", 1usize)?;
        serializer.serialize_field(0usize, Some("filter"), &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::WithClipPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(52usize, "WithClipPath", 1usize)?;
        serializer.serialize_field(0usize, Some("clip-path"), &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::WithMask {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(53usize, "WithMask", 1usize)?;
        serializer.serialize_field(0usize, Some("mask"), &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Opacity {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(54usize, "Opacity", 1usize)?;
        serializer.serialize_field(0usize, Some("opacity"), &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::ViewBox {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(55usize, "ViewBox", 5usize)?;
        serializer.serialize_field(0usize, Some("minx"), &self.minx)?;
        serializer.serialize_field(1usize, Some("miny"), &self.miny)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("aspect"), &self.aspect)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Canvas {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(57usize, "svg", 2usize)?;
        serializer.serialize_field(0usize, Some("width"), &self.width)?;
        serializer.serialize_field(1usize, Some("height"), &self.height)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Mask {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(58usize, "Mask", 6usize)?;
        serializer.serialize_field(0usize, Some("maskUnits"), &self.units)?;
        serializer.serialize_field(1usize, Some("content_units"), &self.content_units)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::ClipPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(59usize, "ClipPath", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Filter {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(60usize, "Filter", 7usize)?;
        serializer.serialize_field(0usize, Some("filterUnits"), &self.units)?;
        serializer.serialize_field(1usize, Some("primitive_units"), &self.primitive_units)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("res"), &self.res)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeDistantLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(61usize, "FeDistantLight", 2usize)?;
        serializer.serialize_field(0usize, Some("azimuth"), &self.azimuth)?;
        serializer.serialize_field(1usize, Some("elevation"), &self.elevation)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FePointLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(62usize, "FePointLight", 3usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("z"), &self.z)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeSpotLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(63usize, "FeSpotLight", 8usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("z"), &self.z)?;
        serializer.serialize_field(3usize, Some("point_at_x"), &self.point_at_x)?;
        serializer.serialize_field(4usize, Some("point_at_y"), &self.point_at_y)?;
        serializer.serialize_field(5usize, Some("point_at_z"), &self.point_at_z)?;
        serializer.serialize_field(6usize, Some("specular_exponent"), &self.specular_exponent)?;
        serializer.serialize_field(
            7usize,
            Some("limiting_cone_angle"),
            &self.limiting_cone_angle,
        )?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeBlend {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(64usize, "FeBlend", 8usize)?;
        serializer.serialize_field(0usize, Some("mode"), &self.mode)?;
        serializer.serialize_field(1usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(2usize, Some("in2"), &self.in2)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeColorMatrixValues {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Matrix(p0) => {
                let mut serializer = serializer.serialize_enum(
                    65usize,
                    "FeColorMatrixValues",
                    "Matrix",
                    0usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Saturate(p0) => {
                let mut serializer = serializer.serialize_enum(
                    65usize,
                    "FeColorMatrixValues",
                    "Saturate",
                    1usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::HueRotate(p0) => {
                let mut serializer = serializer.serialize_enum(
                    65usize,
                    "FeColorMatrixValues",
                    "HueRotate",
                    2usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::LuminanceToAlpha => {
                let serializer = serializer.serialize_enum(
                    65usize,
                    "FeColorMatrixValues",
                    "LuminanceToAlpha",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeColorMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(66usize, "FeColorMatrix", 7usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("values"), &self.values)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeFunc {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Identity => {
                let serializer =
                    serializer.serialize_enum(67usize, "FeFunc", "Identity", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Table(p0) => {
                let mut serializer =
                    serializer.serialize_enum(67usize, "FeFunc", "Table", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Discrete(p0) => {
                let mut serializer =
                    serializer.serialize_enum(67usize, "FeFunc", "Discrete", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Linear { slope, intercept } => {
                let mut serializer =
                    serializer.serialize_enum(67usize, "FeFunc", "Linear", 3usize, 2usize)?;
                serializer.serialize_field(0usize, Some("slope"), slope)?;
                serializer.serialize_field(1usize, Some("intercept"), intercept)?;
                serializer.finish()
            }
            Self::Gamma {
                amplitude,
                exponent,
                offset,
            } => {
                let mut serializer =
                    serializer.serialize_enum(67usize, "FeFunc", "Gamma", 4usize, 3usize)?;
                serializer.serialize_field(0usize, Some("amplitude"), amplitude)?;
                serializer.serialize_field(1usize, Some("exponent"), exponent)?;
                serializer.serialize_field(2usize, Some("offset"), offset)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeCompositeOperator {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Over => {
                let serializer = serializer.serialize_enum(
                    68usize,
                    "FeCompositeOperator",
                    "Over",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::In => {
                let serializer = serializer.serialize_enum(
                    68usize,
                    "FeCompositeOperator",
                    "In",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Out => {
                let serializer = serializer.serialize_enum(
                    68usize,
                    "FeCompositeOperator",
                    "Out",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Atop => {
                let serializer = serializer.serialize_enum(
                    68usize,
                    "FeCompositeOperator",
                    "Atop",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Xor => {
                let serializer = serializer.serialize_enum(
                    68usize,
                    "FeCompositeOperator",
                    "Xor",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Arithmetic { k1, k2, k3, k4 } => {
                let mut serializer = serializer.serialize_enum(
                    68usize,
                    "FeCompositeOperator",
                    "Arithmetic",
                    5usize,
                    4usize,
                )?;
                serializer.serialize_field(0usize, Some("k1"), k1)?;
                serializer.serialize_field(1usize, Some("k2"), k2)?;
                serializer.serialize_field(2usize, Some("k3"), k3)?;
                serializer.serialize_field(3usize, Some("k4"), k4)?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeConvolveMatrixEdgeMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Duplicate => {
                let serializer = serializer.serialize_enum(
                    69usize,
                    "FeConvolveMatrixEdgeMode",
                    "Duplicate",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Wrap => {
                let serializer = serializer.serialize_enum(
                    69usize,
                    "FeConvolveMatrixEdgeMode",
                    "Wrap",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::None => {
                let serializer = serializer.serialize_enum(
                    69usize,
                    "FeConvolveMatrixEdgeMode",
                    "None",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeMorphologyOperator {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Erode => {
                let serializer = serializer.serialize_enum(
                    70usize,
                    "FeMorphologyOperator",
                    "Erode",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Dilate => {
                let serializer = serializer.serialize_enum(
                    70usize,
                    "FeMorphologyOperator",
                    "Dilate",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeStitchTiles {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::Stitch => {
                let serializer = serializer.serialize_enum(
                    71usize,
                    "FeStitchTiles",
                    "Stitch",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::NoStitch => {
                let serializer = serializer.serialize_enum(
                    71usize,
                    "FeStitchTiles",
                    "NoStitch",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeTurbulenceType {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        match self {
            Self::FractalNoise => {
                let serializer = serializer.serialize_enum(
                    72usize,
                    "FeTurbulenceType",
                    "FractalNoise",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Turbulence => {
                let serializer = serializer.serialize_enum(
                    72usize,
                    "FeTurbulenceType",
                    "Turbulence",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeComponentTransfer {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(73usize, "FeComponentTransfer", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeFuncA {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(74usize, "FeFuncA", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeFuncR {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(75usize, "FeFuncR", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeFuncG {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(76usize, "FeFuncG", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeFuncB {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(77usize, "FeFuncB", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeComposite {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(78usize, "FeComposite", 8usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("in2"), &self.in2)?;
        serializer.serialize_field(2usize, Some("operator"), &self.operator)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeConvolveMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(79usize, "FeConvolveMatrix", 15usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("order"), &self.order)?;
        serializer.serialize_field(2usize, Some("kernel"), &self.kernel)?;
        serializer.serialize_field(3usize, Some("divisor"), &self.divisor)?;
        serializer.serialize_field(4usize, Some("bias"), &self.bias)?;
        serializer.serialize_field(5usize, Some("target_x"), &self.target_x)?;
        serializer.serialize_field(6usize, Some("target_y"), &self.target_y)?;
        serializer.serialize_field(7usize, Some("edge_mode"), &self.edge_mode)?;
        serializer.serialize_field(8usize, Some("kernel_unit_len"), &self.kernel_unit_len)?;
        serializer.serialize_field(9usize, Some("preserve_alpha"), &self.preserve_alpha)?;
        serializer.serialize_field(10usize, Some("x"), &self.x)?;
        serializer.serialize_field(11usize, Some("y"), &self.y)?;
        serializer.serialize_field(12usize, Some("width"), &self.width)?;
        serializer.serialize_field(13usize, Some("height"), &self.height)?;
        serializer.serialize_field(14usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeDiffuseLighting {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(80usize, "FeDiffuseLighting", 9usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("surface_scale"), &self.surface_scale)?;
        serializer.serialize_field(2usize, Some("diffuse_constant"), &self.diffuse_constant)?;
        serializer.serialize_field(3usize, Some("kernel_unit_len"), &self.kernel_unit_len)?;
        serializer.serialize_field(4usize, Some("x"), &self.x)?;
        serializer.serialize_field(5usize, Some("y"), &self.y)?;
        serializer.serialize_field(6usize, Some("width"), &self.width)?;
        serializer.serialize_field(7usize, Some("height"), &self.height)?;
        serializer.serialize_field(8usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeDisplacementMap {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(81usize, "FeDisplacementMap", 10usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("in2"), &self.in2)?;
        serializer.serialize_field(2usize, Some("scale"), &self.scale)?;
        serializer.serialize_field(3usize, Some("x_channel_selector"), &self.x_channel_selector)?;
        serializer.serialize_field(4usize, Some("y_channel_selector"), &self.y_channel_selector)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeFlood {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(82usize, "FeFlood", 7usize)?;
        serializer.serialize_field(0usize, Some("color"), &self.color)?;
        serializer.serialize_field(1usize, Some("opacity"), &self.opacity)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeGaussianBlur {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(83usize, "FeGaussianBlur", 7usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("std_deviation"), &self.std_deviation)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeMerge {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(84usize, "FeMerge", 5usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeMergeNode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(85usize, "FeMergeNode", 1usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeImage {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(86usize, "FeImage", 7usize)?;
        serializer.serialize_field(0usize, Some("href"), &self.href)?;
        serializer.serialize_field(1usize, Some("aspect"), &self.aspect)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeMorphology {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(87usize, "FeMorphology", 8usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("mode"), &self.mode)?;
        serializer.serialize_field(2usize, Some("radius"), &self.radius)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeOffset {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(88usize, "FeOffset", 8usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("dx"), &self.dx)?;
        serializer.serialize_field(2usize, Some("dy"), &self.dy)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeSpecularLighting {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(89usize, "FeSpecularLighting", 10usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("surface_scale"), &self.surface_scale)?;
        serializer.serialize_field(2usize, Some("specular_constant"), &self.specular_constant)?;
        serializer.serialize_field(3usize, Some("specular_exponent"), &self.specular_exponent)?;
        serializer.serialize_field(4usize, Some("kernel_unit_len"), &self.kernel_unit_len)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeTile {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(90usize, "FeTile", 6usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("x"), &self.x)?;
        serializer.serialize_field(2usize, Some("y"), &self.y)?;
        serializer.serialize_field(3usize, Some("width"), &self.width)?;
        serializer.serialize_field(4usize, Some("height"), &self.height)?;
        serializer.serialize_field(5usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::FeTurbulence {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(91usize, "FeTurbulence", 10usize)?;
        serializer.serialize_field(0usize, Some("base_frequency"), &self.base_frequency)?;
        serializer.serialize_field(1usize, Some("num_octaves"), &self.num_octaves)?;
        serializer.serialize_field(2usize, Some("seed"), &self.seed)?;
        serializer.serialize_field(3usize, Some("stitch_tiles"), &self.stitch_tiles)?;
        serializer.serialize_field(4usize, Some("type"), &self.r#type)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::LinearGradient {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(92usize, "LinearGradient", 7usize)?;
        serializer.serialize_field(0usize, Some("gradientUnits"), &self.units)?;
        serializer.serialize_field(1usize, Some("transform"), &self.transform)?;
        serializer.serialize_field(2usize, Some("x1"), &self.x1)?;
        serializer.serialize_field(3usize, Some("y1"), &self.y1)?;
        serializer.serialize_field(4usize, Some("x2"), &self.x2)?;
        serializer.serialize_field(5usize, Some("y2"), &self.y2)?;
        serializer.serialize_field(6usize, Some("spread"), &self.spread)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::RadialGradient {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(93usize, "RadialGradient", 8usize)?;
        serializer.serialize_field(0usize, Some("gradientUnits"), &self.unit)?;
        serializer.serialize_field(1usize, Some("transform"), &self.transform)?;
        serializer.serialize_field(2usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(3usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(4usize, Some("r"), &self.r)?;
        serializer.serialize_field(5usize, Some("fx"), &self.fx)?;
        serializer.serialize_field(6usize, Some("fy"), &self.fy)?;
        serializer.serialize_field(7usize, Some("spread"), &self.spread)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::GradientStop {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(94usize, "stop", 3usize)?;
        serializer.serialize_field(0usize, Some("offset"), &self.offset)?;
        serializer.serialize_field(1usize, Some("stop-color"), &self.color)?;
        serializer.serialize_field(2usize, Some("stop-opacity"), &self.opacity)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Group {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let serializer = serializer.serialize_el(95usize, "g", 0usize)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Path {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(96usize, "Path", 2usize)?;
        serializer.serialize_field(0usize, Some("d"), &self.events)?;
        serializer.serialize_field(1usize, Some("length"), &self.length)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(97usize, "Pattern", 7usize)?;
        serializer.serialize_field(0usize, Some("patternUnits"), &self.units)?;
        serializer.serialize_field(1usize, Some("content_units"), &self.content_units)?;
        serializer.serialize_field(2usize, Some("transform"), &self.transform)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Use {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(98usize, "Use", 1usize)?;
        serializer.serialize_field(0usize, Some("xlink:href"), &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Rect {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(99usize, "Rect", 6usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("rx"), &self.rx)?;
        serializer.serialize_field(5usize, Some("ry"), &self.ry)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Circle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(100usize, "Circle", 3usize)?;
        serializer.serialize_field(0usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(1usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(2usize, Some("r"), &self.r)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Ellipse {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(101usize, "Ellipse", 4usize)?;
        serializer.serialize_field(0usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(1usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(2usize, Some("rx"), &self.rx)?;
        serializer.serialize_field(3usize, Some("ry"), &self.ry)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Line {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(102usize, "Line", 4usize)?;
        serializer.serialize_field(0usize, Some("x1"), &self.x1)?;
        serializer.serialize_field(1usize, Some("y1"), &self.y1)?;
        serializer.serialize_field(2usize, Some("x2"), &self.x2)?;
        serializer.serialize_field(3usize, Some("y2"), &self.y2)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Polyline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(103usize, "Polyline", 1usize)?;
        serializer.serialize_field(0usize, Some("points"), &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Polygon {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(104usize, "Polygon", 1usize)?;
        serializer.serialize_field(0usize, Some("points"), &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Text {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(106usize, "Text", 7usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("dx"), &self.dx)?;
        serializer.serialize_field(3usize, Some("dy"), &self.dy)?;
        serializer.serialize_field(4usize, Some("rotate"), &self.rotate)?;
        serializer.serialize_field(5usize, Some("text_length"), &self.text_length)?;
        serializer.serialize_field(6usize, Some("length_adjust"), &self.length_adjust)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::TextSpan {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(107usize, "tspan", 7usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("dx"), &self.dx)?;
        serializer.serialize_field(3usize, Some("dy"), &self.dy)?;
        serializer.serialize_field(4usize, Some("rotate"), &self.rotate)?;
        serializer.serialize_field(5usize, Some("text_length"), &self.text_length)?;
        serializer.serialize_field(6usize, Some("length_adjust"), &self.length_adjust)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Characters {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(108usize, "Characters", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::TextPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        use mlang::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(109usize, "TextPath", 4usize)?;
        serializer.serialize_field(0usize, Some("start_offset"), &self.start_offset)?;
        serializer.serialize_field(1usize, Some("method"), &self.method)?;
        serializer.serialize_field(2usize, Some("spacing"), &self.spacing)?;
        serializer.serialize_field(3usize, Some("href"), &self.href)?;
        serializer.finish()
    }
}
impl mlang::rt::serde::ser::Serialize for super::opcode::Opcode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang::rt::serde::ser::Serializer,
    {
        match self {
            Self::Apply(v) => match v {
                super::opcode::Attr::TextLayout(value) => value.serialize(serializer),
                super::opcode::Attr::WithTransform(value) => value.serialize(serializer),
                super::opcode::Attr::Id(value) => value.serialize(serializer),
                super::opcode::Attr::Fill(value) => value.serialize(serializer),
                super::opcode::Attr::Stroke(value) => value.serialize(serializer),
                super::opcode::Attr::Font(value) => value.serialize(serializer),
                super::opcode::Attr::EnableBackground(value) => value.serialize(serializer),
                super::opcode::Attr::WithFilter(value) => value.serialize(serializer),
                super::opcode::Attr::WithClipPath(value) => value.serialize(serializer),
                super::opcode::Attr::WithMask(value) => value.serialize(serializer),
                super::opcode::Attr::Opacity(value) => value.serialize(serializer),
                super::opcode::Attr::ViewBox(value) => value.serialize(serializer),
            },
            Self::Element(v) => match v {
                super::opcode::Element::Canvas(value) => value.serialize(serializer),
                super::opcode::Element::Mask(value) => value.serialize(serializer),
                super::opcode::Element::ClipPath(value) => value.serialize(serializer),
                super::opcode::Element::Filter(value) => value.serialize(serializer),
                super::opcode::Element::FeComponentTransfer(value) => value.serialize(serializer),
                super::opcode::Element::FeDiffuseLighting(value) => value.serialize(serializer),
                super::opcode::Element::FeMerge(value) => value.serialize(serializer),
                super::opcode::Element::FeSpecularLighting(value) => value.serialize(serializer),
                super::opcode::Element::LinearGradient(value) => value.serialize(serializer),
                super::opcode::Element::RadialGradient(value) => value.serialize(serializer),
                super::opcode::Element::Group(value) => value.serialize(serializer),
                super::opcode::Element::Pattern(value) => value.serialize(serializer),
                super::opcode::Element::Text(value) => value.serialize(serializer),
                super::opcode::Element::TextSpan(value) => value.serialize(serializer),
                super::opcode::Element::TextPath(value) => value.serialize(serializer),
            },
            Self::Leaf(v) => match v {
                super::opcode::Leaf::FeDistantLight(value) => value.serialize(serializer),
                super::opcode::Leaf::FePointLight(value) => value.serialize(serializer),
                super::opcode::Leaf::FeSpotLight(value) => value.serialize(serializer),
                super::opcode::Leaf::FeBlend(value) => value.serialize(serializer),
                super::opcode::Leaf::FeColorMatrix(value) => value.serialize(serializer),
                super::opcode::Leaf::FeFuncA(value) => value.serialize(serializer),
                super::opcode::Leaf::FeFuncR(value) => value.serialize(serializer),
                super::opcode::Leaf::FeFuncG(value) => value.serialize(serializer),
                super::opcode::Leaf::FeFuncB(value) => value.serialize(serializer),
                super::opcode::Leaf::FeComposite(value) => value.serialize(serializer),
                super::opcode::Leaf::FeConvolveMatrix(value) => value.serialize(serializer),
                super::opcode::Leaf::FeDisplacementMap(value) => value.serialize(serializer),
                super::opcode::Leaf::FeFlood(value) => value.serialize(serializer),
                super::opcode::Leaf::FeGaussianBlur(value) => value.serialize(serializer),
                super::opcode::Leaf::FeMergeNode(value) => value.serialize(serializer),
                super::opcode::Leaf::FeImage(value) => value.serialize(serializer),
                super::opcode::Leaf::FeMorphology(value) => value.serialize(serializer),
                super::opcode::Leaf::FeOffset(value) => value.serialize(serializer),
                super::opcode::Leaf::FeTile(value) => value.serialize(serializer),
                super::opcode::Leaf::FeTurbulence(value) => value.serialize(serializer),
                super::opcode::Leaf::GradientStop(value) => value.serialize(serializer),
                super::opcode::Leaf::Path(value) => value.serialize(serializer),
                super::opcode::Leaf::Use(value) => value.serialize(serializer),
                super::opcode::Leaf::Rect(value) => value.serialize(serializer),
                super::opcode::Leaf::Circle(value) => value.serialize(serializer),
                super::opcode::Leaf::Ellipse(value) => value.serialize(serializer),
                super::opcode::Leaf::Line(value) => value.serialize(serializer),
                super::opcode::Leaf::Polyline(value) => value.serialize(serializer),
                super::opcode::Leaf::Polygon(value) => value.serialize(serializer),
                super::opcode::Leaf::Characters(value) => value.serialize(serializer),
            },
            Self::Pop => serializer.serialize_pop(),
        }
    }
}
