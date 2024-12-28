use crate::opcode::data::{Number, Transform};

/// Help trait to construct [`Transform::Translate`]
pub trait Stranslate {
    fn translate(self) -> Transform;
}

impl<X, Y> Stranslate for (X, Y)
where
    Number: From<X> + From<Y>,
{
    fn translate(self) -> Transform {
        Transform::Translate {
            tx: Number::from(self.0).0,
            ty: Number::from(self.1).0,
        }
    }
}

impl Stranslate for f32 {
    fn translate(self) -> Transform {
        Transform::Translate { tx: self, ty: 0.0 }
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
    Number: From<X> + From<Y>,
{
    fn scale(self) -> Transform {
        Transform::Scale {
            sx: Number::from(self.0).0,
            sy: Some(Number::from(self.1).0),
        }
    }
}

impl Sscale for f32 {
    fn scale(self) -> Transform {
        Transform::Scale { sx: self, sy: None }
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
    Number: From<X> + From<Y> + From<Z>,
{
    fn rotate(self) -> Transform {
        Transform::Rotate {
            angle: Number::from(self.0).0,
            cx: Number::from(self.1).0,
            cy: Number::from(self.2).0,
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
    Number: From<X>,
{
    fn skewx(self) -> Transform {
        Transform::SkewX(Number::from(self).0)
    }
}

/// Help trait to construct [`Transform::SkewY`]
pub trait SskewY {
    fn skewy(self) -> Transform;
}

impl<X> SskewY for X
where
    Number: From<X>,
{
    fn skewy(self) -> Transform {
        Transform::SkewY(Number::from(self).0)
    }
}
