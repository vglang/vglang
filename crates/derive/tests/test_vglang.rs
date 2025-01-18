// use std::path::PathBuf;

// use parserc::{ParseContext, PrintReport};
// use vglang_derive::{codegen::gen, parse, semantic_analyze};

// #[test]
// fn test_vglang() {
//     let mut input = ParseContext::from(include_str!("../../vglang/vglang.ml"));

//     let mut opcodes = match parse(&mut input) {
//         Ok(opcodes) => opcodes,
//         Err(err) => {
//             input.report().print_reports();
//             panic!("{}", err);
//         }
//     };

//     if input.report_size() > 0 {
//         input.report().print_reports();
//         panic!("parase vglang.ml failed.");
//     }

//     semantic_analyze(&mut opcodes, &mut input);

//     if input.report_size() > 0 {
//         input.report().print_reports();
//         panic!("vglang.ml semantic analyze failed.");
//     }

//     let ml = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ml.rs");

//     gen(&opcodes, ml);
// }
