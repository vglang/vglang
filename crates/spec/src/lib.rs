use std::panic::RefUnwindSafe;

use vglang::opcode::Opcode;

pub mod compositing;
pub mod filter;
pub mod gradients;
pub mod path;

/// The main entry of the spec.
pub fn run_spec<F>(tester: F)
where
    F: Fn(&str, &str, &[Opcode]) + RefUnwindSafe,
{
    macro_rules! test {
        ($catalog: ident, $case: ident) => {
            print!("run spec {}({})", stringify!($catalog), stringify!($case));

            let result = std::panic::catch_unwind(|| {
                use vglang::sexpr::Graphics;

                let mut ctx = vglang::sexpr::BuildContext::default();

                $catalog::$case().build(&mut ctx);

                tester(stringify!($catalog), stringify!($case), &ctx.0)
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
    test!(filter, feblend_01);
    test!(filter, fecolormatrix_01);
    test!(filter, fe_component_transfer_01);
    test!(gradients, lingrad_01);
    test!(gradients, radgrad_01);
    test!(path, triangle_01);
    test!(path, cubic_01);
    test!(path, quad_01);
    test!(path, arcs_01);
}
