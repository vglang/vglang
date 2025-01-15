/// Error type returns by parser.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Expect hex number body, be like: `0x10`")]
    HexBody,
    #[error("Expect binary number body, be like: `0b110110`")]
    BinaryBody,
    #[error("Expect decimal number, be like: `123`")]
    Decimal,
    #[error("The syntax of number is error.")]
    Number,
}
