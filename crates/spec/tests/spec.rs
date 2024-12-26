use std::{fs::create_dir_all, path::Path};

use vglang::surface::Source;
use vglang_spec::run_spec;

#[test]
fn test_spec() {
    run_spec(write_opcodes);
}

fn write_opcodes(catalog: &str, case: &str, source: Source<'_>) {
    let path: &Path = env!("CARGO_MANIFEST_DIR").as_ref();

    let output_dir = path.join("spec").join(catalog);

    if !output_dir.exists() {
        create_dir_all(&output_dir).unwrap();
    }

    let output = output_dir.join(case).with_extension("json");

    match source {
        Source::Opcode(cow) => {
            std::fs::write(output, serde_json::to_string_pretty(&cow).unwrap()).unwrap();
        }
        _ => unimplemented!("only support opcode source."),
    }
}
