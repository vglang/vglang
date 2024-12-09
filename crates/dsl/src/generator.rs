use cotati_device::Device;
use cotati_ir::{Layer, Rect, Text, IR};

/// This trait defines the compile target generator of `embed VGL language`.
pub trait Generator {
    /// Push a new ir code.
    fn push(&mut self, ir: IR);

    /// pop top `n` graphic element from the drawing stack.
    fn pop(&mut self, n: usize) {
        self.push(IR::Pop(n));
    }

    /// Push a new text element.
    fn push_text(&mut self, value: Text) {
        self.push(IR::Text(Box::new(value)));
    }

    /// Push a new layer element.
    fn push_layer(&mut self, value: Layer) {
        self.push(IR::Layer(Box::new(value)));
    }

    /// Push a new rect element.
    fn push_rect(&mut self, value: Rect) {
        self.push(IR::Rect(Box::new(value)));
    }
}

/// A generator that output compile result as in-memory ir codes stream.
#[derive(Default)]
pub struct IRGenerator {
    codes: Vec<IR>,
}

impl Generator for IRGenerator {
    fn push(&mut self, ir: IR) {
        self.codes.push(ir);
    }
}

impl IRGenerator {
    /// Consume self and use provides [`Device`] to compile output as a `VGL` program.
    pub async fn compile<D>(self, device: &mut D) -> Result<D::Program, D::Error>
    where
        D: Device,
    {
        device.compile(self.codes).await
    }
}
