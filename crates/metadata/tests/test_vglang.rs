use parserc::{ParseContext, PrintReport};
use vglang_metadata::parser::parse;

#[test]
fn test_vglang() {
    let mut input = ParseContext::from(include_str!("../../vglang/vglang.ml"));

    let mut _opcodes = match parse(&mut input) {
        Ok(opcodes) => opcodes,
        Err(err) => {
            input.report().print_reports();
            panic!("{}", err);
        }
    };
}
