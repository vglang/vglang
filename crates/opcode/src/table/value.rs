/// This type defines the table cell value
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    /// indicates this value is reference to a register table value.
    Register(usize),
    /// indicates this value is reference to a constant table value.
    Constant(usize),
}
