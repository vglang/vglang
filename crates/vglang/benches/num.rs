use parserc::ParseContext;
use vglang::lang::parser::parse_lit_num_int_length;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn vglang_num() {
    parse_lit_num_int_length(&mut ParseContext::from("-3.1415e-10")).unwrap();
}

#[divan::bench]
fn vglang_length() {
    parse_lit_num_int_length(&mut ParseContext::from("-3.1px")).unwrap();
}

#[divan::bench]
fn rust_num() {
    let a = "-3.1415e-10".parse::<f32>().unwrap();
    assert_eq!(a, -3.1415e-10);
}

#[divan::bench]
fn vglang_int() {
    parse_lit_num_int_length(&mut ParseContext::from("0x11")).unwrap();
}

#[divan::bench]
fn rust_int() {
    i32::from_str_radix("11", 16).unwrap();
}
