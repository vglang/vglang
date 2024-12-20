use crate::Graphic;
use num::ToPrimitive;
use vglang_opcode::{
    operand::{Path, PathEvent, Point},
    MapCollect,
};
use vglang_targets::Builder;

impl<G> Graphic<G> for Path
where
    G: Builder,
{
    fn draw(self, g: &mut G) {
        g.push(self);
    }
}

/// Create [`PathEvent::MoveTo`].
pub fn move_to<X, Y>(x: X, y: Y) -> PathEvent
where
    X: ToPrimitive,
    Y: ToPrimitive,
{
    PathEvent::MoveTo(Point {
        x: x.to_f32().unwrap(),
        y: y.to_f32().unwrap(),
    })
}

/// Create [`PathEvent::LineTo`].
pub fn line_to<X, Y>(x: X, y: Y) -> PathEvent
where
    X: ToPrimitive,
    Y: ToPrimitive,
{
    PathEvent::LineTo(Point {
        x: x.to_f32().unwrap(),
        y: y.to_f32().unwrap(),
    })
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
    RX: ToPrimitive,
    RY: ToPrimitive,
    R: ToPrimitive,
    Point: From<T>,
{
    PathEvent::Arc {
        rx: rx.to_f32().unwrap(),
        ry: ry.to_f32().unwrap(),
        x_rotation: x_rotation.to_f32().unwrap(),
        large_arc,
        sweep,
        to: to.into(),
    }
}

/// Create [`PathEvent::Close`].
pub fn close() -> PathEvent {
    PathEvent::Close
}

/// Create [`PathEvent::Polyline`].
pub fn polyline<P>(data: P) -> PathEvent
where
    P: MapCollect<Point>,
{
    PathEvent::Polyline(data.map_collect())
}
