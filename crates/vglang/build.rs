#[cfg(docsrs)]
fn ml_gen() {}

#[cfg(not(docsrs))]
fn ml_gen() {
    use std::path::PathBuf;

    use mlang::lang::{compile, rustgen::CodeGen};

    let target = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ml");

    compile(
        include_str!("./vglang.ml"),
        CodeGen::default().target(target),
    )
    .unwrap();
}

fn main() {
    println!("cargo::rerun-if-changed=vglang.ml");
    ml_gen();
}
