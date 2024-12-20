use std::fmt::Display;

use super::{Length, Point, Variable};

/// A direction that representation a path drawing commander.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathEvent {
    // (absolute) Start a new sub-path at the given (x,y) coordinate.
    MoveTo(Point, bool),
    /// Close the current subpath by drawing a straight line from the current point to current subpath's initial point.
    Close,
    /// (absolute) Draw a line from the current point to the given (x,y) coordinate which becomes the new current point.
    LineTo(Point, bool),
    /// (absolute) Draw a polyline. At the end of the command, the new current point is set to the final set of coordinates provided.
    Polyline(Vec<Point>, bool),
    /// Draws a cubic Bézier curve from the current point to `to` point,
    /// using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve.
    CubicBezier {
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        relative: bool,
    },

    /// Draws a cubic Bézier curve from the current point to `to` point,
    /// using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve.
    CubicBezierSmooth {
        ctrl2: Point,
        to: Point,
        relative: bool,
    },

    /// Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point.
    QuadraticBezier {
        ctrl: Point,
        to: Point,
        relative: bool,
    },

    /// Draws a quadratic Bézier curve from the current point to (x,y). The control point is assumed to be the reflection
    /// of the control point on the previous command relative to the current point. (If there is no previous command or
    /// if the previous command was not a Q, q, T or t, assume the control point is coincident with the current point.)
    /// T (uppercase) indicates that absolute coordinates will follow; t (lowercase) indicates that relative coordinates
    /// will follow. At the end of the command, the new current point becomes the final (x,y) coordinate pair used in the
    /// polybézier.
    QuadraticBezierSmooth {
        to: Point,
        relative: bool,
    },

    /// Draws an elliptical arc from the current point to `to` point.
    ///
    /// The center (cx, cy) of the ellipse is calculated automatically to satisfy the constraints
    /// imposed by the other parameters.
    Arc {
        /// The size and orientation of the ellipse are defined by two radii (rx, ry) and an x_rotation,
        /// which indicates how the ellipse as a whole is rotated relative to the current coordinate system.
        rx: f32,
        /// See [`rx`](PathEvent::Arc::rx)
        ry: f32,
        /// See [`rx`](PathEvent::Arc::rx)
        x_rotation: f32,
        /// `large_arc` and [`sweep`](PathEvent::Arc::sweep) contribute to the automatic calculations
        /// and help determine how the arc is drawn.
        large_arc: bool,
        /// See [`large_arc`](PathEvent::Arc::large_arc)
        sweep: bool,
        /// Draws an elliptical arc from the current point to `to` point.
        to: Point,
        relative: bool,
    },
}

impl Display for PathEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathEvent::MoveTo(point, relative) => {
                if *relative {
                    write!(f, "m{},{}", point.x, point.y)
                } else {
                    write!(f, "M{},{}", point.x, point.y)
                }
            }
            PathEvent::Close => {
                write!(f, "Z")
            }
            PathEvent::LineTo(point, relative) => {
                if *relative {
                    write!(f, "l{},{}", point.x, point.y)
                } else {
                    write!(f, "L{},{}", point.x, point.y)
                }
            }
            PathEvent::Polyline(vec, relative) => {
                if *relative {
                    write!(
                        f,
                        "l{}",
                        vec.iter()
                            .map(|v| format!("{},{}", v.x, v.y))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                } else {
                    write!(
                        f,
                        "L{}",
                        vec.iter()
                            .map(|v| format!("{},{}", v.x, v.y))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                }
            }
            PathEvent::CubicBezier {
                ctrl1,
                ctrl2,
                to,
                relative,
            } => {
                if *relative {
                    write!(
                        f,
                        "c{},{} {},{} {},{}",
                        ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y
                    )
                } else {
                    write!(
                        f,
                        "C{},{} {},{} {},{}",
                        ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y
                    )
                }
            }
            PathEvent::QuadraticBezier { ctrl, to, relative } => {
                if *relative {
                    write!(f, "q{},{} {},{}", ctrl.x, ctrl.y, to.x, to.y)
                } else {
                    write!(f, "Q{},{} {},{}", ctrl.x, ctrl.y, to.x, to.y)
                }
            }
            PathEvent::Arc {
                rx,
                ry,
                x_rotation,
                large_arc,
                sweep,
                to,
                relative,
            } => {
                if *relative {
                    write!(
                        f,
                        "a{},{} {} {},{} {},{}",
                        *rx,
                        *ry,
                        *x_rotation,
                        if *large_arc { 1 } else { 0 },
                        if *sweep { 1 } else { 0 },
                        to.x,
                        to.y
                    )
                } else {
                    write!(
                        f,
                        "A{},{} {} {},{} {},{}",
                        *rx,
                        *ry,
                        *x_rotation,
                        if *large_arc { 1 } else { 0 },
                        if *sweep { 1 } else { 0 },
                        to.x,
                        to.y
                    )
                }
            }
            PathEvent::CubicBezierSmooth {
                ctrl2,
                to,
                relative,
            } => {
                if *relative {
                    write!(f, "s{},{} {},{}", ctrl2.x, ctrl2.y, to.x, to.y)
                } else {
                    write!(f, "S{},{} {},{}", ctrl2.x, ctrl2.y, to.x, to.y)
                }
            }
            PathEvent::QuadraticBezierSmooth { to, relative } => {
                if *relative {
                    write!(f, "t{},{}", to.x, to.y)
                } else {
                    write!(f, "T{},{}", to.x, to.y)
                }
            }
        }
    }
}

// Paths represent the outline of a shape which can be filled, stroked, used as a clipping path,
/// or any combination of the three.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "sexpr", derive(vglang_derive::Sexpr))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Path {
    /// The definition of the outline of a shape.
    pub data: Vec<PathEvent>,

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

#[cfg(feature = "sexpr")]
mod sexpr {
    use crate::{tuple_map_collect, MapCollect};

    use super::*;

    impl<T> From<T> for Path
    where
        T: MapCollect<PathEvent>,
    {
        fn from(value: T) -> Self {
            Self {
                data: value.map_collect(),
                ..Default::default()
            }
        }
    }

    tuple_map_collect!(
        PathEvent, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17,
        A18, A19
    );
}
