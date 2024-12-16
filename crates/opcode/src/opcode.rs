//! Defines the optimised low-level instructions.
///
/// Some `Opcode`s have a [`usize`] parameter that points to the row ID of the property table.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    /// Create a `canvas` element.
    Canvas(usize),
    /// Render a `text` element.
    Text(usize),
    /// Render a `text-span` element.
    TextSpan(usize),
    /// characters content of the text/text-span element.
    TextContent(usize),
    /// Apply inheritable property `Fill` to all children instructions.
    Fill(usize),
    /// Apply inheritable property `Stroke` to all children instructions.
    Stroke(usize),
    /// Popup elements, indicates that the popup elements ared fully rendered.
    ///
    /// Unlike other instructions, the [`usize`] parameter of this instruction indicates the number of elements to pop.
    Pop(usize),
}
