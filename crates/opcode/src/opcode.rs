//! Defines the optimised intermediate instructions.

use crate::operand::{
    Canvas, Circle, ClipPath, ClipPathed, ClipRule, Ellipse, FeDistantLight, FePointLight,
    FeSpotLight, Fill, Filter, Font, GradientStop, Id, LightingColor, LinearGradient, Mask, Masked,
    Opacity, Path, Pattern, Polygon, Polyline, RadialGradient, Rect, Stroke, Text, TextLayout,
    TextPath, TextSpan, Transform, Use, Variable,
};

/// Opcodes for vglang.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    /// Create a new `canvas` to draw vector graphics.
    Canvas(Box<Canvas>),
    /// Render a `text` element.
    Text(Box<Text>),
    /// Render a `text-span` element.
    TextSpan(Box<TextSpan>),
    /// Render a `textPath` element.
    TextPath(Box<TextPath>),
    /// characters content of the text/text-span element.
    Characters(Box<Variable<String>>),
    /// Apply inheritable property `Fill` to all children instructions.
    Fill(Box<Fill>),
    /// Apply inheritable property `Stroke` to all children instructions.
    Stroke(Box<Stroke>),
    /// Render a `rect` element.
    Rect(Box<Rect>),
    /// Render a `circle` element.
    Circle(Box<Circle>),
    /// Apply inheritable property `Font` to all children instructions.
    Font(Box<Font>),
    /// Apply inheritable property `TextLayout` to all children instructions.
    TextLayout(Box<TextLayout>),
    Transform(Box<Variable<Transform>>),
    Path(Box<Path>),
    Polyline(Box<Polyline>),
    Polygon(Box<Polygon>),
    Id(Box<Id>),
    Use(Box<Use>),
    LinearGradient(Box<LinearGradient>),
    RadialGradient(Box<RadialGradient>),
    GradientStop(Box<GradientStop>),
    Pattern(Box<Pattern>),
    Ellipse(Box<Ellipse>),
    Mask(Box<Mask>),
    Masked(Box<Masked>),
    Opacity(Box<Opacity>),
    ClipPath(Box<ClipPath>),
    ClipPathed(Box<ClipPathed>),
    ClipRule(Box<Variable<ClipRule>>),
    Filter(Box<Filter>),
    FeDistantLight(Box<FeDistantLight>),
    FePointLight(Box<FePointLight>),
    FeSpotLight(Box<FeSpotLight>),
    LightingColor(Box<LightingColor>),
    /// Popup elements, indicates that the popup elements ared fully rendered.
    Pop(usize),
}

macro_rules! opcode_from {
    ($v: tt) => {
        impl From<$v> for Opcode {
            fn from(value: $v) -> Self {
                Self::$v(Box::new(value))
            }
        }
    };
}

opcode_from!(LightingColor);
opcode_from!(FeSpotLight);
opcode_from!(FePointLight);
opcode_from!(FeDistantLight);
opcode_from!(Filter);
opcode_from!(ClipPath);
opcode_from!(ClipPathed);
opcode_from!(Opacity);
opcode_from!(Masked);
opcode_from!(Mask);
opcode_from!(Ellipse);
opcode_from!(Pattern);
opcode_from!(Canvas);
opcode_from!(Text);
opcode_from!(TextSpan);
opcode_from!(TextPath);
opcode_from!(Fill);
opcode_from!(Stroke);
opcode_from!(Circle);
opcode_from!(Rect);
opcode_from!(Font);
opcode_from!(TextLayout);
opcode_from!(Path);
opcode_from!(Polyline);
opcode_from!(Polygon);
opcode_from!(Id);
opcode_from!(Use);
opcode_from!(LinearGradient);
opcode_from!(RadialGradient);
opcode_from!(GradientStop);

impl From<Variable<ClipRule>> for Opcode {
    fn from(value: Variable<ClipRule>) -> Self {
        Self::ClipRule(Box::new(value))
    }
}

impl From<Variable<String>> for Opcode {
    fn from(value: Variable<String>) -> Self {
        Self::Characters(Box::new(value))
    }
}

impl From<Transform> for Opcode {
    fn from(value: Transform) -> Self {
        Self::Transform(Box::new(Variable::Constant(value)))
    }
}
