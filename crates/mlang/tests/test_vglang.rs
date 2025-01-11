use std::path::PathBuf;

use mlang::{codegen::gen, parse, semantic_analyze};
use parserc::ParseContext;

#[test]
fn test_vglang() {
    let mut input = ParseContext::from(include_str!("../../vglang/vglang.ml"));

    let mut opcodes = match parse(&mut input) {
        Ok(opcodes) => opcodes,
        Err(err) => {
            input.report().eprint();
            panic!("{}", err);
        }
    };

    if input.report_size() > 0 {
        input.report().eprint();
        panic!("parase vglang.ml failed.");
    }

    semantic_analyze(&mut opcodes, &mut input);

    if input.report_size() > 0 {
        input.report().eprint();
        panic!("vglang.ml semantic analyze failed.");
    }

    let ml = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ml.rs");

    gen(&opcodes, ml);
}
