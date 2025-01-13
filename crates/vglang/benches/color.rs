use parserc::ParseContext;
use vglang::lang::parser::parse_hex_color;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn hex3() {
    parse_hex_color(&mut ParseContext::from("#f23")).unwrap();
}

#[divan::bench]
fn hex6() {
    parse_hex_color(&mut ParseContext::from("#f02030")).unwrap();
}
