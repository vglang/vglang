/// Define a fragment by name.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Id(pub String);

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

/// Use a fragment by name.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Use(pub String);

impl From<String> for Use {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Use {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}
