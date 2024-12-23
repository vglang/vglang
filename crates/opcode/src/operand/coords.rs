use std::fmt::Display;

/// Defines the coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’.
///
/// If attribute ‘gradientUnits’ is not specified, then the effect is as if a value of 'objectBoundingBox' were specified.
///
/// Animatable: yes.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Coords {
    /// If gradientUnits="userSpaceOnUse", ‘x1’, ‘y1’, ‘x2’ and ‘y2’ represent values in the coordinate system
    /// that results from taking the current user coordinate system in place at the time when the gradient element
    /// is referenced (i.e., the user coordinate system for the element referencing the gradient element via a ‘fill’
    /// or ‘stroke’ property) and then applying the transform specified by attribute ‘gradientTransform’.
    UserSpaceOnUse,
    /// If gradientUnits="objectBoundingBox", the user coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’ is
    /// established using the bounding box of the element to which the gradient is applied (see Object bounding box units)
    /// and then applying the transform specified by attribute ‘gradientTransform’.
    ///
    /// When gradientUnits="objectBoundingBox" and ‘gradientTransform’ is the identity matrix, the normal of the linear
    /// gradient is perpendicular to the gradient vector in object bounding box space (i.e., the abstract coordinate
    /// system where (0,0) is at the top/left of the object bounding box and (1,1) is at the bottom/right of the object bounding box).
    /// When the object's bounding box is not square, the gradient normal which is initially perpendicular to the gradient vector
    /// within object bounding box space may render non-perpendicular relative to the gradient vector in user space. If the gradient
    /// vector is parallel to one of the axes of the bounding box, the gradient normal will remain perpendicular. This transformation
    /// is due to application of the non-uniform scaling transformation from bounding box space to user space.
    ObjectBoundingBox,
}

impl Default for Coords {
    fn default() -> Self {
        Self::ObjectBoundingBox
    }
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Coords::UserSpaceOnUse => write!(f, "userSpaceOnUse"),
            Coords::ObjectBoundingBox => write!(f, "objectBoundingBox"),
        }
    }
}
