//! This is the official integration test suites for vglang.
//!
//! Most of the cases defined in this crate are directly rewritten from [`SVG 1.1`] example codes.
//!
//! [`SVG 1.1`]: https://www.w3.org/TR/SVG11/Overview.html

use std::panic::RefUnwindSafe;

pub mod compositing;
pub mod filter;
pub mod gradients;
pub mod path;
pub mod pattern;
pub mod shapes;
pub mod text;

/// The main entry of the spec.
pub fn run_spec<F>(tester: F)
where
    F: Fn(&str, &str, vglang::surface::Source<'_>) + RefUnwindSafe,
{
    macro_rules! test {
        ($catalog: ident, $case: ident) => {
            print!("run spec {}({})", stringify!($catalog), stringify!($case));

            let result = std::panic::catch_unwind(|| {
                tester(
                    stringify!($catalog),
                    stringify!($case),
                    vglang::sexpr::BuildContext::create_source($catalog::$case()),
                )
            });

            match result {
                Ok(_) => {
                    println!("... ok");
                }
                Err(err) => {
                    println!("... failed");
                    println!("{:#?}", err);
                }
            }
        };
    }

    test!(text, text_0_1);
    test!(text, tspan_0_1);
    test!(text, tspan_0_2);
    test!(text, tspan_0_3);
    test!(text, tspan_0_4);
    test!(text, tspan_0_5);
    test!(text, rtl_text);
    test!(text, text_decoration_01);
    test!(text, toap_01);
    test!(shapes, rect_01);
    test!(shapes, rect_02);
    test!(shapes, ellipse_01);
    test!(shapes, line_01);
    test!(shapes, polyline_01);
    test!(shapes, polygon_01);
    test!(gradients, lingrad_01);
    test!(gradients, radgrad_01);
    test!(pattern, pattern_01);
    test!(path, triangle_01);
    test!(path, cubic_01);
    test!(path, quad_01);
    test!(path, arcs_01);
    test!(compositing, mask_01);
    test!(compositing, opacity_01);
}
