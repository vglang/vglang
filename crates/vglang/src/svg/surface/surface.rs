use std::{future::Future, pin::Pin};

use crate::{
    opcode::Data,
    surface::{Program, Source, Surface},
};

use super::{SvgRenderer, SvgRendering, SvgRenderingError};

impl Program for SvgRenderer {
    type Output = Vec<u8>;

    type Error = SvgRenderingError;

    type Run<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + 'a>>
    where
        Self: 'a;

    fn run<'a>(&'a self, registers: &'a std::collections::HashMap<String, Data>) -> Self::Run<'a> {
        Box::pin(async move { SvgRendering::new(&self.0, registers).render() })
    }
}

/// A svg rendering target implementation for vglang.
pub struct Svg;

impl Surface for Svg {
    type Program = SvgRenderer;

    type Error = SvgRenderingError;

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
