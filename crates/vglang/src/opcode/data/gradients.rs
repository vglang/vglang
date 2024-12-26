/// Indicates what happens if the gradient starts or ends inside the bounds of the target rectangle.
/// Possible values are: 'pad', which says to use the terminal colors of the gradient to fill the remainder of the target region,
/// 'reflect', which says to reflect the gradient pattern start-to-end, end-to-start, start-to-end, etc. continuously until the
/// target rectangle is filled, and repeat, which says to repeat the gradient pattern start-to-end, start-to-end, start-to-end,
/// etc. continuously until the target region is filled.
/// If the attribute is not specified, the effect is as if a value of 'pad' were specified.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpreadMethod {
    Pad,
    Reflect,
    Repeat,
}

impl Default for SpreadMethod {
    fn default() -> Self {
        Self::Pad
    }
}
