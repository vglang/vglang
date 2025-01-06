/// A direction that representation a path drawing commander.
#[hello]
enum PathEvent {
    /// (absolute) Draws a cubic Bézier curve from the current point to `to` point,
    /// using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve.
    CubicBezier{ ctrl1: Point, ctrl2: Point, to: Point },
    /// (absolute) Draw a polyline. At the end of the command, the new current point is set to the final set of coordinates provided.
    Polyline(vec[Point]),
    /// Close the current subpath by drawing a straight line from the current point to current subpath's initial point.
    Close,
    /// (absolute) Start a new sub-path at the given (x,y) coordinate.
    MoveTo(Point), 
    /// (relative) Start a new sub-path at the given (x,y) coordinate.
    MoveToRelative(Point),
    /// (absolute) Draw a line from the current point to the given (x,y) coordinate which becomes the new current point.
    LineTo(Point),
    /// (relative) Draw a line from the current point to the given (x,y) coordinate which becomes the new current point.
    LineToRelative(Point),
    /// (relative) Draw a polyline. At the end of the command, the new current point is set to the final set of coordinates provided.
    PolylineRelative(vec[Point]),
    /// (relative) Draws a cubic Bézier curve from the current point to `to` point,
    /// using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve.
    CubicBezierRelative{ ctrl1: Point, ctrl2: Point, to: Point },
    /// (smooth) Draws a cubic Bézier curve from the current point to `to` point,
    /// using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve.
    CubicBezierSmooth{ ctrl2: Point, to: Point },
    /// (smooth, relative)Draws a cubic Bézier curve from the current point to `to` point,
    /// using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve.
    CubicBezierSmoothRelative{ ctrl2: Point, to: Point },
    /// (absolute) Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point.
    QuadraticBezier{ ctrl: Point, to: Point },
    /// (relative) Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point.
    QuadraticBezierRelative{ ctrl: Point, to: Point },
    /// (smooth) Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point.
    QuadraticBezierSmooth(Point),
    /// (smooth,relative) Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point.
    QuadraticBezierSmoothRelative(Point),
    /// Draws an elliptical arc from the current point to `to` point.
    ///
    /// The center (cx, cy) of the ellipse is calculated automatically to satisfy the constraints
    /// imposed by the other parameters.
    Arc {
        rx: float,ry:float, x_rotation: float, large_arc: bool, sweep: bool, to: Point
    },

    /// (relative) Draws an elliptical arc from the current point to `to` point.
    ///
    /// The center (cx, cy) of the ellipse is calculated automatically to satisfy the constraints
    /// imposed by the other parameters.
    ArcRelative {
        rx: float,ry:float, x_rotation: float, large_arc: bool, sweep: bool, to: Point
    }
}