use crate::{
    opcode::{Length, PathEvent, Point},
    sexpr::MapCollect,
};

/// A trait convert self into [`Length`]
pub trait Slength {
    fn em(self) -> Length;

    fn ex(self) -> Length;

    fn px(self) -> Length;

    fn r#in(self) -> Length;

    fn cm(self) -> Length;

    fn mm(self) -> Length;

    fn pt(self) -> Length;

    fn pc(self) -> Length;

    fn percent(self) -> Length;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Number(
    /// The wrapped [`f32`] value.
    pub f32,
);

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self(value as f32)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self(value as f32)
    }
}

impl<T> Slength for T
where
    Number: From<T>,
{
    fn em(self) -> Length {
        Length::Em(Number::from(self).0)
    }

    fn ex(self) -> Length {
        Length::Ex(Number::from(self).0)
    }

    fn px(self) -> Length {
        Length::Px(Number::from(self).0)
    }

    fn r#in(self) -> Length {
        Length::Inch(Number::from(self).0)
    }

    fn cm(self) -> Length {
        Length::Cm(Number::from(self).0)
    }

    fn mm(self) -> Length {
        Length::Mm(Number::from(self).0)
    }

    fn pt(self) -> Length {
        Length::Pt(Number::from(self).0)
    }

    fn pc(self) -> Length {
        Length::Pc(Number::from(self).0)
    }

    fn percent(self) -> Length {
        Length::Percent(Number::from(self).0)
    }
}

/// Create [`PathEvent::MoveTo`].
pub fn move_to<X, Y>(x: X, y: Y) -> PathEvent
where
    Number: From<X> + From<Y>,
{
    PathEvent::MoveTo(Point(Number::from(x).0, Number::from(y).0))
}

/// Create [`PathEvent::MoveTo`].
pub fn move_to_relative<X, Y>(x: X, y: Y) -> PathEvent
where
    Number: From<X> + From<Y>,
{
    PathEvent::MoveToRelative(Point(Number::from(x).0, Number::from(y).0))
}

/// Create [`PathEvent::LineTo`].
pub fn line_to<X, Y>(x: X, y: Y) -> PathEvent
where
    Number: From<X> + From<Y>,
{
    PathEvent::LineTo(Point(Number::from(x).0, Number::from(y).0))
}

/// Create [`PathEvent::LineTo`].
pub fn line_to_relative<X, Y>(x: X, y: Y) -> PathEvent
where
    Number: From<X> + From<Y>,
{
    PathEvent::LineToRelative(Point(Number::from(x).0, Number::from(y).0))
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
    }
}

/// Create [`PathEvent::CubicBezier`].
pub fn cubic_bezier_relative<C1, C2, T>(ctrl1: C1, ctrl2: C2, to: T) -> PathEvent
where
    Point: From<C1> + From<C2> + From<T>,
{
    PathEvent::CubicBezierRelative {
        ctrl1: ctrl1.into(),
        ctrl2: ctrl2.into(),
        to: to.into(),
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
    }
}

/// Create [`PathEvent::CubicBezierSmooth`].
pub fn cubic_bezier_smooth_relative<C2, T>(ctrl2: C2, to: T) -> PathEvent
where
    Point: From<C2> + From<T>,
{
    PathEvent::CubicBezierSmoothRelative {
        ctrl2: ctrl2.into(),
        to: to.into(),
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
    }
}

/// Create [`PathEvent::QuadraticBezier`].
pub fn quadratic_bezier_relative<C, T>(ctrl: C, to: T) -> PathEvent
where
    Point: From<C> + From<T>,
{
    PathEvent::QuadraticBezierRelative {
        ctrl: ctrl.into(),
        to: to.into(),
    }
}

/// Create [`PathEvent::QuadraticBezierSmooth`].
pub fn quadratic_bezier_smooth<T>(to: T) -> PathEvent
where
    Point: From<T>,
{
    PathEvent::QuadraticBezierSmooth(to.into())
}

/// Create [`PathEvent::QuadraticBezierSmooth`].
pub fn quadratic_bezier_smooth_relative<T>(to: T) -> PathEvent
where
    Point: From<T>,
{
    PathEvent::QuadraticBezierSmoothRelative(to.into())
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
    PathEvent::ArcRelative {
        rx: Number::from(rx).0,
        ry: Number::from(ry).0,
        x_rotation: Number::from(x_rotation).0,
        large_arc,
        sweep,
        to: to.into(),
    }
}

/// Create [`PathEvent::Polyline`].
pub fn polyline<P>(data: P) -> PathEvent
where
    P: MapCollect<Point>,
{
    PathEvent::Polyline(data.map_collect())
}

/// Create [`PathEvent::Polyline`].
pub fn polyline_relative<P>(data: P) -> PathEvent
where
    P: MapCollect<Point>,
{
    PathEvent::PolylineRelative(data.map_collect())
}

/// Create [`PathEvent::Close`].
pub fn close() -> PathEvent {
    PathEvent::Close
}
