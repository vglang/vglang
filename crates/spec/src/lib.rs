//! This is the official integration test suites for vglang.
//!
//! Most of the cases defined in this crate are directly rewritten from [`SVG 1.1`] example codes.
//!
//! [`SVG 1.1`]: https://www.w3.org/TR/SVG11/Overview.html

pub mod text;

/// The main entry of the spec.
pub fn run_spec<F>(tester: F)
where
    F: Fn(&str, &str, vglang::surface::Source<'_>),
{
    macro_rules! test {
        ($catalog: ident, $case: ident) => {
            tester(
                stringify!($catalog),
                stringify!($case),
                vglang::sexpr::BuildContext::create_source($catalog::$case()),
            );
        };
    }

    test!(text, text_0_1);
    test!(text, tspan_0_1);
    test!(text, tspan_0_2);
}
