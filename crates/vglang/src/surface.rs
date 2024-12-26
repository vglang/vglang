use std::{borrow::Cow, collections::HashMap, future::Future};

use crate::opcode::{data::Data, Opcode};

/// Source of a vglang program.
#[derive(Debug, Clone)]
pub enum Source<'a> {
    /// IR module represented as a slice of opcodes.
    Opcode(Cow<'a, [Opcode]>),
    /// VGL module represented as a string slice.
    Vgl(Cow<'a, str>),
}

/// Rendering target must implement this trait.
pub trait Surface {
    /// Output type of [`build`](Surface::build) function.
    type Program: Program;
    /// Create a new vglang program builder.
    fn build(&self, source: Source<'_>) -> Self::Program;
}

/// A optimized vglang program created by [`build`](Target::build) function.
pub trait Program {
    /// On success, returns by [`Run`](Program::Run) future.
    type Output;
    /// The error type may returns by [`Run`](Program::Run) future.
    type Error;
    /// Future type returns by [`run`](Program::run) function.
    type Run<'a>: Future<Output = Result<Self::Output, Self::Error>>
    where
        Self: 'a;

    /// run this program with provided animation registers instance.
    fn run<'a>(&'a self, registers: &'a HashMap<String, Data>) -> Self::Run<'a>;
}
