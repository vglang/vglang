/// Error type returns by parser.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("The syntax of ident is error, be like: `Hello`,`_world`")]
    Ident,
    #[error("Expect hex number body, be like: `0x10`")]
    HexBody,
    #[error("Expect binary number body, be like: `0b110110`")]
    BinaryBody,
    #[error("Expect decimal number, be like: `123`")]
    Decimal,
    #[error("The syntax of number is error.")]
    Number,
    #[error("The syntax of hex colro value is error.")]
    HexColor,

    #[error("The syntax of color is error or unknown color value.")]
    RecognizedColor,

    #[error("The syntax of enum is error, {0}")]
    LitEnum(EnumKind),

    #[error("The syntax of literal str is error, {0}")]
    LitStr(StrKind),

    #[error("The syntax of call is error, {0}")]
    CallBody(CallBodyKind),
}

/// literal string error.
#[derive(Debug, thiserror::Error)]
pub enum CallBodyKind {
    #[error("invalid param expr")]
    Param,
    #[error("expect param list start `(`")]
    ParamListStart,

    #[error("expect param list end `)`")]
    ParamListEnd,
}

/// literal string error.
#[derive(Debug, thiserror::Error)]
pub enum StrKind {
    #[error("expect end `'`")]
    Quote,
    #[error("expect end `\"`")]
    DoubleQuote,
}

/// Enum parse error
#[derive(Debug, thiserror::Error)]
pub enum EnumKind {
    #[error("invalid target name.")]
    Target,
    #[error("Miss punct `.`")]
    Punct,
    #[error("invalid field name.")]
    Field,
}
