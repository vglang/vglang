use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};
use parserc::{FromSrc, ParseContext};
use vglang::lang::ir::LitColor;

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn hex3() {
    LitColor::parse(&mut ParseContext::from("#f23")).unwrap();
}

#[divan::bench]
fn hex6() {
    LitColor::parse(&mut ParseContext::from("#f02030")).unwrap();
}

#[divan::bench]
fn nom_hex6() {
    hex_color("#f02030").unwrap();
}

#[divan::bench]
fn recognized_yellow() {
    LitColor::parse(&mut ParseContext::from("color.yellowgreen")).unwrap();
}

#[divan::bench]
fn recognized_aliceblue() {
    LitColor::parse(&mut ParseContext::from("color.aliceblue")).unwrap();
}
