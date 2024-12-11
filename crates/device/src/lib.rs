//! A device is an abstract of VGL language rendering target.

use std::{collections::HashMap, future::Future};

use vglang_ir::{AnimatableValue, IR};

/// All `VGL language` rendering target must implement this trait.
pub trait Device {
    /// A memory representation of one `VGL` program that is directly compiled from ir codes.
    ///
    /// See [`VGLProgram`]
    type Program: VGLProgram<Error = Self::Error>;

    type Error;

    /// A future returns by [`compile`](Device::compile) function.
    type Compile<'a>: Future<Output = Result<Self::Program, Self::Error>>
    where
        Self: 'a;

    /// Compile one `VGL` program from ir codes stream.
    fn compile(&self, codes: Vec<IR>) -> Self::Compile<'_>;
}

/// A in-memory representation of one `VGL` program that is generally created by
/// [`compile`](Device::compile) function.
pub trait VGLProgram {
    /// The execution output of one `VGL` program.
    type Output;

    type Error;

    /// A future returns by [`execute`](VGLProgram::execute) function.
    type Execute<'a>: Future<Output = Result<Self::Output, Self::Error>>
    where
        Self: 'a;

    /// Exeucte this `VGL` program.
    ///
    /// The `animatable` parameter represents the named animatable registers.
    fn execute<'a>(&'a self, animatable: &'a HashMap<String, AnimatableValue>)
        -> Self::Execute<'a>;
}
