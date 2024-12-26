use crate::opcode::{
    data::{Length, PathEvent},
    variable::Variable,
};

#[cfg(feature = "sexpr")]
use super::*;

// Paths represent the outline of a shape which can be filled, stroked, used as a clipping path,
/// or any combination of the three.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::shape_element(boxed, If, For, Foreach, Group)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Path {
    /// The definition of the outline of a shape.
    pub data: Variable<Vec<PathEvent>>,

    /// The author's computation of the total length of the path, in user units.
    /// This value is used to calibrate the user agent's own distance-along-a-path
    /// calculations with that of the author. The user agent will scale all
    /// distance-along-a-path computations by the ratio of ‘pathLength’ to the user
    /// agent's own computed value for total path length. ‘pathLength’ potentially
    /// affects calculations for text on a path, motion animation and various stroke
    /// operations.
    ///
    /// A negative value is an error (see Error processing).
    pub length: Variable<Length>,
}
