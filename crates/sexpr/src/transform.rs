use num::ToPrimitive;
use vglang_opcode::operand::Transform;
use vglang_targets::Builder;

use crate::{Appliable, Graphic};

impl Appliable for Transform {
    fn apply<G, C>(self, graphic: C) -> impl Graphic<G>
    where
        C: Graphic<G>,
        G: Builder,
    {
        move |g: &mut G| {
            g.push(self);
            graphic.draw(g);
            g.pop();
        }
    }
}

/// Help trait to construct [`Transform::Translate`]
pub trait Stranslate {
    fn translate(self) -> Transform;
}

impl<X, Y> Stranslate for (X, Y)
where
    X: ToPrimitive,
    Y: ToPrimitive,
{
    fn translate(self) -> Transform {
        Transform::Translate {
            tx: self.0.to_f32().unwrap(),
            ty: self.1.to_f32().unwrap_or(0.0),
        }
    }
}

impl Stranslate for f32 {
    fn translate(self) -> Transform {
        Transform::Translate {
            tx: self.to_f32().unwrap(),
            ty: 0.0,
        }
    }
}

impl Stranslate for i32 {
    fn translate(self) -> Transform {
        Transform::Translate {
            tx: self as f32,
            ty: 0.0,
        }
    }
}

/// Help trait to construct [`Transform::Scale`]
pub trait Sscale {
    fn scale(self) -> Transform;
}

impl<X, Y> Sscale for (X, Y)
where
    X: ToPrimitive,
    Y: ToPrimitive,
{
    fn scale(self) -> Transform {
        Transform::Scale {
            sx: self.0.to_f32().unwrap(),
            sy: Some(self.1.to_f32().unwrap()),
        }
    }
}

impl Sscale for f32 {
    fn scale(self) -> Transform {
        Transform::Scale {
            sx: self.to_f32().unwrap(),
            sy: None,
        }
    }
}

impl Sscale for i32 {
    fn scale(self) -> Transform {
        Transform::Scale {
            sx: self as f32,
            sy: None,
        }
    }
}

/// Help trait to construct [`Transform::Rotate`]
pub trait Srotate {
    fn rotate(self) -> Transform;
}

impl<X, Y, Z> Srotate for (X, Y, Z)
where
    X: ToPrimitive,
    Y: ToPrimitive,
    Z: ToPrimitive,
{
    fn rotate(self) -> Transform {
        Transform::Rotate {
            angle: self.0.to_f32().unwrap(),
            cx: self.1.to_f32().unwrap(),
            cy: self.2.to_f32().unwrap(),
        }
    }
}

impl Srotate for f32 {
    fn rotate(self) -> Transform {
        Transform::Rotate {
            angle: self,
            cx: 0.0,
            cy: 0.0,
        }
    }
}

impl Srotate for i32 {
    fn rotate(self) -> Transform {
        Transform::Rotate {
            angle: self as f32,
            cx: 0.0,
            cy: 0.0,
        }
    }
}

/// Help trait to construct [`Transform::SkewX`]
pub trait SskewX {
    fn skewx(self) -> Transform;
}

impl<X> SskewX for X
where
    X: ToPrimitive,
{
    fn skewx(self) -> Transform {
        Transform::SkewX(self.to_f32().unwrap())
    }
}

/// Help trait to construct [`Transform::SkewY`]
pub trait SskewY {
    fn skewy(self) -> Transform;
}

impl<X> SskewY for X
where
    X: ToPrimitive,
{
    fn skewy(self) -> Transform {
        Transform::SkewY(self.to_f32().unwrap())
    }
}
