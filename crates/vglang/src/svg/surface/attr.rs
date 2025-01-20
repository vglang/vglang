use crate::{
    codegen::svg::{SvgAttrValueWriter, SvgAttrsWriter, SvgContext, SvgNode},
    opcode::{Path, PreserveAspectRatio, ViewBox},
};

impl SvgAttrsWriter for Path {
    fn write_svg_attrs<C, Node, E>(&self, _: &C, _: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        Ok(())
    }
}

impl SvgAttrsWriter for ViewBox {
    fn write_svg_attrs<C, Node, E>(&self, ctx: &C, node: &mut Node) -> Result<(), Node::Error>
    where
        C: SvgContext<Error = E>,
        Node: SvgNode<Error = E>,
    {
        node.set_svg_attr(
            "viewbox",
            &format!(
                "{} {} {} {}",
                ctx.valueof(&self.minx)?,
                ctx.valueof(&self.miny)?,
                ctx.valueof(&self.width)?,
                ctx.valueof(&self.height)?
            ),
        )?;

        if let Some(aspect) = &self.aspect {
            let aspect = ctx.valueof(aspect)?;

            match aspect {
                PreserveAspectRatio::None => {
                    node.set_svg_attr("preserveAspectRatio", "none")?;
                }
                PreserveAspectRatio::XMinYMin(meet_or_slice) => {
                    node.set_svg_attr(
                        "preserveAspectRatio",
                        &format!("xMinYMin {}", meet_or_slice.to_svg_attr_value()),
                    )?;
                }
                PreserveAspectRatio::XMidYMin(meet_or_slice) => {
                    node.set_svg_attr(
                        "preserveAspectRatio",
                        &format!("xMidYMin {}", meet_or_slice.to_svg_attr_value()),
                    )?;
                }
                PreserveAspectRatio::XMaxYMin(meet_or_slice) => {
                    node.set_svg_attr(
                        "preserveAspectRatio",
                        &format!("xMaxYMin {}", meet_or_slice.to_svg_attr_value()),
                    )?;
                }
                PreserveAspectRatio::XMinYMid(meet_or_slice) => {
                    node.set_svg_attr(
                        "preserveAspectRatio",
                        &format!("xMinYMid {}", meet_or_slice.to_svg_attr_value()),
                    )?;
                }
                PreserveAspectRatio::XMidYMid(meet_or_slice) => {
                    node.set_svg_attr(
                        "preserveAspectRatio",
                        &format!("xMidYMid {}", meet_or_slice.to_svg_attr_value()),
                    )?;
                }
                PreserveAspectRatio::XMaxYMid(meet_or_slice) => {
                    node.set_svg_attr(
                        "preserveAspectRatio",
                        &format!("xMaxYMid {}", meet_or_slice.to_svg_attr_value()),
                    )?;
                }
                PreserveAspectRatio::XMinYMax(meet_or_slice) => {
                    node.set_svg_attr(
                        "preserveAspectRatio",
                        &format!("xMinYMax {}", meet_or_slice.to_svg_attr_value()),
                    )?;
                }
                PreserveAspectRatio::XMidYMax(meet_or_slice) => {
                    node.set_svg_attr(
                        "preserveAspectRatio",
                        &format!("xMidYMax {}", meet_or_slice.to_svg_attr_value()),
                    )?;
                }
                PreserveAspectRatio::XMaxYMax(meet_or_slice) => {
                    node.set_svg_attr(
                        "preserveAspectRatio",
                        &format!("xMaxYMax {}", meet_or_slice.to_svg_attr_value()),
                    )?;
                }
            }
        }

        Ok(())
    }
}
