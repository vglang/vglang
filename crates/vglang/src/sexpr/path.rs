use crate::opcode::{
    data::{Number, PathEvent, Point},
    el::Path,
    variable::Variable,
};

use super::MapCollect;

impl<T> From<T> for Path
where
    T: MapCollect<PathEvent>,
{
    fn from(value: T) -> Self {
        Self {
            data: Variable::Constant(value.map_collect()),
            ..Default::default()
        }
    }
}

/// Create [`PathEvent::MoveTo`].
pub fn move_to<X, Y>(x: X, y: Y) -> PathEvent
where
    Number: From<X> + From<Y>,
{
    PathEvent::MoveTo(
        Point {
            x: Number::from(x).0,
            y: Number::from(y).0,
        },
        false,
    )
}

/// Create [`PathEvent::MoveTo`].
pub fn move_to_relative<X, Y>(x: X, y: Y) -> PathEvent
where
    Number: From<X> + From<Y>,
{
    PathEvent::MoveTo(
        Point {
            x: Number::from(x).0,
            y: Number::from(y).0,
        },
        true,
    )
}

/// Create [`PathEvent::LineTo`].
pub fn line_to<X, Y>(x: X, y: Y) -> PathEvent
where
    Number: From<X> + From<Y>,
{
    PathEvent::LineTo(
        Point {
            x: Number::from(x).0,
            y: Number::from(y).0,
        },
        false,
    )
}

/// Create [`PathEvent::LineTo`].
pub fn line_to_relative<X, Y>(x: X, y: Y) -> PathEvent
where
    Number: From<X> + From<Y>,
{
    PathEvent::LineTo(
        Point {
            x: Number::from(x).0,
            y: Number::from(y).0,
        },
        true,
    )
}

/// Create [`PathEvent::CubicBezier`].
pub fn cubic_bezier<C1, C2, T>(ctrl1: C1, ctrl2: C2, to: T) -> PathEvent
where
    Point: From<C1> + From<C2> + From<T>,
{
    PathEvent::CubicBezier {
        ctrl1: ctrl1.into(),
        ctrl2: ctrl2.into(),
        to: to.into(),
        relative: false,
    }
}

/// Create [`PathEvent::CubicBezier`].
pub fn cubic_bezier_relative<C1, C2, T>(ctrl1: C1, ctrl2: C2, to: T) -> PathEvent
where
    Point: From<C1> + From<C2> + From<T>,
{
    PathEvent::CubicBezier {
        ctrl1: ctrl1.into(),
        ctrl2: ctrl2.into(),
        to: to.into(),
        relative: true,
    }
}

/// Create [`PathEvent::CubicBezierSmooth`].
pub fn cubic_bezier_smooth<C2, T>(ctrl2: C2, to: T) -> PathEvent
where
    Point: From<C2> + From<T>,
{
    PathEvent::CubicBezierSmooth {
        ctrl2: ctrl2.into(),
        to: to.into(),
        relative: false,
    }
}

/// Create [`PathEvent::CubicBezierSmooth`].
pub fn cubic_bezier_smooth_relative<C2, T>(ctrl2: C2, to: T) -> PathEvent
where
    Point: From<C2> + From<T>,
{
    PathEvent::CubicBezierSmooth {
        ctrl2: ctrl2.into(),
        to: to.into(),
        relative: true,
    }
}

/// Create [`PathEvent::QuadraticBezier`].
pub fn quadratic_bezier<C, T>(ctrl: C, to: T) -> PathEvent
where
    Point: From<C> + From<T>,
{
    PathEvent::QuadraticBezier {
        ctrl: ctrl.into(),
        to: to.into(),
        relative: false,
    }
}

/// Create [`PathEvent::QuadraticBezier`].
pub fn quadratic_bezier_relative<C, T>(ctrl: C, to: T) -> PathEvent
where
    Point: From<C> + From<T>,
{
    PathEvent::QuadraticBezier {
        ctrl: ctrl.into(),
        to: to.into(),
        relative: true,
    }
}

/// Create [`PathEvent::QuadraticBezierSmooth`].
pub fn quadratic_bezier_smooth<T>(to: T) -> PathEvent
where
    Point: From<T>,
{
    PathEvent::QuadraticBezierSmooth {
        to: to.into(),
        relative: false,
    }
}

/// Create [`PathEvent::QuadraticBezierSmooth`].
pub fn quadratic_bezier_smooth_relative<T>(to: T) -> PathEvent
where
    Point: From<T>,
{
    PathEvent::QuadraticBezierSmooth {
        to: to.into(),
        relative: true,
    }
}

/// Create [`PathEvent::Arc`].
pub fn arc<RX, RY, R, T>(
    rx: RX,
    ry: RY,
    x_rotation: R,
    large_arc: bool,
    sweep: bool,
    to: T,
) -> PathEvent
where
    Number: From<RX> + From<RY> + From<R>,
    Point: From<T>,
{
    PathEvent::Arc {
        rx: Number::from(rx).0,
        ry: Number::from(ry).0,
        x_rotation: Number::from(x_rotation).0,
        large_arc,
        sweep,
        to: to.into(),
        relative: false,
    }
}

/// Create [`PathEvent::Arc`].
pub fn arc_relative<RX, RY, R, T>(
    rx: RX,
    ry: RY,
    x_rotation: R,
    large_arc: bool,
    sweep: bool,
    to: T,
) -> PathEvent
where
    Number: From<RX> + From<RY> + From<R>,
    Point: From<T>,
{
    PathEvent::Arc {
        rx: Number::from(rx).0,
        ry: Number::from(ry).0,
        x_rotation: Number::from(x_rotation).0,
        large_arc,
        sweep,
        to: to.into(),
        relative: true,
    }
}

/// Create [`PathEvent::Polyline`].
pub fn polyline<P>(data: P) -> PathEvent
where
    P: MapCollect<Point>,
{
    PathEvent::Polyline(data.map_collect(), false)
}

/// Create [`PathEvent::Polyline`].
pub fn polyline_relative<P>(data: P) -> PathEvent
where
    P: MapCollect<Point>,
{
    PathEvent::Polyline(data.map_collect(), true)
}

/// Create [`PathEvent::Close`].
pub fn close() -> PathEvent {
    PathEvent::Close
}
