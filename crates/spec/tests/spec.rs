use std::{collections::HashMap, fs::create_dir_all, path::Path};

use futures::executor::block_on;
use vglang::{
    surface::{Program, Source, Surface},
    svg::surface::Svg,
};
use vglang_spec::run_spec;

#[test]
fn test_spec() {
    run_spec(write_json);
    run_spec(write_svg);
}

#[allow(unused)]
fn write_json(catalog: &str, case: &str, source: Source<'_>) {
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

fn write_svg(catalog: &str, case: &str, source: Source<'_>) {
    let path: &Path = env!("CARGO_MANIFEST_DIR").as_ref();

    let output_dir = path.join("spec").join(catalog);

    if !output_dir.exists() {
        create_dir_all(&output_dir).unwrap();
    }

    let output = output_dir.join(case).with_extension("svg");

    let content = block_on(async move {
        Svg.build(source)
            .await
            .unwrap()
            .run(&HashMap::new())
            .await
            .unwrap()
    });

    std::fs::write(output, content).unwrap();
}
