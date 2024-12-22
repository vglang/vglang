use std::fmt::Display;

/// Functional notation for an IRI: "url(" <IRI> ")".
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FuncIRI(pub String);

impl From<String> for FuncIRI {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for FuncIRI {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl Display for FuncIRI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "url(#{})", self.0)
    }
}
