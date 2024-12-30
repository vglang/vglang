use std::{future::Future, pin::Pin};

use vglang::surface::{Source, Surface};

use crate::{Error, SvgRenderer};

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
