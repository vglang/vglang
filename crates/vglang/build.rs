#[cfg(docsrs)]
fn ml_gen() {}

#[cfg(not(docsrs))]
fn ml_gen() {
    use std::path::PathBuf;

    use mlang::{codegen::gen, parse, parserc::ParseContext, semantic_analyze};

    let mut input = ParseContext::from(include_str!("./vglang.ml"));
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
        panic!("semantic anlayze failed: vglang.ml");
    }

    // let ml = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ml.rs");

    // gen(&opcodes, ml);
}

fn main() {
    println!("cargo::rerun-if-changed=vglang.ml");
    ml_gen();
}
