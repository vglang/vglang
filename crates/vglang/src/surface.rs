//! This module defines an abstraction for vglang rendering targets.

use std::{borrow::Cow, collections::HashMap, future::Future};

use crate::opcode::{Data, Opcode};

/// Source of a vglang program.
#[derive(Debug, Clone)]
pub enum Source<'a> {
    /// IR module represented as a slice of opcodes.
    Opcode(Cow<'a, [Opcode]>),
    /// VGL module represented as a string slice.
    Vgl(Cow<'a, str>),
}

impl<'a> From<&'a [Opcode]> for Source<'a> {
    fn from(value: &'a [Opcode]) -> Self {
        Self::Opcode(value.into())
    }
}

impl From<Vec<Opcode>> for Source<'static> {
    fn from(value: Vec<Opcode>) -> Self {
        Self::Opcode(value.into())
    }
}

impl<'a> From<&'a str> for Source<'a> {
    fn from(value: &'a str) -> Self {
        Self::Vgl(value.into())
    }
}

impl From<String> for Source<'static> {
    fn from(value: String) -> Self {
        Self::Vgl(value.into())
    }
}

/// Rendering target must implement this trait.
pub trait Surface {
    /// Output type of [`build`](Surface::build) function.
    type Program: Program;
    /// The error type may returns by [`Run`](Program::Run) future.
    type Error;
    /// Future type returns by [`run`](Surface::build) function.
    type Build<'a>: Future<Output = Result<Self::Program, Self::Error>>
    where
        Self: 'a;
    /// Create a new vglang program builder.
    fn build(&self, source: Source<'_>) -> Self::Build<'_>;
}

/// A optimized vglang program created by [`build`](Surface::build) function.
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
