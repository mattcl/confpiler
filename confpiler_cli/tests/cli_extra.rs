//! So these can't be run with the rest of the integration tests. We probably
//! need to rethink the integration testing schedule.
use assert_cmd::{assert::Assert, Command};
use std::fs;

/// Retrieve golden output from file
fn golden(directory: &str, file: &str) -> String {
    let desired = format!("tests/golden/{directory}/{file}");
    fs::read_to_string(&desired).expect(&format!("could not open golden output file: {}", &desired))
}

fn parse_stdout_stderr(input: &str) -> (String, String) {
    let mut parts = input.split("-STDERR-\n").map(|s| s.to_string());
    let stdout = parts.next().unwrap_or_default();
    let stderr = parts.next().unwrap_or_default();
    (stdout, stderr)
}

/// run the command
fn run(op: &str, args: &[&str]) -> Assert {
    let mut cmd = Command::cargo_bin("confpiler").expect("could not get desired binary");
    cmd.arg(op).args(args).assert()
}

#[test]
fn raw_output() {
    let expected = golden("raw", "build_output.txt");
    let (stdout, stderr) = parse_stdout_stderr(&expected);
    let result = run(
        "build",
        &[
            "tests/fixtures/global_default.yaml",
            "tests/fixtures/conf_dir",
            "--env",
            "production",
            "--raw",
        ],
    );

    result.success().stdout(stdout).stderr(stderr);
}
