use std::path::Path;

use cotati_dsl::generator::IRGenerator;
use cotati_svg::{SvgDevice, VGLProgram};
use heck::ToLowerCamelCase;

pub async fn dsl_test<F>(name: &str, test: F)
where
    F: FnOnce(&mut IRGenerator),
{
    println!("{}", name);

    let mut generator = IRGenerator::default();

    test(&mut generator);

    let mut svg_device = SvgDevice::default();

    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("dsl");

    if !dir.exists() {
        std::fs::create_dir(dir.as_path()).unwrap();
    }

    let program = generator.compile(&mut svg_device).await.unwrap();

    let output = program.execute(&mut Default::default()).await.unwrap();

    let output_file_path = dir.join(format!("{}.svg", name.to_lower_camel_case()));

    println!("    write output: {:?}", output_file_path);

    std::fs::write(output_file_path, output).unwrap();
}
