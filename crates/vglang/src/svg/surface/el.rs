use crate::{codegen::svg::SvgNodeWriter, opcode::Path};

impl SvgNodeWriter for Path {
    fn to_svg_node_name(&self) -> &str {
        "path"
    }
}
