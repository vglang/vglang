use std::{fs::create_dir_all, path::Path};

use vglang::{binary::writer::to_binary, opcode::Opcode, svg::writer::to_svg};
use vglang_spec::run_spec;

#[test]
fn test_spec() {
    run_spec(write_json);
    run_spec(write_binary);
    run_spec(write_svg);
}

fn write_json(catalog: &str, case: &str, opcodes: &[Opcode]) {
    let path: &Path = env!("CARGO_MANIFEST_DIR").as_ref();

    let output_dir = path.join("spec").join(catalog);

    if !output_dir.exists() {
        create_dir_all(&output_dir).unwrap();
    }

    let output = output_dir.join(case).with_extension("json");

    std::fs::write(output, serde_json::to_string_pretty(opcodes).unwrap()).unwrap();
}

fn write_svg(catalog: &str, case: &str, opcodes: &[Opcode]) {
    let path: &Path = env!("CARGO_MANIFEST_DIR").as_ref();

    let output_dir = path.join("spec").join(catalog);

    if !output_dir.exists() {
        create_dir_all(&output_dir).unwrap();
    }

    let output = output_dir.join(case).with_extension("svg");

    std::fs::write(output, to_svg(opcodes).unwrap()).unwrap();
}

fn write_binary(catalog: &str, case: &str, opcodes: &[Opcode]) {
    let path: &Path = env!("CARGO_MANIFEST_DIR").as_ref();

    let output_dir = path.join("spec").join(catalog);

    if !output_dir.exists() {
        create_dir_all(&output_dir).unwrap();
    }

    let output = output_dir.join(case).with_extension("vgb");

    std::fs::write(output, to_binary(opcodes).unwrap()).unwrap();
}
