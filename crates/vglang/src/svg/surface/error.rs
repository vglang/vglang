use xml_builder::XMLError;

/// Error raised by this crate.
#[derive(Debug, thiserror::Error)]
pub enum SvgRenderingError {
    #[error(transparent)]
    XmlError(#[from] XMLError),
    #[error("Root viewport is missing.")]
    RootViewPort,
    #[error("Pop up the wrong {0} elements.")]
    Pop(usize),

    #[error("Unsatisfied register name")]
    Register(String),

    #[error("variable({0}) cast error. ")]
    VariableCast(String),

    #[error(transparent)]
    FormatError(#[from] std::fmt::Error),
}
