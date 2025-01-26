use divan::Bencher;
use vglang::{
    sexpr::{BuildContext, Graphics},
    svg::writer::to_svg,
};
use vglang_spec::filter::fecolormatrix_01;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn filter(bencher: Bencher) {
    let mut builder = BuildContext::default();
    fecolormatrix_01().build(&mut builder);

    bencher.bench(|| to_svg(&builder.0).unwrap());
}
