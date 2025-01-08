#[cfg(docsrs)]
fn ml_gen() {}

#[cfg(not(docsrs))]
fn ml_gen() {
    use std::{fs, path::PathBuf, process::Command};

    use mlang::{codegen::CodeGen, parse, parserc::ParseContext, SemanticAnalyzer};

    let mut input = ParseContext::from(include_str!("./vglang.ml"));
    let opcodes = match parse(&mut input) {
        Ok(opcodes) => opcodes,
        Err(err) => {
            input.report().eprint();
            panic!("parser vglang.ml failed: {}", err);
        }
    };

    if let Err(err) = SemanticAnalyzer::new(&opcodes).analyze(&mut input) {
        input.report().eprint();
        panic!("parser vglang.ml failed: {}", err);
    }

    let token_stream = CodeGen::default().gen(&opcodes);

    let gen_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/opcode.rs");

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
