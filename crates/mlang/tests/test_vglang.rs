use mlang::lang::compile;

#[test]
fn test_vglang() {
    compile(
        include_str!("../../vglang/vglang.ml"),
        "../../vglang/src/ml",
    )
    .unwrap();
}
