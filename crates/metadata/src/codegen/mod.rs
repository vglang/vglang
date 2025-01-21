use std::path::Path;

use quote::quote;

#[allow(unused)]
mod ext;
mod opcode;
mod sexpr;
mod svg;

/// Generate rust codes to disk file.
///
/// If any error occurs, this fn will raise an panic.
pub fn gen<P: AsRef<std::path::Path>>(opcodes: &[crate::ir::Stat], target_dir: P) {
    let opcode_mod = opcode::OpcodeModGen::default().gen(opcodes);

    let sexpr_mod = sexpr::SexprModGen::new("super::opcode::", 16).gen(opcodes);

    let svg_mod = svg::SvgModGen::new("super::opcode::").gen(opcodes);

    let opcode_file = target_dir.as_ref().join("opcode.rs");
    let sexpr_file = target_dir.as_ref().join("sexpr.rs");
    let svg_file = target_dir.as_ref().join("svg.rs");

    let mod_file = target_dir.as_ref().join("mod.rs");

    let mod_content = quote! {
        pub mod opcode;

        #[cfg(feature = "sexpr")]
        #[cfg_attr(docsrs, doc(cfg(feature = "sexpr")))]
        pub mod sexpr;

        #[cfg(feature = "svg")]
        #[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
        pub mod svg;
    };

    write_and_fmt_rs("opcode.rs", opcode_file, opcode_mod.to_string());
    write_and_fmt_rs("sexpr.rs", sexpr_file, sexpr_mod.to_string());
    write_and_fmt_rs("svg.rs", svg_file, svg_mod.to_string());

    write_and_fmt_rs("mod.rs", mod_file, mod_content.to_string());
}

fn write_and_fmt_rs<C: AsRef<[u8]>, P: AsRef<Path>>(debug: &str, path: P, content: C) {
    std::fs::write(path.as_ref(), content).expect(&format!("write {}", debug));

    std::process::Command::new("rustfmt")
        .arg(path.as_ref())
        .output()
        .expect(&format!("format {}", debug));
}
