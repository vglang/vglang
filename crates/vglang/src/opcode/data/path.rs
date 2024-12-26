use super::Point;

/// A direction that representation a path drawing commander.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathEvent {
    // (absolute) Start a new sub-path at the given (x,y) coordinate.
    MoveTo(
        Point,
        /// True if this is a relative commander
        bool,
    ),
    /// Close the current subpath by drawing a straight line from the current point to current subpath's initial point.
    Close,
    /// (absolute) Draw a line from the current point to the given (x,y) coordinate which becomes the new current point.
    LineTo(
        Point,
        /// True if this is a relative commander
        bool,
    ),
    /// (absolute) Draw a polyline. At the end of the command, the new current point is set to the final set of coordinates provided.
    Polyline(
        Vec<Point>,
        /// True if this is a relative commander
        bool,
    ),
    /// Draws a cubic Bézier curve from the current point to `to` point,
    /// using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve.
    CubicBezier {
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        /// True if this is a relative commander
        relative: bool,
    },

    /// Draws a cubic Bézier curve from the current point to `to` point,
    /// using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve.
    CubicBezierSmooth {
        ctrl2: Point,
        to: Point,
        /// True if this is a relative commander
        relative: bool,
    },

    /// Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point.
    QuadraticBezier {
        ctrl: Point,
        to: Point,
        /// True if this is a relative commander
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
        /// True if this is a relative commander
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
        /// True if this is a relative commander
        relative: bool,
    },
}
