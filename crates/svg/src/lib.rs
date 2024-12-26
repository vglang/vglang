//! This crate is the official svg [`Surface`] implementation.
//!
//!
//! [`Surface`]: vglang::surface::Surface

use std::{collections::HashMap, fmt::Debug, future::Future, pin::Pin, slice::Iter};
use vglang::{
    opcode::{data::Data, Opcode},
    surface::*,
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

/// A svg rendering target implementation for vglang.
pub struct Svg {}

impl Surface for Svg {
    type Program = SvgRenderer;

    type Error = Error;

    type Build<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Program, Self::Error>> + 'a>>
    where
        Self: 'a;

    fn build(&self, source: Source<'_>) -> Self::Build<'_> {
        match source {
            Source::Opcode(cow) => {
                let render = SvgRenderer::new(cow.to_vec());

                Box::pin(async move { Ok(render) })
            }
            _ => Box::pin(async move { unimplemented!("compile source(Vgl)") }),
        }
    }
}

/// A program render vglang into svg image.
pub struct SvgRenderer(Vec<Opcode>);

impl SvgRenderer {
    fn new(opcodes: Vec<Opcode>) -> Self {
        Self(opcodes)
    }
}

impl Program for SvgRenderer {
    type Output = String;

    type Error = Error;

    type Run<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + 'a>>
    where
        Self: 'a;

    fn run<'a>(
        &'a self,
        registers: &'a std::collections::HashMap<String, vglang::opcode::data::Data>,
    ) -> Self::Run<'a> {
        Box::pin(async move { SvgRendering::new(self.0.iter(), registers).render() })
    }
}

#[allow(unused)]
struct SvgRendering<'a> {
    iter: Iter<'a, Opcode>,
    registers: &'a HashMap<String, Data>,
}

impl<'a> SvgRendering<'a> {
    fn new(iter: Iter<'a, Opcode>, registers: &'a HashMap<String, Data>) -> Self {
        Self { iter, registers }
    }

    fn render(&mut self) -> Result<String, Error> {
        todo!()
    }
}
