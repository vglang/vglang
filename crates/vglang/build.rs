#[cfg(docsrs)]
fn ml_gen() {}

#[cfg(not(docsrs))]
fn ml_gen() {
    use std::path::PathBuf;

    use ml::compiler::{
        analyzer::semantic_analyze,
        codegen::gen,
        parser::parse,
        parserc::{ParseContext, PrintReport},
    };

    let mut input = ParseContext::from(include_str!("./vglang.ml"));
    let mut opcodes = match parse(&mut input) {
        Ok(opcodes) => opcodes,
        Err(err) => {
            input.report().print_reports();
            panic!("parser vglang.ml failed: {}", err);
        }
    };

    semantic_analyze(&mut opcodes, &mut input);

    if input.report_size() > 0 {
        input.report().print_reports();
        panic!("semantic anlayze failed: vglang.ml");
    }

    let codegen = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/codegen");

    gen(&opcodes, codegen);
}

fn main() {
    println!("cargo::rerun-if-changed=vglang.ml");
    ml_gen();
}
