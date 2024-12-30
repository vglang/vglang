use std::{collections::HashMap, fs::create_dir_all, path::Path};

use futures::executor::block_on;
use vglang::surface::{Program, Source};
use vglang_spec::run_spec;
use vglang_svg::SvgRenderer;

#[test]
fn test_spec() {
    run_spec(write_svg);
}

fn write_svg(catalog: &str, case: &str, source: Source<'_>) {
    let path: &Path = env!("CARGO_MANIFEST_DIR").as_ref();

    let output_dir = path.join("spec").join(catalog);

    if !output_dir.exists() {
        create_dir_all(&output_dir).unwrap();
    }

    let output = output_dir.join(case).with_extension("svg");

    match source {
        Source::Opcode(opcodes) => {
            let renderer = SvgRenderer::new(opcodes.to_vec());

            std::fs::write(output, block_on(renderer.run(&HashMap::new())).unwrap()).unwrap();
        }
        _ => unimplemented!("only support opcode source."),
    }
}
