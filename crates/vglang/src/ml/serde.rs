impl ml::rt::serde::ser::Serialize for super::opcode::Angle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl ml::rt::serde::ser::Serialize for super::opcode::Length {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl ml::rt::serde::ser::Serialize for super::opcode::Color {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Aliceblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Aliceblue", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Antiquewhite => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Antiquewhite", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::Aqua => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Aqua", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Aquamarine => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Aquamarine", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::Azure => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Azure", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::Beige => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Beige", 5usize, 0usize)?;
                serializer.finish()
            }
            Self::Bisque => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Bisque", 6usize, 0usize)?;
                serializer.finish()
            }
            Self::Black => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Black", 7usize, 0usize)?;
                serializer.finish()
            }
            Self::Blanchedalmond => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Blanchedalmond", 8usize, 0usize)?;
                serializer.finish()
            }
            Self::Blue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Blue", 9usize, 0usize)?;
                serializer.finish()
            }
            Self::Blueviolet => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Blueviolet", 10usize, 0usize)?;
                serializer.finish()
            }
            Self::Brown => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Brown", 11usize, 0usize)?;
                serializer.finish()
            }
            Self::Burlywood => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Burlywood", 12usize, 0usize)?;
                serializer.finish()
            }
            Self::Cadetblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Cadetblue", 13usize, 0usize)?;
                serializer.finish()
            }
            Self::Chartreuse => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Chartreuse", 14usize, 0usize)?;
                serializer.finish()
            }
            Self::Chocolate => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Chocolate", 15usize, 0usize)?;
                serializer.finish()
            }
            Self::Coral => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Coral", 16usize, 0usize)?;
                serializer.finish()
            }
            Self::Cornflowerblue => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Cornflowerblue",
                    17usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Cornsilk => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Cornsilk", 18usize, 0usize)?;
                serializer.finish()
            }
            Self::Crimson => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Crimson", 19usize, 0usize)?;
                serializer.finish()
            }
            Self::Cyan => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Cyan", 20usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkblue", 21usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkcyan => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkcyan", 22usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkgoldenrod => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgoldenrod", 23usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkgray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgray", 24usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkgreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgreen", 25usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkgrey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgrey", 26usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkkhaki => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkkhaki", 27usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkmagenta => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkmagenta", 28usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkolivegreen => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Darkolivegreen",
                    29usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkorange => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkorange", 30usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkorchid => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkorchid", 31usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkred => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkred", 32usize, 0usize)?;
                serializer.finish()
            }
            Self::Darksalmon => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darksalmon", 33usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkseagreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkseagreen", 34usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkslateblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkslateblue", 35usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkslategray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkslategray", 36usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkslategrey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkslategrey", 37usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkturquoise => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkturquoise", 38usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkviolet => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkviolet", 39usize, 0usize)?;
                serializer.finish()
            }
            Self::Deeppink => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Deeppink", 40usize, 0usize)?;
                serializer.finish()
            }
            Self::Deepskyblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Deepskyblue", 41usize, 0usize)?;
                serializer.finish()
            }
            Self::Dimgray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Dimgray", 42usize, 0usize)?;
                serializer.finish()
            }
            Self::Dimgrey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Dimgrey", 43usize, 0usize)?;
                serializer.finish()
            }
            Self::Dodgerblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Dodgerblue", 44usize, 0usize)?;
                serializer.finish()
            }
            Self::Firebrick => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Firebrick", 45usize, 0usize)?;
                serializer.finish()
            }
            Self::Floralwhite => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Floralwhite", 46usize, 0usize)?;
                serializer.finish()
            }
            Self::Forestgreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Forestgreen", 47usize, 0usize)?;
                serializer.finish()
            }
            Self::Fuchsia => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Fuchsia", 48usize, 0usize)?;
                serializer.finish()
            }
            Self::Gainsboro => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Gainsboro", 49usize, 0usize)?;
                serializer.finish()
            }
            Self::Ghostwhite => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Ghostwhite", 50usize, 0usize)?;
                serializer.finish()
            }
            Self::Gold => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Gold", 51usize, 0usize)?;
                serializer.finish()
            }
            Self::Goldenrod => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Goldenrod", 52usize, 0usize)?;
                serializer.finish()
            }
            Self::Gray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Gray", 53usize, 0usize)?;
                serializer.finish()
            }
            Self::Grey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Grey", 54usize, 0usize)?;
                serializer.finish()
            }
            Self::Green => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Green", 55usize, 0usize)?;
                serializer.finish()
            }
            Self::Greenyellow => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Greenyellow", 56usize, 0usize)?;
                serializer.finish()
            }
            Self::Honeydew => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Honeydew", 57usize, 0usize)?;
                serializer.finish()
            }
            Self::Hotpink => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Hotpink", 58usize, 0usize)?;
                serializer.finish()
            }
            Self::Indianred => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Indianred", 59usize, 0usize)?;
                serializer.finish()
            }
            Self::Indigo => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Indigo", 60usize, 0usize)?;
                serializer.finish()
            }
            Self::Ivory => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Ivory", 61usize, 0usize)?;
                serializer.finish()
            }
            Self::Khaki => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Khaki", 62usize, 0usize)?;
                serializer.finish()
            }
            Self::Lavender => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lavender", 63usize, 0usize)?;
                serializer.finish()
            }
            Self::Lavenderblush => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lavenderblush", 64usize, 0usize)?;
                serializer.finish()
            }
            Self::Lawngreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lawngreen", 65usize, 0usize)?;
                serializer.finish()
            }
            Self::Lemonchiffon => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lemonchiffon", 66usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightblue", 67usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightcoral => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightcoral", 68usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightcyan => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightcyan", 69usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightgoldenrodyellow => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightgoldenrodyellow",
                    70usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightgray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightgray", 71usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightgreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightgreen", 72usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightgrey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightgrey", 73usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightpink => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightpink", 74usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightsalmon => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightsalmon", 75usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightseagreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightseagreen", 76usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightskyblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightskyblue", 77usize, 0usize)?;
                serializer.finish()
            }
            Self::Lightslategray => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightslategray",
                    78usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightslategrey => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightslategrey",
                    79usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightsteelblue => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightsteelblue",
                    80usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightyellow => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightyellow", 81usize, 0usize)?;
                serializer.finish()
            }
            Self::Lime => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lime", 82usize, 0usize)?;
                serializer.finish()
            }
            Self::Limegreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Limegreen", 83usize, 0usize)?;
                serializer.finish()
            }
            Self::Linen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Linen", 84usize, 0usize)?;
                serializer.finish()
            }
            Self::Magenta => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Magenta", 85usize, 0usize)?;
                serializer.finish()
            }
            Self::Maroon => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Maroon", 86usize, 0usize)?;
                serializer.finish()
            }
            Self::Mediumaquamarine => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumaquamarine",
                    87usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Mediumblue", 88usize, 0usize)?;
                serializer.finish()
            }
            Self::Mediumorchid => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Mediumorchid", 89usize, 0usize)?;
                serializer.finish()
            }
            Self::Mediumpurple => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Mediumpurple", 90usize, 0usize)?;
                serializer.finish()
            }
            Self::Mediumseagreen => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumseagreen",
                    91usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumslateblue => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumslateblue",
                    92usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumspringgreen => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumspringgreen",
                    93usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumturquoise => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumturquoise",
                    94usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumvioletred => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumvioletred",
                    95usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Midnightblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Midnightblue", 96usize, 0usize)?;
                serializer.finish()
            }
            Self::Mintcream => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Mintcream", 97usize, 0usize)?;
                serializer.finish()
            }
            Self::Mistyrose => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Mistyrose", 98usize, 0usize)?;
                serializer.finish()
            }
            Self::Moccasin => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Moccasin", 99usize, 0usize)?;
                serializer.finish()
            }
            Self::Navajowhite => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Navajowhite", 100usize, 0usize)?;
                serializer.finish()
            }
            Self::Navy => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Navy", 101usize, 0usize)?;
                serializer.finish()
            }
            Self::Oldlace => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Oldlace", 102usize, 0usize)?;
                serializer.finish()
            }
            Self::Olive => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Olive", 103usize, 0usize)?;
                serializer.finish()
            }
            Self::Olivedrab => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Olivedrab", 104usize, 0usize)?;
                serializer.finish()
            }
            Self::Orange => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Orange", 105usize, 0usize)?;
                serializer.finish()
            }
            Self::Orangered => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Orangered", 106usize, 0usize)?;
                serializer.finish()
            }
            Self::Orchid => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Orchid", 107usize, 0usize)?;
                serializer.finish()
            }
            Self::Palegoldenrod => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Palegoldenrod",
                    108usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Palegreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Palegreen", 109usize, 0usize)?;
                serializer.finish()
            }
            Self::Paleturquoise => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Paleturquoise",
                    110usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Palevioletred => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Palevioletred",
                    111usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Papayawhip => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Papayawhip", 112usize, 0usize)?;
                serializer.finish()
            }
            Self::Peachpuff => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Peachpuff", 113usize, 0usize)?;
                serializer.finish()
            }
            Self::Peru => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Peru", 114usize, 0usize)?;
                serializer.finish()
            }
            Self::Pink => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Pink", 115usize, 0usize)?;
                serializer.finish()
            }
            Self::Plum => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Plum", 116usize, 0usize)?;
                serializer.finish()
            }
            Self::Powderblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Powderblue", 117usize, 0usize)?;
                serializer.finish()
            }
            Self::Purple => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Purple", 118usize, 0usize)?;
                serializer.finish()
            }
            Self::Red => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Red", 119usize, 0usize)?;
                serializer.finish()
            }
            Self::Rosybrown => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Rosybrown", 120usize, 0usize)?;
                serializer.finish()
            }
            Self::Royalblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Royalblue", 121usize, 0usize)?;
                serializer.finish()
            }
            Self::Saddlebrown => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Saddlebrown", 122usize, 0usize)?;
                serializer.finish()
            }
            Self::Salmon => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Salmon", 123usize, 0usize)?;
                serializer.finish()
            }
            Self::Sandybrown => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Sandybrown", 124usize, 0usize)?;
                serializer.finish()
            }
            Self::Seagreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Seagreen", 125usize, 0usize)?;
                serializer.finish()
            }
            Self::Seashell => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Seashell", 126usize, 0usize)?;
                serializer.finish()
            }
            Self::Sienna => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Sienna", 127usize, 0usize)?;
                serializer.finish()
            }
            Self::Silver => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Silver", 128usize, 0usize)?;
                serializer.finish()
            }
            Self::Skyblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Skyblue", 129usize, 0usize)?;
                serializer.finish()
            }
            Self::Slateblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Slateblue", 130usize, 0usize)?;
                serializer.finish()
            }
            Self::Slategray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Slategray", 131usize, 0usize)?;
                serializer.finish()
            }
            Self::Slategrey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Slategrey", 132usize, 0usize)?;
                serializer.finish()
            }
            Self::Snow => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Snow", 133usize, 0usize)?;
                serializer.finish()
            }
            Self::Springgreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Springgreen", 134usize, 0usize)?;
                serializer.finish()
            }
            Self::Steelblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Steelblue", 135usize, 0usize)?;
                serializer.finish()
            }
            Self::Tan => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Tan", 136usize, 0usize)?;
                serializer.finish()
            }
            Self::Teal => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Teal", 137usize, 0usize)?;
                serializer.finish()
            }
            Self::Thistle => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Thistle", 138usize, 0usize)?;
                serializer.finish()
            }
            Self::Tomato => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Tomato", 139usize, 0usize)?;
                serializer.finish()
            }
            Self::Turquoise => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Turquoise", 140usize, 0usize)?;
                serializer.finish()
            }
            Self::Violet => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Violet", 141usize, 0usize)?;
                serializer.finish()
            }
            Self::Wheat => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Wheat", 142usize, 0usize)?;
                serializer.finish()
            }
            Self::White => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "White", 143usize, 0usize)?;
                serializer.finish()
            }
            Self::Whitesmoke => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Whitesmoke", 144usize, 0usize)?;
                serializer.finish()
            }
            Self::Yellow => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Yellow", 145usize, 0usize)?;
                serializer.finish()
            }
            Self::Yellowgreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Yellowgreen", 146usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(3usize, "Rgb", 3usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        serializer.serialize_field(2usize, None, &self.2)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Rgb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Rgb;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Iri {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl ml::rt::serde::ser::Serialize for super::opcode::FuncIri {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(5usize, "FuncIri", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FuncIri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FuncIri;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Point {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(6usize, "Point", 2usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Point;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Percent {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(7usize, "Percent", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Percent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Percent;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Paint {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::None => {
                let mut serializer =
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
impl ml::rt::serde::ser::Serialize for super::opcode::NumberOptNumber {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(9usize, "NumberOptNumber", 2usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::NumberOptNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::NumberOptNumber;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Coords {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::UserSpaceOnUse => {
                let mut serializer = serializer.serialize_enum(
                    10usize,
                    "Coords",
                    "UserSpaceOnUse",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ObjectBoundingBox => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::Transform {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl ml::rt::serde::ser::Serialize for super::opcode::Channel {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::R => {
                let mut serializer =
                    serializer.serialize_enum(12usize, "Channel", "R", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::G => {
                let mut serializer =
                    serializer.serialize_enum(12usize, "Channel", "G", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::B => {
                let mut serializer =
                    serializer.serialize_enum(12usize, "Channel", "B", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::A => {
                let mut serializer =
                    serializer.serialize_enum(12usize, "Channel", "A", 3usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::ClipRule {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Nonzero => {
                let mut serializer =
                    serializer.serialize_enum(13usize, "ClipRule", "Nonzero", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::EvenOdd => {
                let mut serializer =
                    serializer.serialize_enum(13usize, "ClipRule", "EvenOdd", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::PathEvent {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Close => {
                let mut serializer =
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
impl ml::rt::serde::ser::Serialize for super::opcode::FillRule {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Nonzero => {
                let mut serializer =
                    serializer.serialize_enum(15usize, "FillRule", "Nonzero", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::EvenOdd => {
                let mut serializer =
                    serializer.serialize_enum(15usize, "FillRule", "EvenOdd", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::StrokeLineCap {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Butt => {
                let mut serializer =
                    serializer.serialize_enum(16usize, "stroke-linecap", "Butt", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Round => {
                let mut serializer = serializer.serialize_enum(
                    16usize,
                    "stroke-linecap",
                    "Round",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Square => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::StrokeLineJoin {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
                let mut serializer = serializer.serialize_enum(
                    17usize,
                    "stroke-linejoin",
                    "Round",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Bevel => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::SpreadMethod {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Pad => {
                let mut serializer =
                    serializer.serialize_enum(18usize, "SpreadMethod", "Pad", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Reflect => {
                let mut serializer = serializer.serialize_enum(
                    18usize,
                    "SpreadMethod",
                    "Reflect",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Repeat => {
                let mut serializer =
                    serializer.serialize_enum(18usize, "SpreadMethod", "Repeat", 2usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FontStyle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(19usize, "FontStyle", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Italic => {
                let mut serializer =
                    serializer.serialize_enum(19usize, "FontStyle", "Italic", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::Oblique => {
                let mut serializer =
                    serializer.serialize_enum(19usize, "FontStyle", "Oblique", 2usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FontVariant {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(20usize, "FontVariant", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::SmallCaps => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::FontWeight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Bold => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Bold", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::Bolder => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Bolder", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Lighter => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Lighter", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::W100 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W100", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::W200 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W200", 5usize, 0usize)?;
                serializer.finish()
            }
            Self::W300 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W300", 6usize, 0usize)?;
                serializer.finish()
            }
            Self::W400 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W400", 7usize, 0usize)?;
                serializer.finish()
            }
            Self::W500 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W500", 8usize, 0usize)?;
                serializer.finish()
            }
            Self::W600 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W600", 9usize, 0usize)?;
                serializer.finish()
            }
            Self::W700 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W700", 10usize, 0usize)?;
                serializer.finish()
            }
            Self::W800 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W800", 11usize, 0usize)?;
                serializer.finish()
            }
            Self::W900 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W900", 12usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FontFamily {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Serif => {
                let mut serializer =
                    serializer.serialize_enum(22usize, "FontFamily", "Serif", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::SansSerif => {
                let mut serializer = serializer.serialize_enum(
                    22usize,
                    "FontFamily",
                    "SansSerif",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Cursive => {
                let mut serializer =
                    serializer.serialize_enum(22usize, "FontFamily", "Cursive", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Fantasy => {
                let mut serializer =
                    serializer.serialize_enum(22usize, "FontFamily", "Fantasy", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::Monospace => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::FontStretch {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(23usize, "FontStretch", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Wider => {
                let mut serializer =
                    serializer.serialize_enum(23usize, "FontStretch", "Wider", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::Narrower => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "Narrower",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::UltraCondensed => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "UltraCondensed",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ExtraCondensed => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "ExtraCondensed",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Condensed => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "Condensed",
                    5usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SemiCondensed => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "SemiCondensed",
                    6usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SemiExpanded => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "SemiExpanded",
                    7usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Expanded => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "Expanded",
                    8usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ExtraExpanded => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "ExtraExpanded",
                    9usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::UltraExpanded => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::Background {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Accumulate => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::BackgroundNew {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(25usize, "BackgroundNew", 4usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::BackgroundNew {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::BackgroundNew;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeIn {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::SourceGraphic => {
                let mut serializer =
                    serializer.serialize_enum(26usize, "FeIn", "SourceGraphic", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::SourceAlpha => {
                let mut serializer =
                    serializer.serialize_enum(26usize, "FeIn", "SourceAlpha", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::BackgroundImage => {
                let mut serializer = serializer.serialize_enum(
                    26usize,
                    "FeIn",
                    "BackgroundImage",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::BackgroundAlpha => {
                let mut serializer = serializer.serialize_enum(
                    26usize,
                    "FeIn",
                    "BackgroundAlpha",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::FillPaint => {
                let mut serializer =
                    serializer.serialize_enum(26usize, "FeIn", "FillPaint", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::StrokePaint => {
                let mut serializer =
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
impl ml::rt::serde::ser::Serialize for super::opcode::FeOut {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Position => {
                let mut serializer =
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
impl ml::rt::serde::ser::Serialize for super::opcode::FeBlendMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(28usize, "FeBlendMode", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Multiply => {
                let mut serializer = serializer.serialize_enum(
                    28usize,
                    "FeBlendMode",
                    "Multiply",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Screen => {
                let mut serializer =
                    serializer.serialize_enum(28usize, "FeBlendMode", "Screen", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Darken => {
                let mut serializer =
                    serializer.serialize_enum(28usize, "FeBlendMode", "Darken", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::Lighten => {
                let mut serializer =
                    serializer.serialize_enum(28usize, "FeBlendMode", "Lighten", 4usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::TextLengthAdjust {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Spacing => {
                let mut serializer = serializer.serialize_enum(
                    29usize,
                    "TextLengthAdjust",
                    "Spacing",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SpacingAndGlyphs => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::WritingMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::LrTb => {
                let mut serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "LrTb", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::RlTb => {
                let mut serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "RlTb", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::TbRl => {
                let mut serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "TbRl", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Lr => {
                let mut serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "Lr", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::Rl => {
                let mut serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "Rl", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::Tb => {
                let mut serializer =
                    serializer.serialize_enum(30usize, "WritingMode", "Tb", 5usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::TextDirection {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Ltr => {
                let mut serializer =
                    serializer.serialize_enum(31usize, "TextDirection", "Ltr", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Rtl => {
                let mut serializer =
                    serializer.serialize_enum(31usize, "TextDirection", "Rtl", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::UnicodeBidi {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(32usize, "UnicodeBidi", "Normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Embed => {
                let mut serializer =
                    serializer.serialize_enum(32usize, "UnicodeBidi", "Embed", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::BidiOverride => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::TextAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Start => {
                let mut serializer =
                    serializer.serialize_enum(33usize, "TextAnchor", "Start", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Middle => {
                let mut serializer =
                    serializer.serialize_enum(33usize, "TextAnchor", "Middle", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::End => {
                let mut serializer =
                    serializer.serialize_enum(33usize, "TextAnchor", "End", 2usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::DominantBaseline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Auto => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Auto",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::UseScript => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "UseScript",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::NoChange => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "NoChange",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ResetSize => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "ResetSize",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Ideographic => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Ideographic",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Alphabetic => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Alphabetic",
                    5usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Hanging => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Hanging",
                    6usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mathematical => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Mathematical",
                    7usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Central => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Central",
                    8usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Middle => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "Middle",
                    9usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextAfterEdge => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "DominantBaseline",
                    "TextAfterEdge",
                    10usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextBeforeEdge => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::AlignmentBaseline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Auto => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Auto",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Baseline => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Baseline",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::BeforeEdge => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "BeforeEdge",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextBeforeEdge => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "TextBeforeEdge",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Middle => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Middle",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Central => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Central",
                    5usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::AfterEdge => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "AfterEdge",
                    6usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextAfterEdge => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "TextAfterEdge",
                    7usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Ideographic => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Ideographic",
                    8usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Alphabetic => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Alphabetic",
                    9usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Hanging => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "AlignmentBaseline",
                    "Hanging",
                    10usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mathematical => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::BaselineShift {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Baseline => {
                let mut serializer = serializer.serialize_enum(
                    36usize,
                    "BaselineShift",
                    "Baseline",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SubScripts => {
                let mut serializer = serializer.serialize_enum(
                    36usize,
                    "BaselineShift",
                    "SubScripts",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SuperScripts => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::TextDecoration {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Underline => {
                let mut serializer = serializer.serialize_enum(
                    37usize,
                    "TextDecoration",
                    "Underline",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Overline => {
                let mut serializer = serializer.serialize_enum(
                    37usize,
                    "TextDecoration",
                    "Overline",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::LineThrough => {
                let mut serializer = serializer.serialize_enum(
                    37usize,
                    "TextDecoration",
                    "LineThrough",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Blink => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::TextPathMethod {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Align => {
                let mut serializer = serializer.serialize_enum(
                    38usize,
                    "TextPathMethod",
                    "Align",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Stretch => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::TextPathSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Auto => {
                let mut serializer = serializer.serialize_enum(
                    39usize,
                    "TextPathSpacing",
                    "Auto",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Exact => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::LetterSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::WordSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
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
impl ml::rt::serde::ser::Serialize for super::opcode::MeetOrSlice {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Meet => {
                let mut serializer =
                    serializer.serialize_enum(42usize, "MeetOrSlice", "Meet", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Slice => {
                let mut serializer =
                    serializer.serialize_enum(42usize, "MeetOrSlice", "Slice", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::PreserveAspectRatio {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::None => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::TextLayout {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::TextLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::TextLayout;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::WithTransform {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(45usize, "WithTransform", 1usize)?;
        serializer.serialize_field(0usize, Some("transform"), &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::WithTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::WithTransform;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Id {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(46usize, "Id", 1usize)?;
        serializer.serialize_field(0usize, Some("id"), &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Id;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Fill {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(47usize, "Fill", 3usize)?;
        serializer.serialize_field(0usize, Some("fill"), &self.paint)?;
        serializer.serialize_field(1usize, Some("fill-rule"), &self.rule)?;
        serializer.serialize_field(2usize, Some("fill-opacity"), &self.opacity)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Fill {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Fill;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Stroke {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Stroke {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Stroke;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Font {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Font {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Font;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::EnableBackground {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(50usize, "EnableBackground", 1usize)?;
        serializer.serialize_field(0usize, Some("enable-background"), &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::EnableBackground {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::EnableBackground;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::WithFilter {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(51usize, "WithFilter", 1usize)?;
        serializer.serialize_field(0usize, Some("filter"), &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::WithFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::WithFilter;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::WithClipPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(52usize, "WithClipPath", 1usize)?;
        serializer.serialize_field(0usize, Some("clip-path"), &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::WithClipPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::WithClipPath;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::WithMask {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(53usize, "WithMask", 1usize)?;
        serializer.serialize_field(0usize, Some("mask"), &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::WithMask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::WithMask;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Opacity {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(54usize, "Opacity", 1usize)?;
        serializer.serialize_field(0usize, Some("opacity"), &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Opacity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Opacity;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::ViewBox {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(55usize, "ViewBox", 5usize)?;
        serializer.serialize_field(0usize, Some("minx"), &self.minx)?;
        serializer.serialize_field(1usize, Some("miny"), &self.miny)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("aspect"), &self.aspect)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::ViewBox {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::ViewBox;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Canvas {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(57usize, "svg", 2usize)?;
        serializer.serialize_field(0usize, Some("width"), &self.width)?;
        serializer.serialize_field(1usize, Some("height"), &self.height)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Canvas {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Canvas;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Mask {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Mask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Mask;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::ClipPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(59usize, "ClipPath", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::ClipPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::ClipPath;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Filter {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Filter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Filter;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeDistantLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(61usize, "FeDistantLight", 2usize)?;
        serializer.serialize_field(0usize, Some("azimuth"), &self.azimuth)?;
        serializer.serialize_field(1usize, Some("elevation"), &self.elevation)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeDistantLight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeDistantLight;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FePointLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(62usize, "FePointLight", 3usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("z"), &self.z)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FePointLight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FePointLight;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeSpotLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeSpotLight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeSpotLight;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeBlend {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeBlend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeBlend;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeColorMatrixValues {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::FeColorMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeColorMatrix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeColorMatrix;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeFunc {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Identity => {
                let mut serializer =
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
impl ml::rt::serde::ser::Serialize for super::opcode::FeCompositeOperator {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Over => {
                let mut serializer = serializer.serialize_enum(
                    68usize,
                    "FeCompositeOperator",
                    "Over",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::In => {
                let mut serializer = serializer.serialize_enum(
                    68usize,
                    "FeCompositeOperator",
                    "In",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Out => {
                let mut serializer = serializer.serialize_enum(
                    68usize,
                    "FeCompositeOperator",
                    "Out",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Atop => {
                let mut serializer = serializer.serialize_enum(
                    68usize,
                    "FeCompositeOperator",
                    "Atop",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Xor => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::FeConvolveMatrixEdgeMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Duplicate => {
                let mut serializer = serializer.serialize_enum(
                    69usize,
                    "FeConvolveMatrixEdgeMode",
                    "Duplicate",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Wrap => {
                let mut serializer = serializer.serialize_enum(
                    69usize,
                    "FeConvolveMatrixEdgeMode",
                    "Wrap",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::None => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::FeMorphologyOperator {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Erode => {
                let mut serializer = serializer.serialize_enum(
                    70usize,
                    "FeMorphologyOperator",
                    "Erode",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Dilate => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::FeStitchTiles {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::Stitch => {
                let mut serializer = serializer.serialize_enum(
                    71usize,
                    "FeStitchTiles",
                    "Stitch",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::NoStitch => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::FeTurbulenceType {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        match self {
            Self::FractalNoise => {
                let mut serializer = serializer.serialize_enum(
                    72usize,
                    "FeTurbulenceType",
                    "FractalNoise",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Turbulence => {
                let mut serializer = serializer.serialize_enum(
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
impl ml::rt::serde::ser::Serialize for super::opcode::FeComponentTransfer {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(73usize, "FeComponentTransfer", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeComponentTransfer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeComponentTransfer;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeFuncA {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(74usize, "FeFuncA", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeFuncA {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeFuncA;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeFuncR {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(75usize, "FeFuncR", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeFuncR {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeFuncR;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeFuncG {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(76usize, "FeFuncG", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeFuncG {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeFuncG;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeFuncB {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(77usize, "FeFuncB", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeFuncB {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeFuncB;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeComposite {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeComposite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeComposite;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeConvolveMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeConvolveMatrix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeConvolveMatrix;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeDiffuseLighting {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeDiffuseLighting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeDiffuseLighting;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeDisplacementMap {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeDisplacementMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeDisplacementMap;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeFlood {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeFlood {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeFlood;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeGaussianBlur {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeGaussianBlur {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeGaussianBlur;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeMerge {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(84usize, "FeMerge", 5usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeMerge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeMerge;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeMergeNode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(85usize, "FeMergeNode", 1usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeMergeNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeMergeNode;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeImage {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeImage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeImage;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeMorphology {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeMorphology {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeMorphology;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeOffset {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeOffset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeOffset;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeSpecularLighting {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeSpecularLighting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeSpecularLighting;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeTile {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeTile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeTile;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::FeTurbulence {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::FeTurbulence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::FeTurbulence;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::LinearGradient {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::LinearGradient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::LinearGradient;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::RadialGradient {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::RadialGradient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::RadialGradient;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::GradientStop {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(94usize, "stop", 3usize)?;
        serializer.serialize_field(0usize, Some("offset"), &self.offset)?;
        serializer.serialize_field(1usize, Some("stop-color"), &self.color)?;
        serializer.serialize_field(2usize, Some("stop-opacity"), &self.opacity)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::GradientStop {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::GradientStop;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Group {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(95usize, "g", 0usize)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Group {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Group;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Path {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(96usize, "Path", 2usize)?;
        serializer.serialize_field(0usize, Some("d"), &self.events)?;
        serializer.serialize_field(1usize, Some("length"), &self.length)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Path;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Pattern;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Use {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(98usize, "Use", 1usize)?;
        serializer.serialize_field(0usize, Some("xlink:href"), &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Use {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Use;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Rect {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Rect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Rect;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Circle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(100usize, "Circle", 3usize)?;
        serializer.serialize_field(0usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(1usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(2usize, Some("r"), &self.r)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Circle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Circle;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Ellipse {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(101usize, "Ellipse", 4usize)?;
        serializer.serialize_field(0usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(1usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(2usize, Some("rx"), &self.rx)?;
        serializer.serialize_field(3usize, Some("ry"), &self.ry)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Ellipse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Ellipse;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Line {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(102usize, "Line", 4usize)?;
        serializer.serialize_field(0usize, Some("x1"), &self.x1)?;
        serializer.serialize_field(1usize, Some("y1"), &self.y1)?;
        serializer.serialize_field(2usize, Some("x2"), &self.x2)?;
        serializer.serialize_field(3usize, Some("y2"), &self.y2)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Line {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Line;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Polyline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(103usize, "Polyline", 1usize)?;
        serializer.serialize_field(0usize, Some("points"), &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Polyline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Polyline;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Polygon {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(104usize, "Polygon", 1usize)?;
        serializer.serialize_field(0usize, Some("points"), &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Polygon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Polygon;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Text {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Text {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Text;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::TextSpan {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
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
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::TextSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::TextSpan;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Characters {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(108usize, "Characters", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::Characters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::Characters;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::TextPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
    {
        use ml::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(109usize, "TextPath", 4usize)?;
        serializer.serialize_field(0usize, Some("start_offset"), &self.start_offset)?;
        serializer.serialize_field(1usize, Some("method"), &self.method)?;
        serializer.serialize_field(2usize, Some("spacing"), &self.spacing)?;
        serializer.serialize_field(3usize, Some("href"), &self.href)?;
        serializer.finish()
    }
}
impl<'de> ml::rt::serde::de::Deserialize<'de> for super::opcode::TextPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ml::rt::serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ml::rt::serde::de::Visitor<'de> for Visitor {
            type Value = super::opcode::TextPath;
        }
        todo!()
    }
}
impl ml::rt::serde::ser::Serialize for super::opcode::Opcode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::ser::Serializer,
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
