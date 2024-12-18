use std::path::Path;

use heck::ToLowerCamelCase;
use vglang_sexpr::Graphic;
use vglang_svg::{Builder, Program, SvgBuilder, SvgTarget, Target};

pub async fn svg(name: &str, test: impl Graphic<SvgBuilder>) {
    println!("{}", name);

    let mut builder = SvgTarget::default().build();

    test.draw(&mut builder);

    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("dsl");

    if !dir.exists() {
        std::fs::create_dir(dir.as_path()).unwrap();
    }

    let program = builder.create().await.unwrap();

    let output = program.run(&mut Default::default()).await.unwrap();

    let output_file_path = dir.join(format!("{}.svg", name.to_lower_camel_case()));

    println!("    write output: {:?}", output_file_path);

    std::fs::write(output_file_path, output).unwrap();
}
