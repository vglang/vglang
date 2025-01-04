use mlang::parser::parse;

#[test]
fn test_vglang() {
    parse(include_str!("./vglang.ml")).unwrap();
}
