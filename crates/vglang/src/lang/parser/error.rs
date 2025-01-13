/// `vglang` script parsing error.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParseError {
    /// The error of parsing fn block.
    #[error("Syntax of fn is error.")]
    Fn,
    /// The error of parsing hex color.
    #[error("Syntax of hex color is error, expect `#rgb` or `#rrggbb`")]
    HexColor,

    /// The error of parsing recognized color.
    #[error("Syntax of recognized color is error, expect `color.xxx`.")]
    RecognizedColor,

    /// The error of parsing recognized color.
    #[error("Syntax of ident is error, expect `alphabetic [alphanumeric]*`.")]
    Ident,

    /// The error of parsing named register.
    #[error("Syntax of ident is error, expect $`alphabetic [alphanumeric]*`.")]
    NamedRegister,

    /// The error of parsing literal string.
    #[error("Syntax of literal string is error, must end with `{0}`")]
    LitStr(String),
}
