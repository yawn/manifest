use std::io;
use std::path::Path;
use std::process::{Command, Output};

const fn path_to_fixture() -> impl AsRef<Path> {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/test_crate")
}

fn run(cmd: &str, target: &str) -> io::Result<Output> {
    Command::new("cargo")
        .current_dir(path_to_fixture())
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

#[cfg(feature = "objects")]
#[test]
pub fn test_build_contains_no_duplicated_literals() {
    let output = Command::new("cargo")
        .current_dir(path_to_fixture())
        .arg("build")
        .args(&["--bin", "opt"])
        .args(&["--features", "objects"])
        .arg("--release")
        .output()
        .unwrap();

    if !output.status.success() {
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }

    let output = Command::new("strings")
        .current_dir(path_to_fixture())
        .arg("target/release/opt")
        .output()
        .unwrap();

    if !output.status.success() {
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }

    let all_strings = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = all_strings.lines().collect();

    let messages: Vec<&str> = lines
        .iter()
        .filter(|line| line.contains("user login"))
        .copied()
        .collect();

    let display_count = messages
        .iter()
        .filter(|line| line.contains("user login (TEST_CRATE_00001)"));

    assert_eq!(2, messages.len());
    assert_eq!(1, display_count.count());
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
