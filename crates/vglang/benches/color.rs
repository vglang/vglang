use parserc::ParseContext;
use vglang::lang::parser::{parse_hex_color, parse_recognized_color};

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

#[divan::bench]
fn recognized_yellow() {
    parse_recognized_color(&mut ParseContext::from("color.yellowgreen")).unwrap();
}

#[divan::bench]
fn recognized_aliceblue() {
    parse_recognized_color(&mut ParseContext::from("color.aliceblue")).unwrap();
}
