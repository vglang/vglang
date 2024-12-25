use crate::opcode::Opcode;

/// build context used by [`Graphics`](super::Graphics) trait.
#[derive(Debug, Default)]
pub struct BuildContext(Vec<Opcode>);

impl From<BuildContext> for Vec<Opcode> {
    fn from(value: BuildContext) -> Self {
        value.0
    }
}

impl AsRef<[Opcode]> for BuildContext {
    fn as_ref(&self) -> &[Opcode] {
        &self.0
    }
}

impl BuildContext {
    /// Push a new `opcode`
    pub fn push<O>(&mut self, opcode: O)
    where
        Opcode: From<O>,
    {
        self.0.push(opcode.into());
    }

    /// Push a `Pop` opcode.
    pub fn pop(&mut self) {
        self.0.push(Opcode::Pop);
    }
}
