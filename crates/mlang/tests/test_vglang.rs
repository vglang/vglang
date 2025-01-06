use std::path::Path;

use mlang::parse;

#[test]
fn test_vglang() {
    let opcodes = parse(include_str!("./vglang.ml")).unwrap();

    std::fs::write(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/vglang.json"),
        serde_json::to_string_pretty(&opcodes).unwrap(),
    )
    .unwrap();
}
