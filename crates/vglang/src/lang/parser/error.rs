/// `vglang` script parsing error.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParseError {
    #[error("Parse graphic fn error.")]
    GraphicFn,
}
