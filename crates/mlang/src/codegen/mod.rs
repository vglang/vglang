use std::path::Path;

use quote::quote;

use crate::opcode::Opcode;

pub mod opcode;
pub mod sexpr;

/// Generate rust codes to disk file.
///
/// If any error occurs, this fn will raise an panic.
pub fn gen<P: AsRef<Path>>(opcodes: &[Opcode], target: P) {
    let opcode_mod = opcode::OpcodeModGen::default().gen(opcodes);

    let sexpr_mod = sexpr::SexprModGen::new("super::opcode::").gen(opcodes);

    let content = quote! {
        /// This mod is automatically generated by `mlang` tools, please does not modify it manually.
        pub mod opcode {
            #opcode_mod
        }

        /// This mod is automatically generated by `mlang` tools, please does not modify it manually.
        pub mod sexpr {
            #sexpr_mod
        }
    };

    std::fs::write(target.as_ref(), content.to_string()).expect("write source code");

    std::process::Command::new("rustfmt")
        .arg(target.as_ref())
        .output()
        .expect("execute fmt on opcode.rs");
}
