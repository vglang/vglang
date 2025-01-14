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
    /// The error of parsing literal string.
    #[error("Syntax of number's exponent part is error, be like `e10` or `E-10`")]
    LitExp,
    /// The error of parsing literal string.
    #[error("Syntax of literal number is error")]
    LitNum,
    /// The error of parsing literal string.
    #[error("literal number is overflow")]
    LitNumOverflow,
    /// The error of parsing literal string.
    #[error("Syntax of literal int is error")]
    LitInt,
    /// The error of parsing literal string.
    #[error("literal int is overflow, the range is -2147483648 to 2147483647.")]
    LitIntOverflow,
    /// The error of parsing literal string.
    #[error("Syntax of coords is error, expect `coords.object` or `coords.userspace`")]
    LitCoords,
    /// The error of parsing literal string.
    #[error("Syntax of coords is error, expect `coords.object` or `coords.userspace`")]
    LitLength,
}
