impl ml::rt::serde::Serialize for super::opcode::Angle {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(0usize, "Angle")?;
        match self {
            Self::Deg(p0) => {
                let mut serializer = serializer.serialize_field("Deg")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Grad(p0) => {
                let mut serializer = serializer.serialize_field("Grad")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Rad(p0) => {
                let mut serializer = serializer.serialize_field("Rad")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Length {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(1usize, "Length")?;
        match self {
            Self::Em(p0) => {
                let mut serializer = serializer.serialize_field("Em")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Ex(p0) => {
                let mut serializer = serializer.serialize_field("Ex")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Px(p0) => {
                let mut serializer = serializer.serialize_field("Px")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Inch(p0) => {
                let mut serializer = serializer.serialize_field("Inch")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Cm(p0) => {
                let mut serializer = serializer.serialize_field("Cm")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Mm(p0) => {
                let mut serializer = serializer.serialize_field("Mm")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Pt(p0) => {
                let mut serializer = serializer.serialize_field("Pt")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Pc(p0) => {
                let mut serializer = serializer.serialize_field("Pc")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Percent(p0) => {
                let mut serializer = serializer.serialize_field("Percent")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Color {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(2usize, "Color")?;
        match self {
            Self::Aliceblue => {
                let mut serializer = serializer.serialize_field("Aliceblue")?;
                Ok(())
            }
            Self::Antiquewhite => {
                let mut serializer = serializer.serialize_field("Antiquewhite")?;
                Ok(())
            }
            Self::Aqua => {
                let mut serializer = serializer.serialize_field("Aqua")?;
                Ok(())
            }
            Self::Aquamarine => {
                let mut serializer = serializer.serialize_field("Aquamarine")?;
                Ok(())
            }
            Self::Azure => {
                let mut serializer = serializer.serialize_field("Azure")?;
                Ok(())
            }
            Self::Beige => {
                let mut serializer = serializer.serialize_field("Beige")?;
                Ok(())
            }
            Self::Bisque => {
                let mut serializer = serializer.serialize_field("Bisque")?;
                Ok(())
            }
            Self::Black => {
                let mut serializer = serializer.serialize_field("Black")?;
                Ok(())
            }
            Self::Blanchedalmond => {
                let mut serializer = serializer.serialize_field("Blanchedalmond")?;
                Ok(())
            }
            Self::Blue => {
                let mut serializer = serializer.serialize_field("Blue")?;
                Ok(())
            }
            Self::Blueviolet => {
                let mut serializer = serializer.serialize_field("Blueviolet")?;
                Ok(())
            }
            Self::Brown => {
                let mut serializer = serializer.serialize_field("Brown")?;
                Ok(())
            }
            Self::Burlywood => {
                let mut serializer = serializer.serialize_field("Burlywood")?;
                Ok(())
            }
            Self::Cadetblue => {
                let mut serializer = serializer.serialize_field("Cadetblue")?;
                Ok(())
            }
            Self::Chartreuse => {
                let mut serializer = serializer.serialize_field("Chartreuse")?;
                Ok(())
            }
            Self::Chocolate => {
                let mut serializer = serializer.serialize_field("Chocolate")?;
                Ok(())
            }
            Self::Coral => {
                let mut serializer = serializer.serialize_field("Coral")?;
                Ok(())
            }
            Self::Cornflowerblue => {
                let mut serializer = serializer.serialize_field("Cornflowerblue")?;
                Ok(())
            }
            Self::Cornsilk => {
                let mut serializer = serializer.serialize_field("Cornsilk")?;
                Ok(())
            }
            Self::Crimson => {
                let mut serializer = serializer.serialize_field("Crimson")?;
                Ok(())
            }
            Self::Cyan => {
                let mut serializer = serializer.serialize_field("Cyan")?;
                Ok(())
            }
            Self::Darkblue => {
                let mut serializer = serializer.serialize_field("Darkblue")?;
                Ok(())
            }
            Self::Darkcyan => {
                let mut serializer = serializer.serialize_field("Darkcyan")?;
                Ok(())
            }
            Self::Darkgoldenrod => {
                let mut serializer = serializer.serialize_field("Darkgoldenrod")?;
                Ok(())
            }
            Self::Darkgray => {
                let mut serializer = serializer.serialize_field("Darkgray")?;
                Ok(())
            }
            Self::Darkgreen => {
                let mut serializer = serializer.serialize_field("Darkgreen")?;
                Ok(())
            }
            Self::Darkgrey => {
                let mut serializer = serializer.serialize_field("Darkgrey")?;
                Ok(())
            }
            Self::Darkkhaki => {
                let mut serializer = serializer.serialize_field("Darkkhaki")?;
                Ok(())
            }
            Self::Darkmagenta => {
                let mut serializer = serializer.serialize_field("Darkmagenta")?;
                Ok(())
            }
            Self::Darkolivegreen => {
                let mut serializer = serializer.serialize_field("Darkolivegreen")?;
                Ok(())
            }
            Self::Darkorange => {
                let mut serializer = serializer.serialize_field("Darkorange")?;
                Ok(())
            }
            Self::Darkorchid => {
                let mut serializer = serializer.serialize_field("Darkorchid")?;
                Ok(())
            }
            Self::Darkred => {
                let mut serializer = serializer.serialize_field("Darkred")?;
                Ok(())
            }
            Self::Darksalmon => {
                let mut serializer = serializer.serialize_field("Darksalmon")?;
                Ok(())
            }
            Self::Darkseagreen => {
                let mut serializer = serializer.serialize_field("Darkseagreen")?;
                Ok(())
            }
            Self::Darkslateblue => {
                let mut serializer = serializer.serialize_field("Darkslateblue")?;
                Ok(())
            }
            Self::Darkslategray => {
                let mut serializer = serializer.serialize_field("Darkslategray")?;
                Ok(())
            }
            Self::Darkslategrey => {
                let mut serializer = serializer.serialize_field("Darkslategrey")?;
                Ok(())
            }
            Self::Darkturquoise => {
                let mut serializer = serializer.serialize_field("Darkturquoise")?;
                Ok(())
            }
            Self::Darkviolet => {
                let mut serializer = serializer.serialize_field("Darkviolet")?;
                Ok(())
            }
            Self::Deeppink => {
                let mut serializer = serializer.serialize_field("Deeppink")?;
                Ok(())
            }
            Self::Deepskyblue => {
                let mut serializer = serializer.serialize_field("Deepskyblue")?;
                Ok(())
            }
            Self::Dimgray => {
                let mut serializer = serializer.serialize_field("Dimgray")?;
                Ok(())
            }
            Self::Dimgrey => {
                let mut serializer = serializer.serialize_field("Dimgrey")?;
                Ok(())
            }
            Self::Dodgerblue => {
                let mut serializer = serializer.serialize_field("Dodgerblue")?;
                Ok(())
            }
            Self::Firebrick => {
                let mut serializer = serializer.serialize_field("Firebrick")?;
                Ok(())
            }
            Self::Floralwhite => {
                let mut serializer = serializer.serialize_field("Floralwhite")?;
                Ok(())
            }
            Self::Forestgreen => {
                let mut serializer = serializer.serialize_field("Forestgreen")?;
                Ok(())
            }
            Self::Fuchsia => {
                let mut serializer = serializer.serialize_field("Fuchsia")?;
                Ok(())
            }
            Self::Gainsboro => {
                let mut serializer = serializer.serialize_field("Gainsboro")?;
                Ok(())
            }
            Self::Ghostwhite => {
                let mut serializer = serializer.serialize_field("Ghostwhite")?;
                Ok(())
            }
            Self::Gold => {
                let mut serializer = serializer.serialize_field("Gold")?;
                Ok(())
            }
            Self::Goldenrod => {
                let mut serializer = serializer.serialize_field("Goldenrod")?;
                Ok(())
            }
            Self::Gray => {
                let mut serializer = serializer.serialize_field("Gray")?;
                Ok(())
            }
            Self::Grey => {
                let mut serializer = serializer.serialize_field("Grey")?;
                Ok(())
            }
            Self::Green => {
                let mut serializer = serializer.serialize_field("Green")?;
                Ok(())
            }
            Self::Greenyellow => {
                let mut serializer = serializer.serialize_field("Greenyellow")?;
                Ok(())
            }
            Self::Honeydew => {
                let mut serializer = serializer.serialize_field("Honeydew")?;
                Ok(())
            }
            Self::Hotpink => {
                let mut serializer = serializer.serialize_field("Hotpink")?;
                Ok(())
            }
            Self::Indianred => {
                let mut serializer = serializer.serialize_field("Indianred")?;
                Ok(())
            }
            Self::Indigo => {
                let mut serializer = serializer.serialize_field("Indigo")?;
                Ok(())
            }
            Self::Ivory => {
                let mut serializer = serializer.serialize_field("Ivory")?;
                Ok(())
            }
            Self::Khaki => {
                let mut serializer = serializer.serialize_field("Khaki")?;
                Ok(())
            }
            Self::Lavender => {
                let mut serializer = serializer.serialize_field("Lavender")?;
                Ok(())
            }
            Self::Lavenderblush => {
                let mut serializer = serializer.serialize_field("Lavenderblush")?;
                Ok(())
            }
            Self::Lawngreen => {
                let mut serializer = serializer.serialize_field("Lawngreen")?;
                Ok(())
            }
            Self::Lemonchiffon => {
                let mut serializer = serializer.serialize_field("Lemonchiffon")?;
                Ok(())
            }
            Self::Lightblue => {
                let mut serializer = serializer.serialize_field("Lightblue")?;
                Ok(())
            }
            Self::Lightcoral => {
                let mut serializer = serializer.serialize_field("Lightcoral")?;
                Ok(())
            }
            Self::Lightcyan => {
                let mut serializer = serializer.serialize_field("Lightcyan")?;
                Ok(())
            }
            Self::Lightgoldenrodyellow => {
                let mut serializer = serializer.serialize_field("Lightgoldenrodyellow")?;
                Ok(())
            }
            Self::Lightgray => {
                let mut serializer = serializer.serialize_field("Lightgray")?;
                Ok(())
            }
            Self::Lightgreen => {
                let mut serializer = serializer.serialize_field("Lightgreen")?;
                Ok(())
            }
            Self::Lightgrey => {
                let mut serializer = serializer.serialize_field("Lightgrey")?;
                Ok(())
            }
            Self::Lightpink => {
                let mut serializer = serializer.serialize_field("Lightpink")?;
                Ok(())
            }
            Self::Lightsalmon => {
                let mut serializer = serializer.serialize_field("Lightsalmon")?;
                Ok(())
            }
            Self::Lightseagreen => {
                let mut serializer = serializer.serialize_field("Lightseagreen")?;
                Ok(())
            }
            Self::Lightskyblue => {
                let mut serializer = serializer.serialize_field("Lightskyblue")?;
                Ok(())
            }
            Self::Lightslategray => {
                let mut serializer = serializer.serialize_field("Lightslategray")?;
                Ok(())
            }
            Self::Lightslategrey => {
                let mut serializer = serializer.serialize_field("Lightslategrey")?;
                Ok(())
            }
            Self::Lightsteelblue => {
                let mut serializer = serializer.serialize_field("Lightsteelblue")?;
                Ok(())
            }
            Self::Lightyellow => {
                let mut serializer = serializer.serialize_field("Lightyellow")?;
                Ok(())
            }
            Self::Lime => {
                let mut serializer = serializer.serialize_field("Lime")?;
                Ok(())
            }
            Self::Limegreen => {
                let mut serializer = serializer.serialize_field("Limegreen")?;
                Ok(())
            }
            Self::Linen => {
                let mut serializer = serializer.serialize_field("Linen")?;
                Ok(())
            }
            Self::Magenta => {
                let mut serializer = serializer.serialize_field("Magenta")?;
                Ok(())
            }
            Self::Maroon => {
                let mut serializer = serializer.serialize_field("Maroon")?;
                Ok(())
            }
            Self::Mediumaquamarine => {
                let mut serializer = serializer.serialize_field("Mediumaquamarine")?;
                Ok(())
            }
            Self::Mediumblue => {
                let mut serializer = serializer.serialize_field("Mediumblue")?;
                Ok(())
            }
            Self::Mediumorchid => {
                let mut serializer = serializer.serialize_field("Mediumorchid")?;
                Ok(())
            }
            Self::Mediumpurple => {
                let mut serializer = serializer.serialize_field("Mediumpurple")?;
                Ok(())
            }
            Self::Mediumseagreen => {
                let mut serializer = serializer.serialize_field("Mediumseagreen")?;
                Ok(())
            }
            Self::Mediumslateblue => {
                let mut serializer = serializer.serialize_field("Mediumslateblue")?;
                Ok(())
            }
            Self::Mediumspringgreen => {
                let mut serializer = serializer.serialize_field("Mediumspringgreen")?;
                Ok(())
            }
            Self::Mediumturquoise => {
                let mut serializer = serializer.serialize_field("Mediumturquoise")?;
                Ok(())
            }
            Self::Mediumvioletred => {
                let mut serializer = serializer.serialize_field("Mediumvioletred")?;
                Ok(())
            }
            Self::Midnightblue => {
                let mut serializer = serializer.serialize_field("Midnightblue")?;
                Ok(())
            }
            Self::Mintcream => {
                let mut serializer = serializer.serialize_field("Mintcream")?;
                Ok(())
            }
            Self::Mistyrose => {
                let mut serializer = serializer.serialize_field("Mistyrose")?;
                Ok(())
            }
            Self::Moccasin => {
                let mut serializer = serializer.serialize_field("Moccasin")?;
                Ok(())
            }
            Self::Navajowhite => {
                let mut serializer = serializer.serialize_field("Navajowhite")?;
                Ok(())
            }
            Self::Navy => {
                let mut serializer = serializer.serialize_field("Navy")?;
                Ok(())
            }
            Self::Oldlace => {
                let mut serializer = serializer.serialize_field("Oldlace")?;
                Ok(())
            }
            Self::Olive => {
                let mut serializer = serializer.serialize_field("Olive")?;
                Ok(())
            }
            Self::Olivedrab => {
                let mut serializer = serializer.serialize_field("Olivedrab")?;
                Ok(())
            }
            Self::Orange => {
                let mut serializer = serializer.serialize_field("Orange")?;
                Ok(())
            }
            Self::Orangered => {
                let mut serializer = serializer.serialize_field("Orangered")?;
                Ok(())
            }
            Self::Orchid => {
                let mut serializer = serializer.serialize_field("Orchid")?;
                Ok(())
            }
            Self::Palegoldenrod => {
                let mut serializer = serializer.serialize_field("Palegoldenrod")?;
                Ok(())
            }
            Self::Palegreen => {
                let mut serializer = serializer.serialize_field("Palegreen")?;
                Ok(())
            }
            Self::Paleturquoise => {
                let mut serializer = serializer.serialize_field("Paleturquoise")?;
                Ok(())
            }
            Self::Palevioletred => {
                let mut serializer = serializer.serialize_field("Palevioletred")?;
                Ok(())
            }
            Self::Papayawhip => {
                let mut serializer = serializer.serialize_field("Papayawhip")?;
                Ok(())
            }
            Self::Peachpuff => {
                let mut serializer = serializer.serialize_field("Peachpuff")?;
                Ok(())
            }
            Self::Peru => {
                let mut serializer = serializer.serialize_field("Peru")?;
                Ok(())
            }
            Self::Pink => {
                let mut serializer = serializer.serialize_field("Pink")?;
                Ok(())
            }
            Self::Plum => {
                let mut serializer = serializer.serialize_field("Plum")?;
                Ok(())
            }
            Self::Powderblue => {
                let mut serializer = serializer.serialize_field("Powderblue")?;
                Ok(())
            }
            Self::Purple => {
                let mut serializer = serializer.serialize_field("Purple")?;
                Ok(())
            }
            Self::Red => {
                let mut serializer = serializer.serialize_field("Red")?;
                Ok(())
            }
            Self::Rosybrown => {
                let mut serializer = serializer.serialize_field("Rosybrown")?;
                Ok(())
            }
            Self::Royalblue => {
                let mut serializer = serializer.serialize_field("Royalblue")?;
                Ok(())
            }
            Self::Saddlebrown => {
                let mut serializer = serializer.serialize_field("Saddlebrown")?;
                Ok(())
            }
            Self::Salmon => {
                let mut serializer = serializer.serialize_field("Salmon")?;
                Ok(())
            }
            Self::Sandybrown => {
                let mut serializer = serializer.serialize_field("Sandybrown")?;
                Ok(())
            }
            Self::Seagreen => {
                let mut serializer = serializer.serialize_field("Seagreen")?;
                Ok(())
            }
            Self::Seashell => {
                let mut serializer = serializer.serialize_field("Seashell")?;
                Ok(())
            }
            Self::Sienna => {
                let mut serializer = serializer.serialize_field("Sienna")?;
                Ok(())
            }
            Self::Silver => {
                let mut serializer = serializer.serialize_field("Silver")?;
                Ok(())
            }
            Self::Skyblue => {
                let mut serializer = serializer.serialize_field("Skyblue")?;
                Ok(())
            }
            Self::Slateblue => {
                let mut serializer = serializer.serialize_field("Slateblue")?;
                Ok(())
            }
            Self::Slategray => {
                let mut serializer = serializer.serialize_field("Slategray")?;
                Ok(())
            }
            Self::Slategrey => {
                let mut serializer = serializer.serialize_field("Slategrey")?;
                Ok(())
            }
            Self::Snow => {
                let mut serializer = serializer.serialize_field("Snow")?;
                Ok(())
            }
            Self::Springgreen => {
                let mut serializer = serializer.serialize_field("Springgreen")?;
                Ok(())
            }
            Self::Steelblue => {
                let mut serializer = serializer.serialize_field("Steelblue")?;
                Ok(())
            }
            Self::Tan => {
                let mut serializer = serializer.serialize_field("Tan")?;
                Ok(())
            }
            Self::Teal => {
                let mut serializer = serializer.serialize_field("Teal")?;
                Ok(())
            }
            Self::Thistle => {
                let mut serializer = serializer.serialize_field("Thistle")?;
                Ok(())
            }
            Self::Tomato => {
                let mut serializer = serializer.serialize_field("Tomato")?;
                Ok(())
            }
            Self::Turquoise => {
                let mut serializer = serializer.serialize_field("Turquoise")?;
                Ok(())
            }
            Self::Violet => {
                let mut serializer = serializer.serialize_field("Violet")?;
                Ok(())
            }
            Self::Wheat => {
                let mut serializer = serializer.serialize_field("Wheat")?;
                Ok(())
            }
            Self::White => {
                let mut serializer = serializer.serialize_field("White")?;
                Ok(())
            }
            Self::Whitesmoke => {
                let mut serializer = serializer.serialize_field("Whitesmoke")?;
                Ok(())
            }
            Self::Yellow => {
                let mut serializer = serializer.serialize_field("Yellow")?;
                Ok(())
            }
            Self::Yellowgreen => {
                let mut serializer = serializer.serialize_field("Yellowgreen")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Rgb {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_data(3usize, "Rgb")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        serializer.serialize_field(2usize, None, &self.2)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Iri {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(4usize, "Iri")?;
        match self {
            Self::Local(p0) => {
                let mut serializer = serializer.serialize_field("Local")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Path(p0) => {
                let mut serializer = serializer.serialize_field("Path")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FuncIri {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_data(5usize, "FuncIri")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Point {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_data(6usize, "Point")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Percent {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_data(7usize, "Percent")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Paint {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(8usize, "Paint")?;
        match self {
            Self::None => {
                let mut serializer = serializer.serialize_field("None")?;
                Ok(())
            }
            Self::Color(p0) => {
                let mut serializer = serializer.serialize_field("Color")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Server(p0) => {
                let mut serializer = serializer.serialize_field("Server")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::NumberOptNumber {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_data(9usize, "NumberOptNumber")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Coords {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(10usize, "Coords")?;
        match self {
            Self::UserSpaceOnUse => {
                let mut serializer = serializer.serialize_field("UserSpaceOnUse")?;
                Ok(())
            }
            Self::ObjectBoundingBox => {
                let mut serializer = serializer.serialize_field("ObjectBoundingBox")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Transform {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(11usize, "Transform")?;
        match self {
            Self::Translate(p0, p1) => {
                let mut serializer = serializer.serialize_field("Translate")?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                Ok(())
            }
            Self::Matrix(p0) => {
                let mut serializer = serializer.serialize_field("Matrix")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Scale(p0, p1) => {
                let mut serializer = serializer.serialize_field("Scale")?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                Ok(())
            }
            Self::Rotate { angle, cx, cy } => {
                let mut serializer = serializer.serialize_field("Rotate")?;
                serializer.serialize_field(0usize, Some("angle"), angle)?;
                serializer.serialize_field(1usize, Some("cx"), cx)?;
                serializer.serialize_field(2usize, Some("cy"), cy)?;
                Ok(())
            }
            Self::SkewX(p0) => {
                let mut serializer = serializer.serialize_field("SkewX")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::SkewY(p0) => {
                let mut serializer = serializer.serialize_field("SkewY")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Channel {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(12usize, "Channel")?;
        match self {
            Self::R => {
                let mut serializer = serializer.serialize_field("R")?;
                Ok(())
            }
            Self::G => {
                let mut serializer = serializer.serialize_field("G")?;
                Ok(())
            }
            Self::B => {
                let mut serializer = serializer.serialize_field("B")?;
                Ok(())
            }
            Self::A => {
                let mut serializer = serializer.serialize_field("A")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::ClipRule {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(13usize, "ClipRule")?;
        match self {
            Self::Nonzero => {
                let mut serializer = serializer.serialize_field("Nonzero")?;
                Ok(())
            }
            Self::EvenOdd => {
                let mut serializer = serializer.serialize_field("EvenOdd")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::PathEvent {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(14usize, "PathEvent")?;
        match self {
            Self::Close => {
                let mut serializer = serializer.serialize_field("Close")?;
                Ok(())
            }
            Self::MoveTo(p0) => {
                let mut serializer = serializer.serialize_field("MoveTo")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::MoveToRelative(p0) => {
                let mut serializer = serializer.serialize_field("MoveToRelative")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::LineTo(p0) => {
                let mut serializer = serializer.serialize_field("LineTo")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::LineToRelative(p0) => {
                let mut serializer = serializer.serialize_field("LineToRelative")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Polyline(p0) => {
                let mut serializer = serializer.serialize_field("Polyline")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::PolylineRelative(p0) => {
                let mut serializer = serializer.serialize_field("PolylineRelative")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::CubicBezier {
                ctrl1,
                ctrl2,
                to_point,
            } => {
                let mut serializer = serializer.serialize_field("CubicBezier")?;
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
                let mut serializer = serializer.serialize_field("CubicBezierRelative")?;
                serializer.serialize_field(0usize, Some("ctrl1"), ctrl1)?;
                serializer.serialize_field(1usize, Some("ctrl2"), ctrl2)?;
                serializer.serialize_field(2usize, Some("to_point"), to_point)?;
                Ok(())
            }
            Self::CubicBezierSmooth { ctrl2, to_point } => {
                let mut serializer = serializer.serialize_field("CubicBezierSmooth")?;
                serializer.serialize_field(0usize, Some("ctrl2"), ctrl2)?;
                serializer.serialize_field(1usize, Some("to_point"), to_point)?;
                Ok(())
            }
            Self::CubicBezierSmoothRelative { ctrl2, to_point } => {
                let mut serializer = serializer.serialize_field("CubicBezierSmoothRelative")?;
                serializer.serialize_field(0usize, Some("ctrl2"), ctrl2)?;
                serializer.serialize_field(1usize, Some("to_point"), to_point)?;
                Ok(())
            }
            Self::QuadraticBezier { ctrl, to_point } => {
                let mut serializer = serializer.serialize_field("QuadraticBezier")?;
                serializer.serialize_field(0usize, Some("ctrl"), ctrl)?;
                serializer.serialize_field(1usize, Some("to_point"), to_point)?;
                Ok(())
            }
            Self::QuadraticBezierRelative { ctrl, to_point } => {
                let mut serializer = serializer.serialize_field("QuadraticBezierRelative")?;
                serializer.serialize_field(0usize, Some("ctrl"), ctrl)?;
                serializer.serialize_field(1usize, Some("to_point"), to_point)?;
                Ok(())
            }
            Self::QuadraticBezierSmooth(p0) => {
                let mut serializer = serializer.serialize_field("QuadraticBezierSmooth")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::QuadraticBezierSmoothRelative(p0) => {
                let mut serializer = serializer.serialize_field("QuadraticBezierSmoothRelative")?;
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
                let mut serializer = serializer.serialize_field("Arc")?;
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
                let mut serializer = serializer.serialize_field("ArcRelative")?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(15usize, "FillRule")?;
        match self {
            Self::Nonzero => {
                let mut serializer = serializer.serialize_field("Nonzero")?;
                Ok(())
            }
            Self::EvenOdd => {
                let mut serializer = serializer.serialize_field("EvenOdd")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::StrokeLineCap {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(16usize, "StrokeLineCap")?;
        match self {
            Self::Butt => {
                let mut serializer = serializer.serialize_field("Butt")?;
                Ok(())
            }
            Self::Round => {
                let mut serializer = serializer.serialize_field("Round")?;
                Ok(())
            }
            Self::Square => {
                let mut serializer = serializer.serialize_field("Square")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::StrokeLineJoin {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(17usize, "StrokeLineJoin")?;
        match self {
            Self::Miter(p0) => {
                let mut serializer = serializer.serialize_field("Miter")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Round => {
                let mut serializer = serializer.serialize_field("Round")?;
                Ok(())
            }
            Self::Bevel => {
                let mut serializer = serializer.serialize_field("Bevel")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::SpreadMethod {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(18usize, "SpreadMethod")?;
        match self {
            Self::Pad => {
                let mut serializer = serializer.serialize_field("Pad")?;
                Ok(())
            }
            Self::Reflect => {
                let mut serializer = serializer.serialize_field("Reflect")?;
                Ok(())
            }
            Self::Repeat => {
                let mut serializer = serializer.serialize_field("Repeat")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FontStyle {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(19usize, "FontStyle")?;
        match self {
            Self::Normal => {
                let mut serializer = serializer.serialize_field("Normal")?;
                Ok(())
            }
            Self::Italic => {
                let mut serializer = serializer.serialize_field("Italic")?;
                Ok(())
            }
            Self::Oblique => {
                let mut serializer = serializer.serialize_field("Oblique")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FontVariant {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(20usize, "FontVariant")?;
        match self {
            Self::Normal => {
                let mut serializer = serializer.serialize_field("Normal")?;
                Ok(())
            }
            Self::SmallCaps => {
                let mut serializer = serializer.serialize_field("SmallCaps")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FontWeight {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(21usize, "FontWeight")?;
        match self {
            Self::Normal => {
                let mut serializer = serializer.serialize_field("Normal")?;
                Ok(())
            }
            Self::Bold => {
                let mut serializer = serializer.serialize_field("Bold")?;
                Ok(())
            }
            Self::Bolder => {
                let mut serializer = serializer.serialize_field("Bolder")?;
                Ok(())
            }
            Self::Lighter => {
                let mut serializer = serializer.serialize_field("Lighter")?;
                Ok(())
            }
            Self::W100 => {
                let mut serializer = serializer.serialize_field("W100")?;
                Ok(())
            }
            Self::W200 => {
                let mut serializer = serializer.serialize_field("W200")?;
                Ok(())
            }
            Self::W300 => {
                let mut serializer = serializer.serialize_field("W300")?;
                Ok(())
            }
            Self::W400 => {
                let mut serializer = serializer.serialize_field("W400")?;
                Ok(())
            }
            Self::W500 => {
                let mut serializer = serializer.serialize_field("W500")?;
                Ok(())
            }
            Self::W600 => {
                let mut serializer = serializer.serialize_field("W600")?;
                Ok(())
            }
            Self::W700 => {
                let mut serializer = serializer.serialize_field("W700")?;
                Ok(())
            }
            Self::W800 => {
                let mut serializer = serializer.serialize_field("W800")?;
                Ok(())
            }
            Self::W900 => {
                let mut serializer = serializer.serialize_field("W900")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FontFamily {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(22usize, "FontFamily")?;
        match self {
            Self::Serif => {
                let mut serializer = serializer.serialize_field("Serif")?;
                Ok(())
            }
            Self::SansSerif => {
                let mut serializer = serializer.serialize_field("SansSerif")?;
                Ok(())
            }
            Self::Cursive => {
                let mut serializer = serializer.serialize_field("Cursive")?;
                Ok(())
            }
            Self::Fantasy => {
                let mut serializer = serializer.serialize_field("Fantasy")?;
                Ok(())
            }
            Self::Monospace => {
                let mut serializer = serializer.serialize_field("Monospace")?;
                Ok(())
            }
            Self::Generic(p0) => {
                let mut serializer = serializer.serialize_field("Generic")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FontStretch {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(23usize, "FontStretch")?;
        match self {
            Self::Normal => {
                let mut serializer = serializer.serialize_field("Normal")?;
                Ok(())
            }
            Self::Wider => {
                let mut serializer = serializer.serialize_field("Wider")?;
                Ok(())
            }
            Self::Narrower => {
                let mut serializer = serializer.serialize_field("Narrower")?;
                Ok(())
            }
            Self::UltraCondensed => {
                let mut serializer = serializer.serialize_field("UltraCondensed")?;
                Ok(())
            }
            Self::ExtraCondensed => {
                let mut serializer = serializer.serialize_field("ExtraCondensed")?;
                Ok(())
            }
            Self::Condensed => {
                let mut serializer = serializer.serialize_field("Condensed")?;
                Ok(())
            }
            Self::SemiCondensed => {
                let mut serializer = serializer.serialize_field("SemiCondensed")?;
                Ok(())
            }
            Self::SemiExpanded => {
                let mut serializer = serializer.serialize_field("SemiExpanded")?;
                Ok(())
            }
            Self::Expanded => {
                let mut serializer = serializer.serialize_field("Expanded")?;
                Ok(())
            }
            Self::ExtraExpanded => {
                let mut serializer = serializer.serialize_field("ExtraExpanded")?;
                Ok(())
            }
            Self::UltraExpanded => {
                let mut serializer = serializer.serialize_field("UltraExpanded")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::Background {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(24usize, "Background")?;
        match self {
            Self::Accumulate => {
                let mut serializer = serializer.serialize_field("Accumulate")?;
                Ok(())
            }
            Self::New {
                x,
                y,
                width,
                height,
            } => {
                let mut serializer = serializer.serialize_field("New")?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(25usize, "FeIn")?;
        match self {
            Self::SourceGraphic => {
                let mut serializer = serializer.serialize_field("SourceGraphic")?;
                Ok(())
            }
            Self::SourceAlpha => {
                let mut serializer = serializer.serialize_field("SourceAlpha")?;
                Ok(())
            }
            Self::BackgroundImage => {
                let mut serializer = serializer.serialize_field("BackgroundImage")?;
                Ok(())
            }
            Self::BackgroundAlpha => {
                let mut serializer = serializer.serialize_field("BackgroundAlpha")?;
                Ok(())
            }
            Self::FillPaint => {
                let mut serializer = serializer.serialize_field("FillPaint")?;
                Ok(())
            }
            Self::StrokePaint => {
                let mut serializer = serializer.serialize_field("StrokePaint")?;
                Ok(())
            }
            Self::Result(p0) => {
                let mut serializer = serializer.serialize_field("Result")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeOut {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(26usize, "FeOut")?;
        match self {
            Self::Position => {
                let mut serializer = serializer.serialize_field("Position")?;
                Ok(())
            }
            Self::Named(p0) => {
                let mut serializer = serializer.serialize_field("Named")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeBlendMode {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(27usize, "FeBlendMode")?;
        match self {
            Self::Normal => {
                let mut serializer = serializer.serialize_field("Normal")?;
                Ok(())
            }
            Self::Multiply => {
                let mut serializer = serializer.serialize_field("Multiply")?;
                Ok(())
            }
            Self::Screen => {
                let mut serializer = serializer.serialize_field("Screen")?;
                Ok(())
            }
            Self::Darken => {
                let mut serializer = serializer.serialize_field("Darken")?;
                Ok(())
            }
            Self::Lighten => {
                let mut serializer = serializer.serialize_field("Lighten")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextLengthAdjust {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(28usize, "TextLengthAdjust")?;
        match self {
            Self::Spacing => {
                let mut serializer = serializer.serialize_field("Spacing")?;
                Ok(())
            }
            Self::SpacingAndGlyphs => {
                let mut serializer = serializer.serialize_field("SpacingAndGlyphs")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::WritingMode {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(29usize, "WritingMode")?;
        match self {
            Self::LrTb => {
                let mut serializer = serializer.serialize_field("LrTb")?;
                Ok(())
            }
            Self::RlTb => {
                let mut serializer = serializer.serialize_field("RlTb")?;
                Ok(())
            }
            Self::TbRl => {
                let mut serializer = serializer.serialize_field("TbRl")?;
                Ok(())
            }
            Self::Lr => {
                let mut serializer = serializer.serialize_field("Lr")?;
                Ok(())
            }
            Self::Rl => {
                let mut serializer = serializer.serialize_field("Rl")?;
                Ok(())
            }
            Self::Tb => {
                let mut serializer = serializer.serialize_field("Tb")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextDirection {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(30usize, "TextDirection")?;
        match self {
            Self::Ltr => {
                let mut serializer = serializer.serialize_field("Ltr")?;
                Ok(())
            }
            Self::Rtl => {
                let mut serializer = serializer.serialize_field("Rtl")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::UnicodeBidi {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(31usize, "UnicodeBidi")?;
        match self {
            Self::Normal => {
                let mut serializer = serializer.serialize_field("Normal")?;
                Ok(())
            }
            Self::Embed => {
                let mut serializer = serializer.serialize_field("Embed")?;
                Ok(())
            }
            Self::BidiOverride => {
                let mut serializer = serializer.serialize_field("BidiOverride")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextAnchor {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(32usize, "TextAnchor")?;
        match self {
            Self::Start => {
                let mut serializer = serializer.serialize_field("Start")?;
                Ok(())
            }
            Self::Middle => {
                let mut serializer = serializer.serialize_field("Middle")?;
                Ok(())
            }
            Self::End => {
                let mut serializer = serializer.serialize_field("End")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::DominantBaseline {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(33usize, "DominantBaseline")?;
        match self {
            Self::Auto => {
                let mut serializer = serializer.serialize_field("Auto")?;
                Ok(())
            }
            Self::UseScript => {
                let mut serializer = serializer.serialize_field("UseScript")?;
                Ok(())
            }
            Self::NoChange => {
                let mut serializer = serializer.serialize_field("NoChange")?;
                Ok(())
            }
            Self::ResetSize => {
                let mut serializer = serializer.serialize_field("ResetSize")?;
                Ok(())
            }
            Self::Ideographic => {
                let mut serializer = serializer.serialize_field("Ideographic")?;
                Ok(())
            }
            Self::Alphabetic => {
                let mut serializer = serializer.serialize_field("Alphabetic")?;
                Ok(())
            }
            Self::Hanging => {
                let mut serializer = serializer.serialize_field("Hanging")?;
                Ok(())
            }
            Self::Mathematical => {
                let mut serializer = serializer.serialize_field("Mathematical")?;
                Ok(())
            }
            Self::Central => {
                let mut serializer = serializer.serialize_field("Central")?;
                Ok(())
            }
            Self::Middle => {
                let mut serializer = serializer.serialize_field("Middle")?;
                Ok(())
            }
            Self::TextAfterEdge => {
                let mut serializer = serializer.serialize_field("TextAfterEdge")?;
                Ok(())
            }
            Self::TextBeforeEdge => {
                let mut serializer = serializer.serialize_field("TextBeforeEdge")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::AlignmentBaseline {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(34usize, "AlignmentBaseline")?;
        match self {
            Self::Auto => {
                let mut serializer = serializer.serialize_field("Auto")?;
                Ok(())
            }
            Self::Baseline => {
                let mut serializer = serializer.serialize_field("Baseline")?;
                Ok(())
            }
            Self::BeforeEdge => {
                let mut serializer = serializer.serialize_field("BeforeEdge")?;
                Ok(())
            }
            Self::TextBeforeEdge => {
                let mut serializer = serializer.serialize_field("TextBeforeEdge")?;
                Ok(())
            }
            Self::Middle => {
                let mut serializer = serializer.serialize_field("Middle")?;
                Ok(())
            }
            Self::Central => {
                let mut serializer = serializer.serialize_field("Central")?;
                Ok(())
            }
            Self::AfterEdge => {
                let mut serializer = serializer.serialize_field("AfterEdge")?;
                Ok(())
            }
            Self::TextAfterEdge => {
                let mut serializer = serializer.serialize_field("TextAfterEdge")?;
                Ok(())
            }
            Self::Ideographic => {
                let mut serializer = serializer.serialize_field("Ideographic")?;
                Ok(())
            }
            Self::Alphabetic => {
                let mut serializer = serializer.serialize_field("Alphabetic")?;
                Ok(())
            }
            Self::Hanging => {
                let mut serializer = serializer.serialize_field("Hanging")?;
                Ok(())
            }
            Self::Mathematical => {
                let mut serializer = serializer.serialize_field("Mathematical")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::BaselineShift {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(35usize, "BaselineShift")?;
        match self {
            Self::Baseline => {
                let mut serializer = serializer.serialize_field("Baseline")?;
                Ok(())
            }
            Self::SubScripts => {
                let mut serializer = serializer.serialize_field("SubScripts")?;
                Ok(())
            }
            Self::SuperScripts => {
                let mut serializer = serializer.serialize_field("SuperScripts")?;
                Ok(())
            }
            Self::Value(p0) => {
                let mut serializer = serializer.serialize_field("Value")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextDecoration {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(36usize, "TextDecoration")?;
        match self {
            Self::Underline => {
                let mut serializer = serializer.serialize_field("Underline")?;
                Ok(())
            }
            Self::Overline => {
                let mut serializer = serializer.serialize_field("Overline")?;
                Ok(())
            }
            Self::LineThrough => {
                let mut serializer = serializer.serialize_field("LineThrough")?;
                Ok(())
            }
            Self::Blink => {
                let mut serializer = serializer.serialize_field("Blink")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextPathMethod {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(37usize, "TextPathMethod")?;
        match self {
            Self::Align => {
                let mut serializer = serializer.serialize_field("Align")?;
                Ok(())
            }
            Self::Stretch => {
                let mut serializer = serializer.serialize_field("Stretch")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextPathSpacing {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(38usize, "TextPathSpacing")?;
        match self {
            Self::Auto => {
                let mut serializer = serializer.serialize_field("Auto")?;
                Ok(())
            }
            Self::Exact => {
                let mut serializer = serializer.serialize_field("Exact")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::LetterSpacing {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(39usize, "LetterSpacing")?;
        match self {
            Self::Normal => {
                let mut serializer = serializer.serialize_field("Normal")?;
                Ok(())
            }
            Self::Length(p0) => {
                let mut serializer = serializer.serialize_field("Length")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::WordSpacing {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(40usize, "WordSpacing")?;
        match self {
            Self::Normal => {
                let mut serializer = serializer.serialize_field("Normal")?;
                Ok(())
            }
            Self::Length(p0) => {
                let mut serializer = serializer.serialize_field("Length")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::MeetOrSlice {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(41usize, "MeetOrSlice")?;
        match self {
            Self::Meet => {
                let mut serializer = serializer.serialize_field("Meet")?;
                Ok(())
            }
            Self::Slice => {
                let mut serializer = serializer.serialize_field("Slice")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::PreserveAspectRatio {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(42usize, "PreserveAspectRatio")?;
        match self {
            Self::None => {
                let mut serializer = serializer.serialize_field("None")?;
                Ok(())
            }
            Self::XMinYMin(p0) => {
                let mut serializer = serializer.serialize_field("XMinYMin")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMidYMin(p0) => {
                let mut serializer = serializer.serialize_field("XMidYMin")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMaxYMin(p0) => {
                let mut serializer = serializer.serialize_field("XMaxYMin")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMinYMid(p0) => {
                let mut serializer = serializer.serialize_field("XMinYMid")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMidYMid(p0) => {
                let mut serializer = serializer.serialize_field("XMidYMid")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMaxYMid(p0) => {
                let mut serializer = serializer.serialize_field("XMaxYMid")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMinYMax(p0) => {
                let mut serializer = serializer.serialize_field("XMinYMax")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMidYMax(p0) => {
                let mut serializer = serializer.serialize_field("XMidYMax")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::XMaxYMax(p0) => {
                let mut serializer = serializer.serialize_field("XMaxYMax")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextLayout {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(43usize, "TextLayout")?;
        serializer.serialize_field(0usize, Some("write_mode"), &self.write_mode)?;
        serializer.serialize_field(1usize, Some("direction"), &self.direction)?;
        serializer.serialize_field(2usize, Some("unicode_bidi"), &self.unicode_bidi)?;
        serializer.serialize_field(3usize, Some("anchor"), &self.anchor)?;
        serializer.serialize_field(4usize, Some("dominant_baseline"), &self.dominant_baseline)?;
        serializer.serialize_field(5usize, Some("alignment_baseline"), &self.alignment_baseline)?;
        serializer.serialize_field(6usize, Some("baseline_shift"), &self.baseline_shift)?;
        serializer.serialize_field(7usize, Some("decoration"), &self.decoration)?;
        serializer.serialize_field(8usize, Some("letter_spacing"), &self.letter_spacing)?;
        serializer.serialize_field(9usize, Some("word_spacing"), &self.word_spacing)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::WithTransform {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(44usize, "WithTransform")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Id {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(45usize, "Id")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Fill {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(46usize, "Fill")?;
        serializer.serialize_field(0usize, Some("paint"), &self.paint)?;
        serializer.serialize_field(1usize, Some("rule"), &self.rule)?;
        serializer.serialize_field(2usize, Some("opacity"), &self.opacity)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Stroke {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(47usize, "Stroke")?;
        serializer.serialize_field(0usize, Some("paint"), &self.paint)?;
        serializer.serialize_field(1usize, Some("width"), &self.width)?;
        serializer.serialize_field(2usize, Some("linecap"), &self.linecap)?;
        serializer.serialize_field(3usize, Some("linejoin"), &self.linejoin)?;
        serializer.serialize_field(4usize, Some("dasharray"), &self.dasharray)?;
        serializer.serialize_field(5usize, Some("dashoffset"), &self.dashoffset)?;
        serializer.serialize_field(6usize, Some("opacity"), &self.opacity)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Font {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(48usize, "Font")?;
        serializer.serialize_field(0usize, Some("family"), &self.family)?;
        serializer.serialize_field(1usize, Some("style"), &self.style)?;
        serializer.serialize_field(2usize, Some("variant"), &self.variant)?;
        serializer.serialize_field(3usize, Some("weight"), &self.weight)?;
        serializer.serialize_field(4usize, Some("size"), &self.size)?;
        serializer.serialize_field(5usize, Some("stretch"), &self.stretch)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::EnableBackground {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(49usize, "EnableBackground")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::WithFilter {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(50usize, "WithFilter")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::WithClipPath {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(51usize, "WithClipPath")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::WithMask {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(52usize, "WithMask")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Opacity {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(53usize, "Opacity")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::ViewBox {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_attr(54usize, "ViewBox")?;
        serializer.serialize_field(0usize, Some("minx"), &self.minx)?;
        serializer.serialize_field(1usize, Some("miny"), &self.miny)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("aspect"), &self.aspect)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Canvas {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(56usize, "Canvas")?;
        serializer.serialize_field(0usize, Some("width"), &self.width)?;
        serializer.serialize_field(1usize, Some("height"), &self.height)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Mask {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(57usize, "Mask")?;
        serializer.serialize_field(0usize, Some("units"), &self.units)?;
        serializer.serialize_field(1usize, Some("content_units"), &self.content_units)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::ClipPath {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(58usize, "ClipPath")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Filter {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(59usize, "Filter")?;
        serializer.serialize_field(0usize, Some("units"), &self.units)?;
        serializer.serialize_field(1usize, Some("primitive_units"), &self.primitive_units)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("res"), &self.res)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeDistantLight {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(60usize, "FeDistantLight")?;
        serializer.serialize_field(0usize, Some("azimuth"), &self.azimuth)?;
        serializer.serialize_field(1usize, Some("elevation"), &self.elevation)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FePointLight {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(61usize, "FePointLight")?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("z"), &self.z)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeSpotLight {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(62usize, "FeSpotLight")?;
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
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeBlend {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(63usize, "FeBlend")?;
        serializer.serialize_field(0usize, Some("mode"), &self.mode)?;
        serializer.serialize_field(1usize, Some("r#in"), &self.r#in)?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(64usize, "FeColorMatrixValues")?;
        match self {
            Self::Matrix(p0) => {
                let mut serializer = serializer.serialize_field("Matrix")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Saturate(p0) => {
                let mut serializer = serializer.serialize_field("Saturate")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::HueRotate(p0) => {
                let mut serializer = serializer.serialize_field("HueRotate")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::LuminanceToAlpha => {
                let mut serializer = serializer.serialize_field("LuminanceToAlpha")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeColorMatrix {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(65usize, "FeColorMatrix")?;
        serializer.serialize_field(0usize, Some("r#in"), &self.r#in)?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(66usize, "FeFunc")?;
        match self {
            Self::Identity => {
                let mut serializer = serializer.serialize_field("Identity")?;
                Ok(())
            }
            Self::Table(p0) => {
                let mut serializer = serializer.serialize_field("Table")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Discrete(p0) => {
                let mut serializer = serializer.serialize_field("Discrete")?;
                serializer.serialize_field(0usize, None, p0)?;
                Ok(())
            }
            Self::Linear { slope, intercept } => {
                let mut serializer = serializer.serialize_field("Linear")?;
                serializer.serialize_field(0usize, Some("slope"), slope)?;
                serializer.serialize_field(1usize, Some("intercept"), intercept)?;
                Ok(())
            }
            Self::Gamma {
                amplitude,
                exponent,
                offset,
            } => {
                let mut serializer = serializer.serialize_field("Gamma")?;
                serializer.serialize_field(0usize, Some("amplitude"), amplitude)?;
                serializer.serialize_field(1usize, Some("exponent"), exponent)?;
                serializer.serialize_field(2usize, Some("offset"), offset)?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeCompositeOperator {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(67usize, "FeCompositeOperator")?;
        match self {
            Self::Over => {
                let mut serializer = serializer.serialize_field("Over")?;
                Ok(())
            }
            Self::In => {
                let mut serializer = serializer.serialize_field("In")?;
                Ok(())
            }
            Self::Out => {
                let mut serializer = serializer.serialize_field("Out")?;
                Ok(())
            }
            Self::Atop => {
                let mut serializer = serializer.serialize_field("Atop")?;
                Ok(())
            }
            Self::Xor => {
                let mut serializer = serializer.serialize_field("Xor")?;
                Ok(())
            }
            Self::Arithmetic { k1, k2, k3, k4 } => {
                let mut serializer = serializer.serialize_field("Arithmetic")?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(68usize, "FeConvolveMatrixEdgeMode")?;
        match self {
            Self::Duplicate => {
                let mut serializer = serializer.serialize_field("Duplicate")?;
                Ok(())
            }
            Self::Wrap => {
                let mut serializer = serializer.serialize_field("Wrap")?;
                Ok(())
            }
            Self::None => {
                let mut serializer = serializer.serialize_field("None")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeMorphologyOperator {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(69usize, "FeMorphologyOperator")?;
        match self {
            Self::Erode => {
                let mut serializer = serializer.serialize_field("Erode")?;
                Ok(())
            }
            Self::Dilate => {
                let mut serializer = serializer.serialize_field("Dilate")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeStitchTiles {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(70usize, "FeStitchTiles")?;
        match self {
            Self::Stitch => {
                let mut serializer = serializer.serialize_field("Stitch")?;
                Ok(())
            }
            Self::NoStitch => {
                let mut serializer = serializer.serialize_field("NoStitch")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeTurbulenceType {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeEnum;
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_enum(71usize, "FeTurbulenceType")?;
        match self {
            Self::FractalNoise => {
                let mut serializer = serializer.serialize_field("FractalNoise")?;
                Ok(())
            }
            Self::Turbulence => {
                let mut serializer = serializer.serialize_field("Turbulence")?;
                Ok(())
            }
        }
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeComponentTransfer {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(72usize, "FeComponentTransfer")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeFuncA {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(73usize, "FeFuncA")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeFuncR {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(74usize, "FeFuncR")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeFuncG {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(75usize, "FeFuncG")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeFuncB {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(76usize, "FeFuncB")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeComposite {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(77usize, "FeComposite")?;
        serializer.serialize_field(0usize, Some("r#in"), &self.r#in)?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(78usize, "FeConvolveMatrix")?;
        serializer.serialize_field(0usize, Some("r#in"), &self.r#in)?;
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
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeDiffuseLighting {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(79usize, "FeDiffuseLighting")?;
        serializer.serialize_field(0usize, Some("r#in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("surface_scale"), &self.surface_scale)?;
        serializer.serialize_field(2usize, Some("diffuse_constant"), &self.diffuse_constant)?;
        serializer.serialize_field(3usize, Some("kernel_unit_len"), &self.kernel_unit_len)?;
        serializer.serialize_field(4usize, Some("x"), &self.x)?;
        serializer.serialize_field(5usize, Some("y"), &self.y)?;
        serializer.serialize_field(6usize, Some("width"), &self.width)?;
        serializer.serialize_field(7usize, Some("height"), &self.height)?;
        serializer.serialize_field(8usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeDisplacementMap {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(80usize, "FeDisplacementMap")?;
        serializer.serialize_field(0usize, Some("r#in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("in2"), &self.in2)?;
        serializer.serialize_field(2usize, Some("scale"), &self.scale)?;
        serializer.serialize_field(3usize, Some("x_channel_selector"), &self.x_channel_selector)?;
        serializer.serialize_field(4usize, Some("y_channel_selector"), &self.y_channel_selector)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeFlood {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(81usize, "FeFlood")?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(82usize, "FeGaussianBlur")?;
        serializer.serialize_field(0usize, Some("r#in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("std_deviation"), &self.std_deviation)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeMerge {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(83usize, "FeMerge")?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeMergeNode {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(84usize, "FeMergeNode")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeImage {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(85usize, "FeImage")?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(86usize, "FeMorphology")?;
        serializer.serialize_field(0usize, Some("r#in"), &self.r#in)?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(87usize, "FeOffset")?;
        serializer.serialize_field(0usize, Some("r#in"), &self.r#in)?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(88usize, "FeSpecularLighting")?;
        serializer.serialize_field(0usize, Some("r#in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("surface_scale"), &self.surface_scale)?;
        serializer.serialize_field(2usize, Some("specular_constant"), &self.specular_constant)?;
        serializer.serialize_field(3usize, Some("specular_exponent"), &self.specular_exponent)?;
        serializer.serialize_field(4usize, Some("kernel_unit_len"), &self.kernel_unit_len)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeTile {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(89usize, "FeTile")?;
        serializer.serialize_field(0usize, Some("r#in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("x"), &self.x)?;
        serializer.serialize_field(2usize, Some("y"), &self.y)?;
        serializer.serialize_field(3usize, Some("width"), &self.width)?;
        serializer.serialize_field(4usize, Some("height"), &self.height)?;
        serializer.serialize_field(5usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::FeTurbulence {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(90usize, "FeTurbulence")?;
        serializer.serialize_field(0usize, Some("base_frequency"), &self.base_frequency)?;
        serializer.serialize_field(1usize, Some("num_octaves"), &self.num_octaves)?;
        serializer.serialize_field(2usize, Some("seed"), &self.seed)?;
        serializer.serialize_field(3usize, Some("stitch_tiles"), &self.stitch_tiles)?;
        serializer.serialize_field(4usize, Some("r#type"), &self.r#type)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::LinearGradient {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(91usize, "LinearGradient")?;
        serializer.serialize_field(0usize, Some("units"), &self.units)?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(92usize, "RadialGradient")?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(93usize, "GradientStop")?;
        serializer.serialize_field(0usize, Some("offset"), &self.offset)?;
        serializer.serialize_field(1usize, Some("color"), &self.color)?;
        serializer.serialize_field(2usize, Some("opacity"), &self.opacity)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Group {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(94usize, "Group")?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Path {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(95usize, "Path")?;
        serializer.serialize_field(0usize, Some("events"), &self.events)?;
        serializer.serialize_field(1usize, Some("length"), &self.length)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Pattern {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(96usize, "Pattern")?;
        serializer.serialize_field(0usize, Some("units"), &self.units)?;
        serializer.serialize_field(1usize, Some("content_units"), &self.content_units)?;
        serializer.serialize_field(2usize, Some("transform"), &self.transform)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Use {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(97usize, "Use")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Rect {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(98usize, "Rect")?;
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
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(99usize, "Circle")?;
        serializer.serialize_field(0usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(1usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(2usize, Some("r"), &self.r)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Ellipse {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(100usize, "Ellipse")?;
        serializer.serialize_field(0usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(1usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(2usize, Some("rx"), &self.rx)?;
        serializer.serialize_field(3usize, Some("ry"), &self.ry)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Line {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(101usize, "Line")?;
        serializer.serialize_field(0usize, Some("x1"), &self.x1)?;
        serializer.serialize_field(1usize, Some("y1"), &self.y1)?;
        serializer.serialize_field(2usize, Some("x2"), &self.x2)?;
        serializer.serialize_field(3usize, Some("y2"), &self.y2)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Polyline {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(102usize, "Polyline")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Polygon {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(103usize, "Polygon")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Text {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(105usize, "Text")?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("dx"), &self.dx)?;
        serializer.serialize_field(3usize, Some("dy"), &self.dy)?;
        serializer.serialize_field(4usize, Some("rotate"), &self.rotate)?;
        serializer.serialize_field(5usize, Some("text_length"), &self.text_length)?;
        serializer.serialize_field(6usize, Some("length_adjust"), &self.length_adjust)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextSpan {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(106usize, "TextSpan")?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("dx"), &self.dx)?;
        serializer.serialize_field(3usize, Some("dy"), &self.dy)?;
        serializer.serialize_field(4usize, Some("rotate"), &self.rotate)?;
        serializer.serialize_field(5usize, Some("text_length"), &self.text_length)?;
        serializer.serialize_field(6usize, Some("length_adjust"), &self.length_adjust)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::Characters {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_leaf(107usize, "Characters")?;
        serializer.serialize_field(0usize, None, &self.0)?;
        Ok(())
    }
}
impl ml::rt::serde::Serialize for super::opcode::TextPath {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: ml::rt::serde::Serializer,
    {
        use ml::rt::serde::SerializeNode;
        let mut serializer = serializer.serialize_el(108usize, "TextPath")?;
        serializer.serialize_field(0usize, Some("start_offset"), &self.start_offset)?;
        serializer.serialize_field(1usize, Some("method"), &self.method)?;
        serializer.serialize_field(2usize, Some("spacing"), &self.spacing)?;
        serializer.serialize_field(3usize, Some("href"), &self.href)?;
        Ok(())
    }
}
