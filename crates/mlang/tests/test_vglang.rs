use mlang::parse;
use parserc::ParseContext;

#[test]
fn test_vglang() {
    let mut input = ParseContext::from(include_str!("./vglang.ml"));
    if let Err(err) = parse(&mut input) {
        input.report().eprint();
        panic!("parser vglang.ml failed: {}", err);
    }

    // std::fs::write(
    //     Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/vglang.json"),
    //     serde_json::to_string_pretty(&opcodes).unwrap(),
    // )
    // .unwrap();
}
