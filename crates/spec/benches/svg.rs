use divan::Bencher;
use vglang::{
    sexpr::{BuildContext, Graphics},
    targets::svg::writer::to_svg,
};
use vglang_spec::filter::fecolormatrix_01;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn filter_to_svg(bencher: Bencher) {
    let mut builder = BuildContext::default();
    fecolormatrix_01().build(&mut builder);

    bencher.bench(|| to_svg(&builder.0).unwrap());
}

#[divan::bench]
fn filter_build() {
    let mut builder = BuildContext::default();
    fecolormatrix_01().build(&mut builder);
}
