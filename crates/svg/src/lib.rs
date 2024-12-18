use std::{future::Future, pin::Pin, slice::Iter};

use vglang_opcode::{operand::Value, Opcode};
pub use vglang_targets::*;

/// Error raised by this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    XmlError(#[from] xml_dom::level2::Error),

    #[error("Root viewport is missing.")]
    RootViewPort,

    #[error("Unsatisfied register name")]
    Register(String),
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

    type Build = Pin<Box<dyn Future<Output = Result<Self::Program, Self::Error>> + 'static>>;

    fn push<O>(&mut self, opcode: O)
    where
        Opcode: From<O>,
    {
        self.0.push(opcode.into());
    }

    fn create(self) -> Self::Build {
        Box::pin(async move { Ok(SvgProgram(self.0)) })
    }
}

/// A svg [`Program`] builder.
#[derive(Default, Debug)]
pub struct SvgProgram(Vec<Opcode>);

impl Program for SvgProgram {
    type Output = String;

    type Error = Error;

    type Run<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + 'a>>
    where
        Self: 'a;

    fn run<'a>(&'a self, registers: &'a std::collections::HashMap<String, Value>) -> Self::Run<'a> {
        Box::pin(async move {
            SvgRun {
                opcodes: self.0.iter(),
                registers,
            }
            .run()
        })
    }
}

#[allow(unused)]
struct SvgRun<'a> {
    opcodes: Iter<'a, Opcode>,
    registers: &'a std::collections::HashMap<String, Value>,
}

impl<'a> SvgRun<'a> {
    fn run(&mut self) -> Result<String, Error> {
        todo!()
    }
}
