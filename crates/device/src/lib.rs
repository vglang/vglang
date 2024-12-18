use std::{collections::HashMap, future::Future};

use vglang_opcode::{Opcode, Value};

/// Rendering target must implement this trait.
pub trait Device {
    /// `vglang` program builder.
    ///
    /// See [`Builder`]
    type Builder: Builder;

    /// Create a new vglang program builder.
    fn build(&self) -> Self::Builder;
}

/// A vglang program builder must implement this trait.
pub trait Builder {
    /// On successfully, returns the `Program`.
    ///
    /// See [`Program`]
    type Program: Program;

    /// Error type returns by [`Build`](Builder::Build) future.
    type Error;

    /// Future type returns by [`build`](Builder::build) function.
    type Build<'a>: Future<Output = Result<Self::Program, Self::Error>>
    where
        Self: 'a;
    /// Push a new `opcode` to cache buf..
    fn push<O>(&mut self, opcode: O)
    where
        Opcode: From<O>;

    /// Build vglang program via opcodes.
    fn build(&mut self) -> Self::Build<'_>;
}

/// A optimized vglang program created by [`build`](Device::build) function.
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
    fn run(&self, registers: &HashMap<String, Value>) -> Self::Run<'_>;
}
