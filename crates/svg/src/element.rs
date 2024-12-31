use vglang::opcode::el::*;
use xml_builder::XMLElement;

use crate::Context;

/// A svg rendering element must implement this trait.
pub trait SvgElement {
    /// Create a new [`XMLElement`] for this svg element.
    fn create_xml_element<C>(&self, ctx: &C) -> XMLElement
    where
        C: Context;
}

impl<T> SvgElement for Box<T>
where
    T: SvgElement,
{
    fn create_xml_element<C>(&self, ctx: &C) -> XMLElement
    where
        C: Context,
    {
        self.as_ref().create_xml_element(ctx)
    }
}

macro_rules! impl_svg_element {
    ($ty: ident, $el: ident) => {
        impl SvgElement for $ty {
            fn create_xml_element<C>(&self, _ctx: &C) -> XMLElement
            where
                C: Context,
            {
                XMLElement::new(stringify!($el))
            }
        }
    };
}

impl SvgElement for Group {
    fn create_xml_element<C>(&self, _ctx: &C) -> XMLElement
    where
        C: Context,
    {
        let el = XMLElement::new("g");

        el
    }
}

impl SvgElement for Text {
    fn create_xml_element<C>(&self, _ctx: &C) -> XMLElement
    where
        C: Context,
    {
        let el = XMLElement::new("text");

        if let Some(v) = &self.x {
            // el.add_attribute("x", ctx.valueof(v));
        }

        el
    }
}

impl_svg_element!(TextSpan, tspan);
impl_svg_element!(Canvas, svg);
impl_svg_element!(LinearGradient, linearGradient);
impl_svg_element!(RadialGradient, radialGradient);
impl_svg_element!(Pattern, pattern);
impl_svg_element!(Filter, filter);
impl_svg_element!(FeComponentTransfer, feComponentTransfer);
impl_svg_element!(FeDiffuseLighting, feDiffuseLighting);
impl_svg_element!(FeMerge, feMerge);
impl_svg_element!(FeSpecularLighting, feSpecularLighting);
impl_svg_element!(Mask, mask);
impl_svg_element!(ClipPath, clipPath);
impl_svg_element!(TextPath, textPath);
impl_svg_element!(Use, use);
impl_svg_element!(Path, path);
impl_svg_element!(GradientStop, stop);
impl_svg_element!(Rect, rect);
impl_svg_element!(Circle, circle);
impl_svg_element!(Ellipse, ellipse);
impl_svg_element!(Line, line);
impl_svg_element!(Polyline, polyline);
impl_svg_element!(Polygon, polygon);
impl_svg_element!(FeDistantLight, feDistantLight);
impl_svg_element!(FeSpotLight, feSpotLight);
impl_svg_element!(FePointLight, fePointLight);
impl_svg_element!(FeBlend, feBlend);
impl_svg_element!(FeColorMatrix, feColorMatrix);
impl_svg_element!(FeFuncR, feFuncR);
impl_svg_element!(FeFuncG, feFuncG);
impl_svg_element!(FeFuncB, feFuncB);
impl_svg_element!(FeFuncA, feFuncA);
impl_svg_element!(FeComposite, feComposite);
impl_svg_element!(FeDisplacementMap, feDisplacementMap);
impl_svg_element!(FeConvolveMatrix, feConvolveMatrix);
impl_svg_element!(FeGaussianBlur, feGaussianBlur);
impl_svg_element!(FeMergeNode, feMergeNode);
impl_svg_element!(FeImage, feImage);
impl_svg_element!(FeFlood, feFlood);
impl_svg_element!(FeOffset, feOffset);
impl_svg_element!(FeMorphology, feMorphology);
impl_svg_element!(FeTile, feTile);
impl_svg_element!(FeTurbulence, feTurbulence);
