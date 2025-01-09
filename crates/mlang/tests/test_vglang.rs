use mlang::{
    codegen::{codegen, CoreGen},
    parse, SemanticAnalyzer,
};
use parserc::ParseContext;

#[test]
fn test_vglang() {
    let mut input = ParseContext::from(include_str!("./vglang.ml"));
    let mut opcodes = match parse(&mut input) {
        Ok(opcodes) => opcodes,
        Err(err) => {
            input.report().eprint();
            panic!("parser vglang.ml failed: {}", err);
        }
    };

    if let Err(err) = SemanticAnalyzer::new(&mut opcodes).analyze(&mut input) {
        input.report().eprint();
        panic!("parser vglang.ml failed: {}", err);
    }

    codegen(&mut opcodes, CoreGen::default());
}
