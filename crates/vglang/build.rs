#[cfg(docsrs)]
fn ml_gen() {}

#[cfg(not(docsrs))]
fn ml_gen() {
    use std::{fs, path::PathBuf, process::Command};

    use mlang::{parse, parserc::ParseContext, semantic_analyze};

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

    let opcode_gen = mlang::codegen::opcode::CodeGen::default();

    let opcode_token_stream = opcode_gen.generate(&opcodes);

    // let token_stream = codegen(&opcodes, OpcodeCodeGen::default());

    let opcode_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/opcode.rs");

    fs::write(&opcode_file, opcode_token_stream.to_string()).unwrap();

    Command::new("rustfmt")
        .arg(opcode_file.to_str().unwrap().to_string())
        .output()
        .expect("execute fmt on opcode.rs");
}

fn main() {
    println!("cargo::rerun-if-changed=vglang.ml");
    ml_gen();
}
