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
    #[error("Syntax of literal decimal is error, expect `[0-9]+`")]
    LitDecimal,

    /// The error of parsing literal string.
    #[error("decimal is overflow: {0}")]
    LitDecimalOverflow(String),

    /// The error of parsing literal string.
    #[error("Syntax of literal hexadecimal is error, expect `[0-9,a-f,A-F]+`")]
    LitHex,
    /// The error of parsing literal string.
    #[error("hexadecimal is overflow: {0}")]
    LitHexOverflow(String),
    /// The error of parsing literal string.
    #[error("Syntax of literal binary is error, expect `[0,1]+`")]
    LitBinary,
    /// The error of parsing literal string.
    #[error("binary is overflow: {0}")]
    LitBinaryOverflow(String),

    #[error("Invalid digit for a base {0} literal")]
    InvalidDigit(u8),

    /// The error of parsing literal string.
    #[error("Syntax of number's exponent part is error, be like `e10` or `E-10`")]
    LitExp,
}
