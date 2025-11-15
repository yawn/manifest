use std::io;
use std::process::{Command, Output};

fn run(cmd: &str, target: &str) -> io::Result<Output> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/test_crate");

    Command::new("cargo")
        .current_dir(path)
        .arg(cmd)
        .args(&["--bin", target])
        .output()
}

#[test]
pub fn test_constants() {
    test_crate::test_constants();
}

#[cfg(feature = "objects")]
#[test]
pub fn test_refs() {
    test_crate::test_refs();
}

#[cfg(feature = "objects")]
#[test]
pub fn test_objects() {
    test_crate::test_objects();
}

#[test]
fn test_build_all() {
    let output = run("build", "all").expect("failed to build with all constants used");

    if !output.status.success() {
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }
}

#[test]
fn test_build_deprecated() {
    let output = run("build", "deprecated").expect("failed to build with all constants used");

    if !output.status.success() {
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }

    let err = "use of deprecated constant `messages::TEST_CRATE_00100_DEPRECATED_FEATURE`: Use feature XYZ instead";
    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains(err));
}

#[test]
fn test_build_some() {
    let output = run("build", "some").expect("failed to build with all constants used");

    if output.status.success() {
        panic!("{}", String::from_utf8(output.stdout).unwrap());
    }

    let err = "`TEST_CRATE_00005_PASSWORD_RESET` is never used";
    let output = String::from_utf8(output.stderr).unwrap();

    assert!(output.contains(err));
}
