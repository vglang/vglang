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
}
