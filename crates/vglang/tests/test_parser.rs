use std::{fs::DirEntry, path::PathBuf};

use parserc::ParseContext;
use vglang::lang::parser::parse;

#[test]
fn test_spec() {
    _ = pretty_env_logger::try_init();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lang/spec");

    let entries = std::fs::read_dir(&path)
        .unwrap()
        .collect::<std::io::Result<Vec<DirEntry>>>()
        .unwrap();

    for entry in entries {
        let code = std::fs::read_to_string(entry.path()).unwrap();
        let mut ctx = ParseContext::from(code.as_str());

        parse(&mut ctx).unwrap();
    }
}
