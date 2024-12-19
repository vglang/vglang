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

impl Stranslate for (i32, i32) {
    fn translate(self) -> Transform {
        Transform::Translate {
            tx: self.0 as f32,
            ty: self.1 as f32,
        }
    }
}

impl Stranslate for (f32, f32) {
    fn translate(self) -> Transform {
        Transform::Translate {
            tx: self.0 as f32,
            ty: self.1 as f32,
        }
    }
}

impl Stranslate for (i32, f32) {
    fn translate(self) -> Transform {
        Transform::Translate {
            tx: self.0 as f32,
            ty: self.1 as f32,
        }
    }
}

impl Stranslate for (f32, i32) {
    fn translate(self) -> Transform {
        Transform::Translate {
            tx: self.0 as f32,
            ty: self.1 as f32,
        }
    }
}
