use std::panic::RefUnwindSafe;

pub mod compositing;
pub mod filter;

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

    test!(compositing, mask_01);
    test!(compositing, opacity_01);
    test!(filter, enable_background_01);
}
