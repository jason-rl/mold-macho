use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cases = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("tests/cases");
    let enabled_archs = &[
        #[cfg(feature = "arm64")]
        "arm64",
        #[cfg(feature = "x86_64")]
        "x86_64",
    ];
    mold_macho_tests::run(&cases, Path::new(env!("CARGO_BIN_EXE_mold")), enabled_archs)
}
