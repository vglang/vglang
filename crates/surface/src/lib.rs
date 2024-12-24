use std::{collections::HashMap, future::Future};

use vglang_opcode::{data::Data, Opcode};

/// Rendering target must implement this trait.
pub trait Target {
    /// `vglang` program builder.
    ///
    /// See [`Builder`]
    type Builder: Builder;

    /// Create a new vglang program builder.
    fn build(&self) -> Self::Builder;
}

/// A builder comsume input [`Opcode`]s and on success, output target specific [`Program`]
pub trait Builder {
    /// On successfully, returns the `Program`.
    ///
    /// See [`Program`]
    type Program: Program;

    /// Error type returns by [`Build`](Builder::Create) future.
    type Error;

    /// Future type returns by [`create`](Builder::create) function.
    type Create: Future<Output = Result<Self::Program, Self::Error>> + 'static;

    /// Push a new `opcode` to cache buf..
    fn push(&mut self, opcode: Opcode);

    fn pop(&mut self) {
        self.push(Opcode::Pop);
    }

    /// Build vglang program via opcodes.
    fn create(self) -> Self::Create;
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
