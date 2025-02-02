use std::path::Path;

use mlang::lang::compile;

#[test]
fn test_vglang() {
    let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../vglang/src/ml");

    compile(include_str!("../../vglang/vglang.ml"), target, false).unwrap();
}
