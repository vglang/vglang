use ml::compiler::{analyzer::semantic_analyze, parser::parse};
use parserc::{ParseContext, PrintReport};

#[test]
fn test_vglang() {
    let mut ctx = ParseContext::from(include_str!("../../vglang/vglang.ml"));

    let mut opcodes = match parse(&mut ctx) {
        Ok(opcodes) => opcodes,
        Err(err) => {
            ctx.report().print_reports();
            panic!("{}", err);
        }
    };

    semantic_analyze(&mut opcodes, &mut ctx);

    if ctx.report_size() > 0 {
        ctx.report().print_reports();
        panic!("semantic analyze error");
    }
}
