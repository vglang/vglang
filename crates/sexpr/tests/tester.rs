use std::path::Path;

use heck::ToLowerCamelCase;
use serde::Serialize;
use vglang_sexpr::{
    apply,
    operand::{Color, Fill, Number, Rect, Stroke},
    Graphic,
};
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

    let name = name.to_lower_camel_case();

    let output_file_path = dir.join(format!("{}.svg", name));

    println!("    write output: {:?}", output_file_path);

    std::fs::write(output_file_path, output).unwrap();

    let output_file_path = dir.join(format!("{}.json", name));

    println!("    write output: {:?}", output_file_path);

    std::fs::write(output_file_path, serde_json::to_string(&program.0).unwrap()).unwrap();

    let output_file_path = dir.join(format!("{}.bson", name));

    println!("    write output: {:?}", output_file_path);

    std::fs::write(
        output_file_path,
        bson::to_bson(&program.0).unwrap().to_string(),
    )
    .unwrap();

    let output_file_path = dir.join(format!("{}.mpk", name));

    println!("    write output: {:?}", output_file_path);

    let mut buf = Vec::new();

    let mut serializer = rmp_serde::Serializer::new(&mut buf);

    program.0.serialize(&mut serializer).unwrap();

    std::fs::write(output_file_path, buf).unwrap();
}

pub fn border<G, W, H>(width: W, height: H) -> impl Graphic<G>
where
    G: Builder,
    Number: From<W> + From<H>,
{
    apply(
        (Fill::default(), Stroke::from(Color::blue).width(2)),
        Rect::from((
            1,
            1,
            Number::from(width).0 - 1.0,
            Number::from(height).0 - 1.0,
        )),
    )
}
