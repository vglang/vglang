use divan::bench;
use nom::{number::complete::recognize_float, IResult};
use parserc::{FromSrc, ParseContext};
use vglang::lang::ir::LitNum;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[bench]
fn iter_parse_context() {
    let mut ctx = ParseContext::from("-3.1415e-10");

    while let (Some(_), _) = ctx.next() {}
}

#[bench]
fn parse_num() {
    LitNum::parse(&mut ParseContext::from("-3.1415e-10")).unwrap();
}

fn nom_parser(input: &str) -> IResult<&str, &str> {
    recognize_float(input)
}

#[bench]
fn nom_num() {
    nom_parser("-3.1415e-10").unwrap();
}

#[divan::bench]
fn rust_from_str() {
    let a = "-3.1415e-10".parse::<f32>().unwrap();
    assert_eq!(a, -3.1415e-10);
}
