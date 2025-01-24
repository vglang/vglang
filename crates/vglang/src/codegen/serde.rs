impl ml::rt::serde::Serialize for super::opcode::Angle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Deg(p0) => {
                let mut serializer =
                    serializer.serialize_enum(0usize, "Angle", "Deg", 0usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Grad(p0) => {
                let mut serializer =
                    serializer.serialize_enum(0usize, "Angle", "Grad", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Rad(p0) => {
                let mut serializer =
                    serializer.serialize_enum(0usize, "Angle", "Rad", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Length {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Em(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Em", 0usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Ex(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Ex", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Px(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Px", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Inch(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Inch", 3usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Cm(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Cm", 4usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Mm(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Mm", 5usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Pt(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Pt", 6usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Pc(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Pc", 7usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Percent(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "Length", "Percent", 8usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Color {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Aliceblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Aliceblue", 0usize, 0usize)?;
                Ok(())
            }
            Self::Antiquewhite => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Antiquewhite", 1usize, 0usize)?;
                Ok(())
            }
            Self::Aqua => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Aqua", 2usize, 0usize)?;
                Ok(())
            }
            Self::Aquamarine => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Aquamarine", 3usize, 0usize)?;
                Ok(())
            }
            Self::Azure => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Azure", 4usize, 0usize)?;
                Ok(())
            }
            Self::Beige => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Beige", 5usize, 0usize)?;
                Ok(())
            }
            Self::Bisque => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Bisque", 6usize, 0usize)?;
                Ok(())
            }
            Self::Black => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Black", 7usize, 0usize)?;
                Ok(())
            }
            Self::Blanchedalmond => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Blanchedalmond", 8usize, 0usize)?;
                Ok(())
            }
            Self::Blue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Blue", 9usize, 0usize)?;
                Ok(())
            }
            Self::Blueviolet => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Blueviolet", 10usize, 0usize)?;
                Ok(())
            }
            Self::Brown => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Brown", 11usize, 0usize)?;
                Ok(())
            }
            Self::Burlywood => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Burlywood", 12usize, 0usize)?;
                Ok(())
            }
            Self::Cadetblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Cadetblue", 13usize, 0usize)?;
                Ok(())
            }
            Self::Chartreuse => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Chartreuse", 14usize, 0usize)?;
                Ok(())
            }
            Self::Chocolate => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Chocolate", 15usize, 0usize)?;
                Ok(())
            }
            Self::Coral => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Coral", 16usize, 0usize)?;
                Ok(())
            }
            Self::Cornflowerblue => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Cornflowerblue",
                    17usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Cornsilk => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Cornsilk", 18usize, 0usize)?;
                Ok(())
            }
            Self::Crimson => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Crimson", 19usize, 0usize)?;
                Ok(())
            }
            Self::Cyan => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Cyan", 20usize, 0usize)?;
                Ok(())
            }
            Self::Darkblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkblue", 21usize, 0usize)?;
                Ok(())
            }
            Self::Darkcyan => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkcyan", 22usize, 0usize)?;
                Ok(())
            }
            Self::Darkgoldenrod => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgoldenrod", 23usize, 0usize)?;
                Ok(())
            }
            Self::Darkgray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgray", 24usize, 0usize)?;
                Ok(())
            }
            Self::Darkgreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgreen", 25usize, 0usize)?;
                Ok(())
            }
            Self::Darkgrey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkgrey", 26usize, 0usize)?;
                Ok(())
            }
            Self::Darkkhaki => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkkhaki", 27usize, 0usize)?;
                Ok(())
            }
            Self::Darkmagenta => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkmagenta", 28usize, 0usize)?;
                Ok(())
            }
            Self::Darkolivegreen => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Darkolivegreen",
                    29usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Darkorange => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkorange", 30usize, 0usize)?;
                Ok(())
            }
            Self::Darkorchid => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkorchid", 31usize, 0usize)?;
                Ok(())
            }
            Self::Darkred => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkred", 32usize, 0usize)?;
                Ok(())
            }
            Self::Darksalmon => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darksalmon", 33usize, 0usize)?;
                Ok(())
            }
            Self::Darkseagreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkseagreen", 34usize, 0usize)?;
                Ok(())
            }
            Self::Darkslateblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkslateblue", 35usize, 0usize)?;
                Ok(())
            }
            Self::Darkslategray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkslategray", 36usize, 0usize)?;
                Ok(())
            }
            Self::Darkslategrey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkslategrey", 37usize, 0usize)?;
                Ok(())
            }
            Self::Darkturquoise => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkturquoise", 38usize, 0usize)?;
                Ok(())
            }
            Self::Darkviolet => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Darkviolet", 39usize, 0usize)?;
                Ok(())
            }
            Self::Deeppink => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Deeppink", 40usize, 0usize)?;
                Ok(())
            }
            Self::Deepskyblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Deepskyblue", 41usize, 0usize)?;
                Ok(())
            }
            Self::Dimgray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Dimgray", 42usize, 0usize)?;
                Ok(())
            }
            Self::Dimgrey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Dimgrey", 43usize, 0usize)?;
                Ok(())
            }
            Self::Dodgerblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Dodgerblue", 44usize, 0usize)?;
                Ok(())
            }
            Self::Firebrick => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Firebrick", 45usize, 0usize)?;
                Ok(())
            }
            Self::Floralwhite => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Floralwhite", 46usize, 0usize)?;
                Ok(())
            }
            Self::Forestgreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Forestgreen", 47usize, 0usize)?;
                Ok(())
            }
            Self::Fuchsia => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Fuchsia", 48usize, 0usize)?;
                Ok(())
            }
            Self::Gainsboro => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Gainsboro", 49usize, 0usize)?;
                Ok(())
            }
            Self::Ghostwhite => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Ghostwhite", 50usize, 0usize)?;
                Ok(())
            }
            Self::Gold => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Gold", 51usize, 0usize)?;
                Ok(())
            }
            Self::Goldenrod => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Goldenrod", 52usize, 0usize)?;
                Ok(())
            }
            Self::Gray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Gray", 53usize, 0usize)?;
                Ok(())
            }
            Self::Grey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Grey", 54usize, 0usize)?;
                Ok(())
            }
            Self::Green => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Green", 55usize, 0usize)?;
                Ok(())
            }
            Self::Greenyellow => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Greenyellow", 56usize, 0usize)?;
                Ok(())
            }
            Self::Honeydew => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Honeydew", 57usize, 0usize)?;
                Ok(())
            }
            Self::Hotpink => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Hotpink", 58usize, 0usize)?;
                Ok(())
            }
            Self::Indianred => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Indianred", 59usize, 0usize)?;
                Ok(())
            }
            Self::Indigo => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Indigo", 60usize, 0usize)?;
                Ok(())
            }
            Self::Ivory => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Ivory", 61usize, 0usize)?;
                Ok(())
            }
            Self::Khaki => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Khaki", 62usize, 0usize)?;
                Ok(())
            }
            Self::Lavender => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lavender", 63usize, 0usize)?;
                Ok(())
            }
            Self::Lavenderblush => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lavenderblush", 64usize, 0usize)?;
                Ok(())
            }
            Self::Lawngreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lawngreen", 65usize, 0usize)?;
                Ok(())
            }
            Self::Lemonchiffon => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lemonchiffon", 66usize, 0usize)?;
                Ok(())
            }
            Self::Lightblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightblue", 67usize, 0usize)?;
                Ok(())
            }
            Self::Lightcoral => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightcoral", 68usize, 0usize)?;
                Ok(())
            }
            Self::Lightcyan => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightcyan", 69usize, 0usize)?;
                Ok(())
            }
            Self::Lightgoldenrodyellow => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightgoldenrodyellow",
                    70usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Lightgray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightgray", 71usize, 0usize)?;
                Ok(())
            }
            Self::Lightgreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightgreen", 72usize, 0usize)?;
                Ok(())
            }
            Self::Lightgrey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightgrey", 73usize, 0usize)?;
                Ok(())
            }
            Self::Lightpink => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightpink", 74usize, 0usize)?;
                Ok(())
            }
            Self::Lightsalmon => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightsalmon", 75usize, 0usize)?;
                Ok(())
            }
            Self::Lightseagreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightseagreen", 76usize, 0usize)?;
                Ok(())
            }
            Self::Lightskyblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightskyblue", 77usize, 0usize)?;
                Ok(())
            }
            Self::Lightslategray => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightslategray",
                    78usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Lightslategrey => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightslategrey",
                    79usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Lightsteelblue => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Lightsteelblue",
                    80usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Lightyellow => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lightyellow", 81usize, 0usize)?;
                Ok(())
            }
            Self::Lime => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Lime", 82usize, 0usize)?;
                Ok(())
            }
            Self::Limegreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Limegreen", 83usize, 0usize)?;
                Ok(())
            }
            Self::Linen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Linen", 84usize, 0usize)?;
                Ok(())
            }
            Self::Magenta => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Magenta", 85usize, 0usize)?;
                Ok(())
            }
            Self::Maroon => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Maroon", 86usize, 0usize)?;
                Ok(())
            }
            Self::Mediumaquamarine => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumaquamarine",
                    87usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Mediumblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Mediumblue", 88usize, 0usize)?;
                Ok(())
            }
            Self::Mediumorchid => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Mediumorchid", 89usize, 0usize)?;
                Ok(())
            }
            Self::Mediumpurple => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Mediumpurple", 90usize, 0usize)?;
                Ok(())
            }
            Self::Mediumseagreen => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumseagreen",
                    91usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Mediumslateblue => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumslateblue",
                    92usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Mediumspringgreen => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumspringgreen",
                    93usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Mediumturquoise => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumturquoise",
                    94usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Mediumvioletred => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Mediumvioletred",
                    95usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Midnightblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Midnightblue", 96usize, 0usize)?;
                Ok(())
            }
            Self::Mintcream => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Mintcream", 97usize, 0usize)?;
                Ok(())
            }
            Self::Mistyrose => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Mistyrose", 98usize, 0usize)?;
                Ok(())
            }
            Self::Moccasin => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Moccasin", 99usize, 0usize)?;
                Ok(())
            }
            Self::Navajowhite => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Navajowhite", 100usize, 0usize)?;
                Ok(())
            }
            Self::Navy => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Navy", 101usize, 0usize)?;
                Ok(())
            }
            Self::Oldlace => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Oldlace", 102usize, 0usize)?;
                Ok(())
            }
            Self::Olive => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Olive", 103usize, 0usize)?;
                Ok(())
            }
            Self::Olivedrab => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Olivedrab", 104usize, 0usize)?;
                Ok(())
            }
            Self::Orange => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Orange", 105usize, 0usize)?;
                Ok(())
            }
            Self::Orangered => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Orangered", 106usize, 0usize)?;
                Ok(())
            }
            Self::Orchid => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Orchid", 107usize, 0usize)?;
                Ok(())
            }
            Self::Palegoldenrod => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Palegoldenrod",
                    108usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Palegreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Palegreen", 109usize, 0usize)?;
                Ok(())
            }
            Self::Paleturquoise => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Paleturquoise",
                    110usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Palevioletred => {
                let mut serializer = serializer.serialize_enum(
                    2usize,
                    "Color",
                    "Palevioletred",
                    111usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Papayawhip => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Papayawhip", 112usize, 0usize)?;
                Ok(())
            }
            Self::Peachpuff => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Peachpuff", 113usize, 0usize)?;
                Ok(())
            }
            Self::Peru => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Peru", 114usize, 0usize)?;
                Ok(())
            }
            Self::Pink => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Pink", 115usize, 0usize)?;
                Ok(())
            }
            Self::Plum => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Plum", 116usize, 0usize)?;
                Ok(())
            }
            Self::Powderblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Powderblue", 117usize, 0usize)?;
                Ok(())
            }
            Self::Purple => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Purple", 118usize, 0usize)?;
                Ok(())
            }
            Self::Red => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Red", 119usize, 0usize)?;
                Ok(())
            }
            Self::Rosybrown => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Rosybrown", 120usize, 0usize)?;
                Ok(())
            }
            Self::Royalblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Royalblue", 121usize, 0usize)?;
                Ok(())
            }
            Self::Saddlebrown => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Saddlebrown", 122usize, 0usize)?;
                Ok(())
            }
            Self::Salmon => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Salmon", 123usize, 0usize)?;
                Ok(())
            }
            Self::Sandybrown => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Sandybrown", 124usize, 0usize)?;
                Ok(())
            }
            Self::Seagreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Seagreen", 125usize, 0usize)?;
                Ok(())
            }
            Self::Seashell => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Seashell", 126usize, 0usize)?;
                Ok(())
            }
            Self::Sienna => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Sienna", 127usize, 0usize)?;
                Ok(())
            }
            Self::Silver => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Silver", 128usize, 0usize)?;
                Ok(())
            }
            Self::Skyblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Skyblue", 129usize, 0usize)?;
                Ok(())
            }
            Self::Slateblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Slateblue", 130usize, 0usize)?;
                Ok(())
            }
            Self::Slategray => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Slategray", 131usize, 0usize)?;
                Ok(())
            }
            Self::Slategrey => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Slategrey", 132usize, 0usize)?;
                Ok(())
            }
            Self::Snow => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Snow", 133usize, 0usize)?;
                Ok(())
            }
            Self::Springgreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Springgreen", 134usize, 0usize)?;
                Ok(())
            }
            Self::Steelblue => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Steelblue", 135usize, 0usize)?;
                Ok(())
            }
            Self::Tan => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Tan", 136usize, 0usize)?;
                Ok(())
            }
            Self::Teal => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Teal", 137usize, 0usize)?;
                Ok(())
            }
            Self::Thistle => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Thistle", 138usize, 0usize)?;
                Ok(())
            }
            Self::Tomato => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Tomato", 139usize, 0usize)?;
                Ok(())
            }
            Self::Turquoise => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Turquoise", 140usize, 0usize)?;
                Ok(())
            }
            Self::Violet => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Violet", 141usize, 0usize)?;
                Ok(())
            }
            Self::Wheat => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Wheat", 142usize, 0usize)?;
                Ok(())
            }
            Self::White => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "White", 143usize, 0usize)?;
                Ok(())
            }
            Self::Whitesmoke => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Whitesmoke", 144usize, 0usize)?;
                Ok(())
            }
            Self::Yellow => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Yellow", 145usize, 0usize)?;
                Ok(())
            }
            Self::Yellowgreen => {
                let mut serializer =
                    serializer.serialize_enum(2usize, "Color", "Yellowgreen", 146usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_data(3usize, "Rgb", 3usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        serializer.serialize_field(2usize, None, &self.2)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Iri {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Local(p0) => {
                let mut serializer =
                    serializer.serialize_enum(4usize, "Iri", "Local", 0usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Path(p0) => {
                let mut serializer =
                    serializer.serialize_enum(4usize, "Iri", "Path", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FuncIri {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_data(5usize, "FuncIri", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Point {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_data(6usize, "Point", 2usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Percent {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_data(7usize, "Percent", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Paint {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::None => {
                let mut serializer =
                    serializer.serialize_enum(8usize, "Paint", "None", 0usize, 0usize)?;
                Ok(())
            }
            Self::Color(p0) => {
                let mut serializer =
                    serializer.serialize_enum(8usize, "Paint", "Color", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Server(p0) => {
                let mut serializer =
                    serializer.serialize_enum(8usize, "Paint", "Server", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::NumberOptNumber {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_data(9usize, "NumberOptNumber", 2usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Coords {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::UserSpaceOnUse => {
                let mut serializer = serializer.serialize_enum(
                    10usize,
                    "Coords",
                    "UserSpaceOnUse",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::ObjectBoundingBox => {
                let mut serializer = serializer.serialize_enum(
                    10usize,
                    "Coords",
                    "ObjectBoundingBox",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Transform {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Translate(p0, p1) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "Translate", 0usize, 2usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                Ok(())
            }
            Self::Matrix(p0) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "Matrix", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Scale(p0, p1) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "Scale", 2usize, 2usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                Ok(())
            }
            Self::Rotate { angle, cx, cy } => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "Rotate", 3usize, 3usize)?;
                serializer.serialize_field(0usize, Some("angle"), angle)?;
                serializer.serialize_field(1usize, Some("cx"), cx)?;
                serializer.serialize_field(2usize, Some("cy"), cy)?;
                Ok(())
            }
            Self::SkewX(p0) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "SkewX", 4usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::SkewY(p0) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "Transform", "SkewY", 5usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Channel {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::R => {
                let mut serializer =
                    serializer.serialize_enum(12usize, "Channel", "R", 0usize, 0usize)?;
                Ok(())
            }
            Self::G => {
                let mut serializer =
                    serializer.serialize_enum(12usize, "Channel", "G", 1usize, 0usize)?;
                Ok(())
            }
            Self::B => {
                let mut serializer =
                    serializer.serialize_enum(12usize, "Channel", "B", 2usize, 0usize)?;
                Ok(())
            }
            Self::A => {
                let mut serializer =
                    serializer.serialize_enum(12usize, "Channel", "A", 3usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::ClipRule {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Nonzero => {
                let mut serializer =
                    serializer.serialize_enum(13usize, "ClipRule", "Nonzero", 0usize, 0usize)?;
                Ok(())
            }
            Self::EvenOdd => {
                let mut serializer =
                    serializer.serialize_enum(13usize, "ClipRule", "EvenOdd", 1usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::PathEvent {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Close => {
                let mut serializer =
                    serializer.serialize_enum(14usize, "PathEvent", "Close", 0usize, 0usize)?;
                Ok(())
            }
            Self::MoveTo(p0) => {
                let mut serializer =
                    serializer.serialize_enum(14usize, "PathEvent", "MoveTo", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
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
                Ok(())
            }
            Self::LineTo(p0) => {
                let mut serializer =
                    serializer.serialize_enum(14usize, "PathEvent", "LineTo", 3usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
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
                Ok(())
            }
            Self::Polyline(p0) => {
                let mut serializer =
                    serializer.serialize_enum(14usize, "PathEvent", "Polyline", 5usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
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
                Ok(())
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
                Ok(())
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
                Ok(())
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
                Ok(())
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
                Ok(())
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
                Ok(())
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
                Ok(())
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
                Ok(())
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
                Ok(())
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
                Ok(())
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
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FillRule {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Nonzero => {
                let mut serializer =
                    serializer.serialize_enum(15usize, "FillRule", "Nonzero", 0usize, 0usize)?;
                Ok(())
            }
            Self::EvenOdd => {
                let mut serializer =
                    serializer.serialize_enum(15usize, "FillRule", "EvenOdd", 1usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::StrokeLineCap {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Butt => {
                let mut serializer =
                    serializer.serialize_enum(16usize, "StrokeLineCap", "Butt", 0usize, 0usize)?;
                Ok(())
            }
            Self::Round => {
                let mut serializer =
                    serializer.serialize_enum(16usize, "StrokeLineCap", "Round", 1usize, 0usize)?;
                Ok(())
            }
            Self::Square => {
                let mut serializer = serializer.serialize_enum(
                    16usize,
                    "StrokeLineCap",
                    "Square",
                    2usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::StrokeLineJoin {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Miter(p0) => {
                let mut serializer = serializer.serialize_enum(
                    17usize,
                    "StrokeLineJoin",
                    "Miter",
                    0usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Round => {
                let mut serializer = serializer.serialize_enum(
                    17usize,
                    "StrokeLineJoin",
                    "Round",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Bevel => {
                let mut serializer = serializer.serialize_enum(
                    17usize,
                    "StrokeLineJoin",
                    "Bevel",
                    2usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::SpreadMethod {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Pad => {
                let mut serializer =
                    serializer.serialize_enum(18usize, "SpreadMethod", "Pad", 0usize, 0usize)?;
                Ok(())
            }
            Self::Reflect => {
                let mut serializer = serializer.serialize_enum(
                    18usize,
                    "SpreadMethod",
                    "Reflect",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Repeat => {
                let mut serializer =
                    serializer.serialize_enum(18usize, "SpreadMethod", "Repeat", 2usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FontStyle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(19usize, "FontStyle", "Normal", 0usize, 0usize)?;
                Ok(())
            }
            Self::Italic => {
                let mut serializer =
                    serializer.serialize_enum(19usize, "FontStyle", "Italic", 1usize, 0usize)?;
                Ok(())
            }
            Self::Oblique => {
                let mut serializer =
                    serializer.serialize_enum(19usize, "FontStyle", "Oblique", 2usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FontVariant {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(20usize, "FontVariant", "Normal", 0usize, 0usize)?;
                Ok(())
            }
            Self::SmallCaps => {
                let mut serializer = serializer.serialize_enum(
                    20usize,
                    "FontVariant",
                    "SmallCaps",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FontWeight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Normal", 0usize, 0usize)?;
                Ok(())
            }
            Self::Bold => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Bold", 1usize, 0usize)?;
                Ok(())
            }
            Self::Bolder => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Bolder", 2usize, 0usize)?;
                Ok(())
            }
            Self::Lighter => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "Lighter", 3usize, 0usize)?;
                Ok(())
            }
            Self::W100 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W100", 4usize, 0usize)?;
                Ok(())
            }
            Self::W200 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W200", 5usize, 0usize)?;
                Ok(())
            }
            Self::W300 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W300", 6usize, 0usize)?;
                Ok(())
            }
            Self::W400 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W400", 7usize, 0usize)?;
                Ok(())
            }
            Self::W500 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W500", 8usize, 0usize)?;
                Ok(())
            }
            Self::W600 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W600", 9usize, 0usize)?;
                Ok(())
            }
            Self::W700 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W700", 10usize, 0usize)?;
                Ok(())
            }
            Self::W800 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W800", 11usize, 0usize)?;
                Ok(())
            }
            Self::W900 => {
                let mut serializer =
                    serializer.serialize_enum(21usize, "FontWeight", "W900", 12usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FontFamily {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Serif => {
                let mut serializer =
                    serializer.serialize_enum(22usize, "FontFamily", "Serif", 0usize, 0usize)?;
                Ok(())
            }
            Self::SansSerif => {
                let mut serializer = serializer.serialize_enum(
                    22usize,
                    "FontFamily",
                    "SansSerif",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Cursive => {
                let mut serializer =
                    serializer.serialize_enum(22usize, "FontFamily", "Cursive", 2usize, 0usize)?;
                Ok(())
            }
            Self::Fantasy => {
                let mut serializer =
                    serializer.serialize_enum(22usize, "FontFamily", "Fantasy", 3usize, 0usize)?;
                Ok(())
            }
            Self::Monospace => {
                let mut serializer = serializer.serialize_enum(
                    22usize,
                    "FontFamily",
                    "Monospace",
                    4usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Generic(p0) => {
                let mut serializer =
                    serializer.serialize_enum(22usize, "FontFamily", "Generic", 5usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FontStretch {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(23usize, "FontStretch", "Normal", 0usize, 0usize)?;
                Ok(())
            }
            Self::Wider => {
                let mut serializer =
                    serializer.serialize_enum(23usize, "FontStretch", "Wider", 1usize, 0usize)?;
                Ok(())
            }
            Self::Narrower => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "Narrower",
                    2usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::UltraCondensed => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "UltraCondensed",
                    3usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::ExtraCondensed => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "ExtraCondensed",
                    4usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Condensed => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "Condensed",
                    5usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::SemiCondensed => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "SemiCondensed",
                    6usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::SemiExpanded => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "SemiExpanded",
                    7usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Expanded => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "Expanded",
                    8usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::ExtraExpanded => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "ExtraExpanded",
                    9usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::UltraExpanded => {
                let mut serializer = serializer.serialize_enum(
                    23usize,
                    "FontStretch",
                    "UltraExpanded",
                    10usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Background {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Accumulate => {
                let mut serializer = serializer.serialize_enum(
                    24usize,
                    "Background",
                    "Accumulate",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::New {
                x,
                y,
                width,
                height,
            } => {
                let mut serializer =
                    serializer.serialize_enum(24usize, "Background", "New", 1usize, 4usize)?;
                serializer.serialize_field(0usize, Some("x"), x)?;
                serializer.serialize_field(1usize, Some("y"), y)?;
                serializer.serialize_field(2usize, Some("width"), width)?;
                serializer.serialize_field(3usize, Some("height"), height)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeIn {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::SourceGraphic => {
                let mut serializer =
                    serializer.serialize_enum(25usize, "FeIn", "SourceGraphic", 0usize, 0usize)?;
                Ok(())
            }
            Self::SourceAlpha => {
                let mut serializer =
                    serializer.serialize_enum(25usize, "FeIn", "SourceAlpha", 1usize, 0usize)?;
                Ok(())
            }
            Self::BackgroundImage => {
                let mut serializer = serializer.serialize_enum(
                    25usize,
                    "FeIn",
                    "BackgroundImage",
                    2usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::BackgroundAlpha => {
                let mut serializer = serializer.serialize_enum(
                    25usize,
                    "FeIn",
                    "BackgroundAlpha",
                    3usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::FillPaint => {
                let mut serializer =
                    serializer.serialize_enum(25usize, "FeIn", "FillPaint", 4usize, 0usize)?;
                Ok(())
            }
            Self::StrokePaint => {
                let mut serializer =
                    serializer.serialize_enum(25usize, "FeIn", "StrokePaint", 5usize, 0usize)?;
                Ok(())
            }
            Self::Result(p0) => {
                let mut serializer =
                    serializer.serialize_enum(25usize, "FeIn", "Result", 6usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeOut {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Position => {
                let mut serializer =
                    serializer.serialize_enum(26usize, "FeOut", "Position", 0usize, 0usize)?;
                Ok(())
            }
            Self::Named(p0) => {
                let mut serializer =
                    serializer.serialize_enum(26usize, "FeOut", "Named", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeBlendMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(27usize, "FeBlendMode", "Normal", 0usize, 0usize)?;
                Ok(())
            }
            Self::Multiply => {
                let mut serializer = serializer.serialize_enum(
                    27usize,
                    "FeBlendMode",
                    "Multiply",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Screen => {
                let mut serializer =
                    serializer.serialize_enum(27usize, "FeBlendMode", "Screen", 2usize, 0usize)?;
                Ok(())
            }
            Self::Darken => {
                let mut serializer =
                    serializer.serialize_enum(27usize, "FeBlendMode", "Darken", 3usize, 0usize)?;
                Ok(())
            }
            Self::Lighten => {
                let mut serializer =
                    serializer.serialize_enum(27usize, "FeBlendMode", "Lighten", 4usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextLengthAdjust {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Spacing => {
                let mut serializer = serializer.serialize_enum(
                    28usize,
                    "TextLengthAdjust",
                    "Spacing",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::SpacingAndGlyphs => {
                let mut serializer = serializer.serialize_enum(
                    28usize,
                    "TextLengthAdjust",
                    "SpacingAndGlyphs",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::WritingMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::LrTb => {
                let mut serializer =
                    serializer.serialize_enum(29usize, "WritingMode", "LrTb", 0usize, 0usize)?;
                Ok(())
            }
            Self::RlTb => {
                let mut serializer =
                    serializer.serialize_enum(29usize, "WritingMode", "RlTb", 1usize, 0usize)?;
                Ok(())
            }
            Self::TbRl => {
                let mut serializer =
                    serializer.serialize_enum(29usize, "WritingMode", "TbRl", 2usize, 0usize)?;
                Ok(())
            }
            Self::Lr => {
                let mut serializer =
                    serializer.serialize_enum(29usize, "WritingMode", "Lr", 3usize, 0usize)?;
                Ok(())
            }
            Self::Rl => {
                let mut serializer =
                    serializer.serialize_enum(29usize, "WritingMode", "Rl", 4usize, 0usize)?;
                Ok(())
            }
            Self::Tb => {
                let mut serializer =
                    serializer.serialize_enum(29usize, "WritingMode", "Tb", 5usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextDirection {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Ltr => {
                let mut serializer =
                    serializer.serialize_enum(30usize, "TextDirection", "Ltr", 0usize, 0usize)?;
                Ok(())
            }
            Self::Rtl => {
                let mut serializer =
                    serializer.serialize_enum(30usize, "TextDirection", "Rtl", 1usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::UnicodeBidi {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(31usize, "UnicodeBidi", "Normal", 0usize, 0usize)?;
                Ok(())
            }
            Self::Embed => {
                let mut serializer =
                    serializer.serialize_enum(31usize, "UnicodeBidi", "Embed", 1usize, 0usize)?;
                Ok(())
            }
            Self::BidiOverride => {
                let mut serializer = serializer.serialize_enum(
                    31usize,
                    "UnicodeBidi",
                    "BidiOverride",
                    2usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Start => {
                let mut serializer =
                    serializer.serialize_enum(32usize, "TextAnchor", "Start", 0usize, 0usize)?;
                Ok(())
            }
            Self::Middle => {
                let mut serializer =
                    serializer.serialize_enum(32usize, "TextAnchor", "Middle", 1usize, 0usize)?;
                Ok(())
            }
            Self::End => {
                let mut serializer =
                    serializer.serialize_enum(32usize, "TextAnchor", "End", 2usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::DominantBaseline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Auto => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "Auto",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::UseScript => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "UseScript",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::NoChange => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "NoChange",
                    2usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::ResetSize => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "ResetSize",
                    3usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Ideographic => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "Ideographic",
                    4usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Alphabetic => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "Alphabetic",
                    5usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Hanging => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "Hanging",
                    6usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Mathematical => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "Mathematical",
                    7usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Central => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "Central",
                    8usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Middle => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "Middle",
                    9usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::TextAfterEdge => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "TextAfterEdge",
                    10usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::TextBeforeEdge => {
                let mut serializer = serializer.serialize_enum(
                    33usize,
                    "DominantBaseline",
                    "TextBeforeEdge",
                    11usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::AlignmentBaseline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Auto => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "Auto",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Baseline => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "Baseline",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::BeforeEdge => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "BeforeEdge",
                    2usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::TextBeforeEdge => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "TextBeforeEdge",
                    3usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Middle => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "Middle",
                    4usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Central => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "Central",
                    5usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::AfterEdge => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "AfterEdge",
                    6usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::TextAfterEdge => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "TextAfterEdge",
                    7usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Ideographic => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "Ideographic",
                    8usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Alphabetic => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "Alphabetic",
                    9usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Hanging => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "Hanging",
                    10usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Mathematical => {
                let mut serializer = serializer.serialize_enum(
                    34usize,
                    "AlignmentBaseline",
                    "Mathematical",
                    11usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::BaselineShift {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Baseline => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "BaselineShift",
                    "Baseline",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::SubScripts => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "BaselineShift",
                    "SubScripts",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::SuperScripts => {
                let mut serializer = serializer.serialize_enum(
                    35usize,
                    "BaselineShift",
                    "SuperScripts",
                    2usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Value(p0) => {
                let mut serializer =
                    serializer.serialize_enum(35usize, "BaselineShift", "Value", 3usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextDecoration {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Underline => {
                let mut serializer = serializer.serialize_enum(
                    36usize,
                    "TextDecoration",
                    "Underline",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Overline => {
                let mut serializer = serializer.serialize_enum(
                    36usize,
                    "TextDecoration",
                    "Overline",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::LineThrough => {
                let mut serializer = serializer.serialize_enum(
                    36usize,
                    "TextDecoration",
                    "LineThrough",
                    2usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Blink => {
                let mut serializer = serializer.serialize_enum(
                    36usize,
                    "TextDecoration",
                    "Blink",
                    3usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextPathMethod {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Align => {
                let mut serializer = serializer.serialize_enum(
                    37usize,
                    "TextPathMethod",
                    "Align",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Stretch => {
                let mut serializer = serializer.serialize_enum(
                    37usize,
                    "TextPathMethod",
                    "Stretch",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextPathSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Auto => {
                let mut serializer = serializer.serialize_enum(
                    38usize,
                    "TextPathSpacing",
                    "Auto",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Exact => {
                let mut serializer = serializer.serialize_enum(
                    38usize,
                    "TextPathSpacing",
                    "Exact",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::LetterSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer = serializer.serialize_enum(
                    39usize,
                    "LetterSpacing",
                    "Normal",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Length(p0) => {
                let mut serializer = serializer.serialize_enum(
                    39usize,
                    "LetterSpacing",
                    "Length",
                    1usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::WordSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Normal => {
                let mut serializer =
                    serializer.serialize_enum(40usize, "WordSpacing", "Normal", 0usize, 0usize)?;
                Ok(())
            }
            Self::Length(p0) => {
                let mut serializer =
                    serializer.serialize_enum(40usize, "WordSpacing", "Length", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::MeetOrSlice {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Meet => {
                let mut serializer =
                    serializer.serialize_enum(41usize, "MeetOrSlice", "Meet", 0usize, 0usize)?;
                Ok(())
            }
            Self::Slice => {
                let mut serializer =
                    serializer.serialize_enum(41usize, "MeetOrSlice", "Slice", 1usize, 0usize)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::PreserveAspectRatio {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::None => {
                let mut serializer = serializer.serialize_enum(
                    42usize,
                    "PreserveAspectRatio",
                    "None",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::XMinYMin(p0) => {
                let mut serializer = serializer.serialize_enum(
                    42usize,
                    "PreserveAspectRatio",
                    "XMinYMin",
                    1usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMidYMin(p0) => {
                let mut serializer = serializer.serialize_enum(
                    42usize,
                    "PreserveAspectRatio",
                    "XMidYMin",
                    2usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMaxYMin(p0) => {
                let mut serializer = serializer.serialize_enum(
                    42usize,
                    "PreserveAspectRatio",
                    "XMaxYMin",
                    3usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMinYMid(p0) => {
                let mut serializer = serializer.serialize_enum(
                    42usize,
                    "PreserveAspectRatio",
                    "XMinYMid",
                    4usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMidYMid(p0) => {
                let mut serializer = serializer.serialize_enum(
                    42usize,
                    "PreserveAspectRatio",
                    "XMidYMid",
                    5usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMaxYMid(p0) => {
                let mut serializer = serializer.serialize_enum(
                    42usize,
                    "PreserveAspectRatio",
                    "XMaxYMid",
                    6usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMinYMax(p0) => {
                let mut serializer = serializer.serialize_enum(
                    42usize,
                    "PreserveAspectRatio",
                    "XMinYMax",
                    7usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMidYMax(p0) => {
                let mut serializer = serializer.serialize_enum(
                    42usize,
                    "PreserveAspectRatio",
                    "XMidYMax",
                    8usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMaxYMax(p0) => {
                let mut serializer = serializer.serialize_enum(
                    42usize,
                    "PreserveAspectRatio",
                    "XMaxYMax",
                    9usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextLayout {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(43usize, "TextLayout", 10usize)?;
        serializer.serialize_field(0usize, Some("writeMode"), &self.write_mode)?;
        serializer.serialize_field(1usize, Some("direction"), &self.direction)?;
        serializer.serialize_field(2usize, Some("unicodeBidi"), &self.unicode_bidi)?;
        serializer.serialize_field(3usize, Some("text-anchor"), &self.anchor)?;
        serializer.serialize_field(4usize, Some("dominantBaseline"), &self.dominant_baseline)?;
        serializer.serialize_field(5usize, Some("alignmentBaseline"), &self.alignment_baseline)?;
        serializer.serialize_field(6usize, Some("baselineShift"), &self.baseline_shift)?;
        serializer.serialize_field(7usize, Some("text-decoration"), &self.decoration)?;
        serializer.serialize_field(8usize, Some("letterSpacing"), &self.letter_spacing)?;
        serializer.serialize_field(9usize, Some("wordSpacing"), &self.word_spacing)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::WithTransform {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(44usize, "WithTransform", 1usize)?;
        serializer.serialize_field(0usize, Some("transform"), &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Id {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(45usize, "Id", 1usize)?;
        serializer.serialize_field(0usize, Some("id"), &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Fill {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(46usize, "Fill", 3usize)?;
        serializer.serialize_field(0usize, Some("fill"), &self.paint)?;
        serializer.serialize_field(1usize, Some("fill-rule"), &self.rule)?;
        serializer.serialize_field(2usize, Some("fill-opacity"), &self.opacity)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Stroke {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(47usize, "Stroke", 7usize)?;
        serializer.serialize_field(0usize, Some("stroke"), &self.paint)?;
        serializer.serialize_field(1usize, Some("stroke-width"), &self.width)?;
        serializer.serialize_field(2usize, Some("stroke-linecap"), &self.linecap)?;
        serializer.serialize_field(3usize, Some("stroke-linejoin"), &self.linejoin)?;
        serializer.serialize_field(4usize, Some("stroke-dasharray"), &self.dasharray)?;
        serializer.serialize_field(5usize, Some("stroke-dashoffset"), &self.dashoffset)?;
        serializer.serialize_field(6usize, Some("stroke-opacity"), &self.opacity)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Font {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(48usize, "Font", 6usize)?;
        serializer.serialize_field(0usize, Some("font-family"), &self.family)?;
        serializer.serialize_field(1usize, Some("font-style"), &self.style)?;
        serializer.serialize_field(2usize, Some("font-variant"), &self.variant)?;
        serializer.serialize_field(3usize, Some("font-weight"), &self.weight)?;
        serializer.serialize_field(4usize, Some("font-size"), &self.size)?;
        serializer.serialize_field(5usize, Some("font-stretch"), &self.stretch)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::EnableBackground {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(49usize, "EnableBackground", 1usize)?;
        serializer.serialize_field(0usize, Some("enable-background"), &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::WithFilter {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(50usize, "WithFilter", 1usize)?;
        serializer.serialize_field(0usize, Some("filter"), &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::WithClipPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(51usize, "WithClipPath", 1usize)?;
        serializer.serialize_field(0usize, Some("clip-path"), &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::WithMask {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(52usize, "WithMask", 1usize)?;
        serializer.serialize_field(0usize, Some("mask"), &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Opacity {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(53usize, "Opacity", 1usize)?;
        serializer.serialize_field(0usize, Some("opacity"), &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::ViewBox {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(54usize, "ViewBox", 5usize)?;
        serializer.serialize_field(0usize, Some("minx"), &self.minx)?;
        serializer.serialize_field(1usize, Some("miny"), &self.miny)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("aspect"), &self.aspect)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Canvas {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(56usize, "Canvas", 2usize)?;
        serializer.serialize_field(0usize, Some("width"), &self.width)?;
        serializer.serialize_field(1usize, Some("height"), &self.height)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Mask {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(57usize, "Mask", 6usize)?;
        serializer.serialize_field(0usize, Some("maskUnits"), &self.units)?;
        serializer.serialize_field(1usize, Some("contentUnits"), &self.content_units)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::ClipPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(58usize, "ClipPath", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Filter {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(59usize, "Filter", 7usize)?;
        serializer.serialize_field(0usize, Some("filterUnits"), &self.units)?;
        serializer.serialize_field(1usize, Some("primitiveUnits"), &self.primitive_units)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("res"), &self.res)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeDistantLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(60usize, "FeDistantLight", 2usize)?;
        serializer.serialize_field(0usize, Some("azimuth"), &self.azimuth)?;
        serializer.serialize_field(1usize, Some("elevation"), &self.elevation)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FePointLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(61usize, "FePointLight", 3usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("z"), &self.z)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeSpotLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(62usize, "FeSpotLight", 8usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("z"), &self.z)?;
        serializer.serialize_field(3usize, Some("pointAtX"), &self.point_at_x)?;
        serializer.serialize_field(4usize, Some("pointAtY"), &self.point_at_y)?;
        serializer.serialize_field(5usize, Some("pointAtZ"), &self.point_at_z)?;
        serializer.serialize_field(6usize, Some("specularExponent"), &self.specular_exponent)?;
        serializer.serialize_field(7usize, Some("limitingConeAngle"), &self.limiting_cone_angle)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeBlend {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(63usize, "FeBlend", 8usize)?;
        serializer.serialize_field(0usize, Some("mode"), &self.mode)?;
        serializer.serialize_field(1usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(2usize, Some("in2"), &self.in2)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeColorMatrixValues {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Matrix(p0) => {
                let mut serializer = serializer.serialize_enum(
                    64usize,
                    "FeColorMatrixValues",
                    "Matrix",
                    0usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Saturate(p0) => {
                let mut serializer = serializer.serialize_enum(
                    64usize,
                    "FeColorMatrixValues",
                    "Saturate",
                    1usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::HueRotate(p0) => {
                let mut serializer = serializer.serialize_enum(
                    64usize,
                    "FeColorMatrixValues",
                    "HueRotate",
                    2usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::LuminanceToAlpha => {
                let mut serializer = serializer.serialize_enum(
                    64usize,
                    "FeColorMatrixValues",
                    "LuminanceToAlpha",
                    3usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeColorMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(65usize, "FeColorMatrix", 7usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("values"), &self.values)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeFunc {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Identity => {
                let mut serializer =
                    serializer.serialize_enum(66usize, "FeFunc", "Identity", 0usize, 0usize)?;
                Ok(())
            }
            Self::Table(p0) => {
                let mut serializer =
                    serializer.serialize_enum(66usize, "FeFunc", "Table", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Discrete(p0) => {
                let mut serializer =
                    serializer.serialize_enum(66usize, "FeFunc", "Discrete", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Linear { slope, intercept } => {
                let mut serializer =
                    serializer.serialize_enum(66usize, "FeFunc", "Linear", 3usize, 2usize)?;
                serializer.serialize_field(0usize, Some("slope"), slope)?;
                serializer.serialize_field(1usize, Some("intercept"), intercept)?;
                Ok(())
            }
            Self::Gamma {
                amplitude,
                exponent,
                offset,
            } => {
                let mut serializer =
                    serializer.serialize_enum(66usize, "FeFunc", "Gamma", 4usize, 3usize)?;
                serializer.serialize_field(0usize, Some("amplitude"), amplitude)?;
                serializer.serialize_field(1usize, Some("exponent"), exponent)?;
                serializer.serialize_field(2usize, Some("offset"), offset)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeCompositeOperator {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Over => {
                let mut serializer = serializer.serialize_enum(
                    67usize,
                    "FeCompositeOperator",
                    "Over",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::In => {
                let mut serializer = serializer.serialize_enum(
                    67usize,
                    "FeCompositeOperator",
                    "In",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Out => {
                let mut serializer = serializer.serialize_enum(
                    67usize,
                    "FeCompositeOperator",
                    "Out",
                    2usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Atop => {
                let mut serializer = serializer.serialize_enum(
                    67usize,
                    "FeCompositeOperator",
                    "Atop",
                    3usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Xor => {
                let mut serializer = serializer.serialize_enum(
                    67usize,
                    "FeCompositeOperator",
                    "Xor",
                    4usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Arithmetic { k1, k2, k3, k4 } => {
                let mut serializer = serializer.serialize_enum(
                    67usize,
                    "FeCompositeOperator",
                    "Arithmetic",
                    5usize,
                    4usize,
                )?;
                serializer.serialize_field(0usize, Some("k1"), k1)?;
                serializer.serialize_field(1usize, Some("k2"), k2)?;
                serializer.serialize_field(2usize, Some("k3"), k3)?;
                serializer.serialize_field(3usize, Some("k4"), k4)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeConvolveMatrixEdgeMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Duplicate => {
                let mut serializer = serializer.serialize_enum(
                    68usize,
                    "FeConvolveMatrixEdgeMode",
                    "Duplicate",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Wrap => {
                let mut serializer = serializer.serialize_enum(
                    68usize,
                    "FeConvolveMatrixEdgeMode",
                    "Wrap",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::None => {
                let mut serializer = serializer.serialize_enum(
                    68usize,
                    "FeConvolveMatrixEdgeMode",
                    "None",
                    2usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeMorphologyOperator {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Erode => {
                let mut serializer = serializer.serialize_enum(
                    69usize,
                    "FeMorphologyOperator",
                    "Erode",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Dilate => {
                let mut serializer = serializer.serialize_enum(
                    69usize,
                    "FeMorphologyOperator",
                    "Dilate",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeStitchTiles {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::Stitch => {
                let mut serializer = serializer.serialize_enum(
                    70usize,
                    "FeStitchTiles",
                    "Stitch",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::NoStitch => {
                let mut serializer = serializer.serialize_enum(
                    70usize,
                    "FeStitchTiles",
                    "NoStitch",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeTurbulenceType {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        match self {
            Self::FractalNoise => {
                let mut serializer = serializer.serialize_enum(
                    71usize,
                    "FeTurbulenceType",
                    "FractalNoise",
                    0usize,
                    0usize,
                )?;
                Ok(())
            }
            Self::Turbulence => {
                let mut serializer = serializer.serialize_enum(
                    71usize,
                    "FeTurbulenceType",
                    "Turbulence",
                    1usize,
                    0usize,
                )?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeComponentTransfer {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(72usize, "FeComponentTransfer", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeFuncA {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(73usize, "FeFuncA", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeFuncR {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(74usize, "FeFuncR", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeFuncG {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(75usize, "FeFuncG", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeFuncB {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(76usize, "FeFuncB", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeComposite {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(77usize, "FeComposite", 8usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("in2"), &self.in2)?;
        serializer.serialize_field(2usize, Some("operator"), &self.operator)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeConvolveMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(78usize, "FeConvolveMatrix", 15usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("order"), &self.order)?;
        serializer.serialize_field(2usize, Some("kernel"), &self.kernel)?;
        serializer.serialize_field(3usize, Some("divisor"), &self.divisor)?;
        serializer.serialize_field(4usize, Some("bias"), &self.bias)?;
        serializer.serialize_field(5usize, Some("targetX"), &self.target_x)?;
        serializer.serialize_field(6usize, Some("targetY"), &self.target_y)?;
        serializer.serialize_field(7usize, Some("edgeMode"), &self.edge_mode)?;
        serializer.serialize_field(8usize, Some("kernelUnitLen"), &self.kernel_unit_len)?;
        serializer.serialize_field(9usize, Some("preserveAlpha"), &self.preserve_alpha)?;
        serializer.serialize_field(10usize, Some("x"), &self.x)?;
        serializer.serialize_field(11usize, Some("y"), &self.y)?;
        serializer.serialize_field(12usize, Some("width"), &self.width)?;
        serializer.serialize_field(13usize, Some("height"), &self.height)?;
        serializer.serialize_field(14usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeDiffuseLighting {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(79usize, "FeDiffuseLighting", 9usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("surfaceScale"), &self.surface_scale)?;
        serializer.serialize_field(2usize, Some("diffuseConstant"), &self.diffuse_constant)?;
        serializer.serialize_field(3usize, Some("kernelUnitLen"), &self.kernel_unit_len)?;
        serializer.serialize_field(4usize, Some("x"), &self.x)?;
        serializer.serialize_field(5usize, Some("y"), &self.y)?;
        serializer.serialize_field(6usize, Some("width"), &self.width)?;
        serializer.serialize_field(7usize, Some("height"), &self.height)?;
        serializer.serialize_field(8usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeDisplacementMap {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(80usize, "FeDisplacementMap", 10usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("in2"), &self.in2)?;
        serializer.serialize_field(2usize, Some("scale"), &self.scale)?;
        serializer.serialize_field(3usize, Some("xChannelSelector"), &self.x_channel_selector)?;
        serializer.serialize_field(4usize, Some("yChannelSelector"), &self.y_channel_selector)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeFlood {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(81usize, "FeFlood", 7usize)?;
        serializer.serialize_field(0usize, Some("color"), &self.color)?;
        serializer.serialize_field(1usize, Some("opacity"), &self.opacity)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeGaussianBlur {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(82usize, "FeGaussianBlur", 7usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("stdDeviation"), &self.std_deviation)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeMerge {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(83usize, "FeMerge", 5usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeMergeNode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(84usize, "FeMergeNode", 1usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeImage {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(85usize, "FeImage", 7usize)?;
        serializer.serialize_field(0usize, Some("href"), &self.href)?;
        serializer.serialize_field(1usize, Some("aspect"), &self.aspect)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeMorphology {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(86usize, "FeMorphology", 8usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("mode"), &self.mode)?;
        serializer.serialize_field(2usize, Some("radius"), &self.radius)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeOffset {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(87usize, "FeOffset", 8usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("dx"), &self.dx)?;
        serializer.serialize_field(2usize, Some("dy"), &self.dy)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeSpecularLighting {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(88usize, "FeSpecularLighting", 10usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("surfaceScale"), &self.surface_scale)?;
        serializer.serialize_field(2usize, Some("specularConstant"), &self.specular_constant)?;
        serializer.serialize_field(3usize, Some("specularExponent"), &self.specular_exponent)?;
        serializer.serialize_field(4usize, Some("kernelUnitLen"), &self.kernel_unit_len)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeTile {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(89usize, "FeTile", 6usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("x"), &self.x)?;
        serializer.serialize_field(2usize, Some("y"), &self.y)?;
        serializer.serialize_field(3usize, Some("width"), &self.width)?;
        serializer.serialize_field(4usize, Some("height"), &self.height)?;
        serializer.serialize_field(5usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeTurbulence {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(90usize, "FeTurbulence", 10usize)?;
        serializer.serialize_field(0usize, Some("baseFrequency"), &self.base_frequency)?;
        serializer.serialize_field(1usize, Some("numOctaves"), &self.num_octaves)?;
        serializer.serialize_field(2usize, Some("seed"), &self.seed)?;
        serializer.serialize_field(3usize, Some("stitchTiles"), &self.stitch_tiles)?;
        serializer.serialize_field(4usize, Some("type"), &self.r#type)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::LinearGradient {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(91usize, "LinearGradient", 7usize)?;
        serializer.serialize_field(0usize, Some("gradientUnits"), &self.units)?;
        serializer.serialize_field(1usize, Some("transform"), &self.transform)?;
        serializer.serialize_field(2usize, Some("x1"), &self.x1)?;
        serializer.serialize_field(3usize, Some("y1"), &self.y1)?;
        serializer.serialize_field(4usize, Some("x2"), &self.x2)?;
        serializer.serialize_field(5usize, Some("y2"), &self.y2)?;
        serializer.serialize_field(6usize, Some("spread"), &self.spread)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::RadialGradient {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(92usize, "RadialGradient", 8usize)?;
        serializer.serialize_field(0usize, Some("unit"), &self.unit)?;
        serializer.serialize_field(1usize, Some("transform"), &self.transform)?;
        serializer.serialize_field(2usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(3usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(4usize, Some("r"), &self.r)?;
        serializer.serialize_field(5usize, Some("fx"), &self.fx)?;
        serializer.serialize_field(6usize, Some("fy"), &self.fy)?;
        serializer.serialize_field(7usize, Some("spread"), &self.spread)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::GradientStop {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(93usize, "GradientStop", 3usize)?;
        serializer.serialize_field(0usize, Some("offset"), &self.offset)?;
        serializer.serialize_field(1usize, Some("stop-color"), &self.color)?;
        serializer.serialize_field(2usize, Some("stop-opacity"), &self.opacity)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Group {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(94usize, "Group", 0usize)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Path {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(95usize, "Path", 2usize)?;
        serializer.serialize_field(0usize, Some("events"), &self.events)?;
        serializer.serialize_field(1usize, Some("length"), &self.length)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(96usize, "Pattern", 7usize)?;
        serializer.serialize_field(0usize, Some("units"), &self.units)?;
        serializer.serialize_field(1usize, Some("contentUnits"), &self.content_units)?;
        serializer.serialize_field(2usize, Some("transform"), &self.transform)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Use {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(97usize, "Use", 1usize)?;
        serializer.serialize_field(0usize, Some("xlink:href"), &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Rect {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(98usize, "Rect", 6usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("rx"), &self.rx)?;
        serializer.serialize_field(5usize, Some("ry"), &self.ry)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Circle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(99usize, "Circle", 3usize)?;
        serializer.serialize_field(0usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(1usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(2usize, Some("r"), &self.r)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Ellipse {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(100usize, "Ellipse", 4usize)?;
        serializer.serialize_field(0usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(1usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(2usize, Some("rx"), &self.rx)?;
        serializer.serialize_field(3usize, Some("ry"), &self.ry)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Line {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(101usize, "Line", 4usize)?;
        serializer.serialize_field(0usize, Some("x1"), &self.x1)?;
        serializer.serialize_field(1usize, Some("y1"), &self.y1)?;
        serializer.serialize_field(2usize, Some("x2"), &self.x2)?;
        serializer.serialize_field(3usize, Some("y2"), &self.y2)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Polyline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(102usize, "Polyline", 1usize)?;
        serializer.serialize_field(0usize, Some("points"), &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Polygon {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(103usize, "Polygon", 1usize)?;
        serializer.serialize_field(0usize, Some("points"), &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Text {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(105usize, "Text", 7usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("dx"), &self.dx)?;
        serializer.serialize_field(3usize, Some("dy"), &self.dy)?;
        serializer.serialize_field(4usize, Some("rotate"), &self.rotate)?;
        serializer.serialize_field(5usize, Some("textLength"), &self.text_length)?;
        serializer.serialize_field(6usize, Some("lengthAdjust"), &self.length_adjust)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextSpan {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(106usize, "TextSpan", 7usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("dx"), &self.dx)?;
        serializer.serialize_field(3usize, Some("dy"), &self.dy)?;
        serializer.serialize_field(4usize, Some("rotate"), &self.rotate)?;
        serializer.serialize_field(5usize, Some("textLength"), &self.text_length)?;
        serializer.serialize_field(6usize, Some("lengthAdjust"), &self.length_adjust)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Characters {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(107usize, "Characters", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(108usize, "TextPath", 4usize)?;
        serializer.serialize_field(0usize, Some("startOffset"), &self.start_offset)?;
        serializer.serialize_field(1usize, Some("method"), &self.method)?;
        serializer.serialize_field(2usize, Some("spacing"), &self.spacing)?;
        serializer.serialize_field(3usize, Some("href"), &self.href)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Opcode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
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
