use mlang::{
    codegen::{codegen, CoreGen},
    parse, semantic_analyze,
};
use parserc::ParseContext;

#[test]
fn test_vglang() {
    let mut input = ParseContext::from(include_str!("../../vglang/vglang.ml"));
    let mut opcodes = match parse(&mut input) {
        Ok(opcodes) => opcodes,
        Err(err) => {
            input.report().eprint();
            panic!("parser vglang.ml failed: {}", err);
        }
    };

    semantic_analyze(&mut opcodes, &mut input);

    if input.report_size() > 0 {
        input.report().eprint();
    }

    codegen(&mut opcodes, CoreGen::default());
}
