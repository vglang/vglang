pub use cotati_device::{Device, VGLProgram};
use cotati_ir::IR;
use futures::future::BoxFuture;

/// Error raised by this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {}

/// A svg rendering target implementation.
#[derive(Default)]
pub struct SvgDevice {}

impl Device for SvgDevice {
    type Program = SvgGenerator;

    type Error = Error;

    type Compile<'a>
        = BoxFuture<'a, Result<SvgGenerator, Error>>
    where
        Self: 'a;

    fn compile(&self, codes: Vec<cotati_ir::IR>) -> Self::Compile<'_> {
        Box::pin(async move { Ok(SvgGenerator(codes)) })
    }
}

/// `VGLProgram` implementation for svg generator.
#[allow(unused)]
pub struct SvgGenerator(Vec<IR>);

impl VGLProgram for SvgGenerator {
    type Output = String;

    type Error = Error;

    type Execute<'a>
        = BoxFuture<'a, Result<String, Error>>
    where
        Self: 'a;

    fn execute<'a>(
        &'a self,
        animatable: &'a std::collections::HashMap<String, cotati_ir::AnimatableValue>,
    ) -> Self::Execute<'a> {
        Box::pin(async move { self.do_execute(animatable) })
    }
}

impl SvgGenerator {
    fn do_execute<'a>(
        &'a self,
        _animatable: &'a std::collections::HashMap<String, cotati_ir::AnimatableValue>,
    ) -> Result<String, Error> {
        Ok("".to_owned())
    }
}
