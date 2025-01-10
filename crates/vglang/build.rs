#[cfg(docsrs)]
fn ml_gen() {}

#[cfg(not(docsrs))]
fn ml_gen() {
    use std::{fs, path::PathBuf, process::Command};

    use mlang::{
        codegen::{codegen, CoreGen},
        parse,
        parserc::ParseContext,
        semantic_analyze,
    };

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

    let token_stream = codegen(&opcodes, CoreGen::default());

    let gen_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ml.rs");

    fs::write(&gen_path, token_stream.to_string()).unwrap();

    Command::new("rustfmt")
        .arg(gen_path.to_str().unwrap().to_string())
        .output()
        .expect("execute fmt.");
}

fn main() {
    println!("cargo::rerun-if-changed=vglang.ml");
    ml_gen();
}
