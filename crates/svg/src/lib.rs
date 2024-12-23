use std::{fmt::Debug, future::Future, pin::Pin, slice::Iter};

use vglang_opcode::{data::Data, Opcode};
use vglang_surface::*;
use xml_dom::level2::{
    ext::{DocumentDecl, XmlDecl},
    get_implementation, Document, Element, RefNode,
};

/// Error raised by this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    XmlError(#[from] xml_dom::level2::Error),

    #[error("Root viewport is missing.")]
    RootViewPort,
    #[error("Pop up the wrong {0} elements.")]
    Pop(usize),

    #[error("Unsatisfied register name")]
    Register(String),

    #[error(transparent)]
    FormatError(#[from] std::fmt::Error),
}

/// A target output rendering result to svg image.
#[derive(Default, Debug)]
pub struct SvgTarget {}

impl Target for SvgTarget {
    type Builder = SvgBuilder;
    fn build(&self) -> Self::Builder {
        SvgBuilder::default()
    }
}

/// A [`SvgProgram`] builder.
#[derive(Default, Debug)]
pub struct SvgBuilder(Vec<Opcode>);

impl Builder for SvgBuilder {
    type Error = Error;
    type Program = SvgProgram;

    type Create = Pin<Box<dyn Future<Output = Result<Self::Program, Self::Error>> + 'static>>;

    fn push<O>(&mut self, opcode: O)
    where
        Opcode: From<O>,
    {
        self.0.push(opcode.into());
    }

    fn create(self) -> Self::Create {
        Box::pin(async move { Ok(SvgProgram(self.0)) })
    }
}

/// A svg [`Program`] builder.
#[derive(Default, Debug)]
pub struct SvgProgram(pub Vec<Opcode>);

impl Program for SvgProgram {
    type Output = String;

    type Error = Error;

    type Run<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + 'a>>
    where
        Self: 'a;

    fn run<'a>(&'a self, registers: &'a std::collections::HashMap<String, Data>) -> Self::Run<'a> {
        Box::pin(async move {
            let mut document = get_implementation().create_document(
                Some("http://www.w3.org/2000/svg"),
                Some("svg"),
                None,
            )?;

            let xml_decl = XmlDecl::new(xml_dom::level2::ext::XmlVersion::V11, None, Some(true));

            document.set_xml_declaration(xml_decl)?;

            let mut root_element = document.document_element().unwrap();

            root_element.set_attribute("xmlns", "http://www.w3.org/2000/svg")?;
            root_element.set_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink")?;
            root_element.set_attribute("version", "1.1")?;

            SvgCreator {
                document,
                el_stack: vec![root_element],
                opcodes: self.0.iter(),
                registers,
                id: Default::default(),
            }
            .create()
        })
    }
}

#[allow(unused)]
struct SvgCreator<'a> {
    opcodes: Iter<'a, Opcode>,
    registers: &'a std::collections::HashMap<String, Data>,
    document: RefNode,
    el_stack: Vec<RefNode>,
    id: Option<String>,
}

impl<'a> SvgCreator<'a> {
    fn create(&mut self) -> Result<String, Error> {
        todo!()
    }
}
