use mlang::rt::serde::de::{self};
use xml_dom::{level2::RefNode, parser::read_xml};

#[derive(Debug, thiserror::Error)]
pub enum SvgReadError {
    #[error(transparent)]
    XmlError(#[from] xml_dom::parser::Error),
    #[error(transparent)]
    DeError(#[from] de::Error),
}

#[allow(unused)]
pub struct SvgReader {
    /// loaded svg document.
    doc: RefNode,
    current_node: Option<RefNode>,
    current_attr_value: Option<String>,
}

impl SvgReader {
    pub fn parse(xml: impl AsRef<str>) -> Result<Self, SvgReadError> {
        Ok(SvgReader {
            doc: read_xml(xml)?,
            current_attr_value: None,
            current_node: None,
        })
    }
}
