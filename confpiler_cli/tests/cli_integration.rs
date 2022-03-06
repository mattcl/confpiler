//! most of the tests for the actual behavior of the compilation can be found
//! in the tests for the confpiler library. These tests are really more about
//! the interface of the command-line tool and file ordering behaviors

// annoyingly, we have to put all the macros at the top of the file, so
// readability is going to suffer a bit here
macro_rules! check {
    (@args check) => {
        []
    };
    (@args check_strict) => {
        ["--strict"]
    };
    (@body $name:ident, $outcome:ident) => {
        mod $name {
            use super::*;

            #[test]
            fn $outcome() {
                let args = check!(@args $name);
                let expected = golden(&format!("{}_output.txt", stringify!($name)));
                let (stdout, stderr) = parse_stdout_stderr(&expected);

                let result = run("check", &args);
                result
                    .$outcome()
                    .stdout(stdout)
                    .stderr(stderr);
            }
        }
    };
    (succeeds) => {
        check! { @body check, success }
        check! { @body check_strict, success }
    };
    (fails) => {
        check! { @body check, failure }
    };
    (fails_strict) => {
        check! { @body check, success }
        check! { @body check_strict, failure }
    }
}

macro_rules! build {
    (@args build$(, $($arg:literal),* $(,)?)?) => {
        [$($($arg,)*)?]
    };
    (@args build_strict$(, $($arg:literal),* $(,)?)?) => {
        build! { @args build, "--strict", $($($arg,)*)? }
    };
    (@json_body $name:ident, success) => {
        use serde_json;
        use std::collections::HashMap;

        #[test]
        fn success() {
            let args = build!(@args $name, "--json");

            // so we have no guarantee about ordering, so we ne need to
            // parse this stuff and assert we get the same result. Note
            // that this doesn't assert on the actual displayed format of
            // the output. There's also no difference in compiled output
            // if we succeed when running with --strict, so just use the
            // same output file as without strict
            let expected: HashMap<String, String> = serde_json::from_str(
                &golden("build_output.json")
            ).expect("could not parse expected output as json");

            let result = run("build", &args);
            let parsed: HashMap<String, String> = serde_json::from_slice(
                &result.get_output().stdout
            ).expect("coudl not parse command output as json");

            result.success();

            assert_eq!(parsed, expected);
        }
    };
    // there's a bit of repeating here, but maybe these eventually have
    // different error messages. For now, if we fail when attempting to compile
    // to json, it's just he same output as if we didn't pass the json flag
    (@json_body $name:ident, failure) => {
        #[test]
        fn failure() {
            let args = build!(@args $name, "--json");
            let expected = golden(&format!("{}_output.txt", stringify!($name)));
            let (stdout, stderr) = parse_stdout_stderr(&expected);
            let result = run("build", &args);
            result
                .failure()
                .stdout(stdout)
                .stderr(stderr);
        }
    };
    (@body $name:ident, $outcome:ident) => {
        mod $name {
            mod env {
                use super::super::*;

                #[test]
                fn $outcome() {
                    let args = build!(@args $name);
                    let expected = golden(&format!("{}_output.txt", stringify!($name)));
                    let (stdout, stderr) = parse_stdout_stderr(&expected);
                    let result = run("build", &args);

                    result
                        .$outcome()
                        .stdout(stdout)
                        .stderr(stderr);
                }
            }

            mod json {
                use super::super::*;
                build! { @json_body $name, $outcome }
            }

        }
    };
    (succeeds) => {
        build! { @body build, success }
        build! { @body build_strict, success }
    };
    (fails) => {
        build! { @body build, failure }
    };
    (fails_strict) => {
        build! { @body build, success }
        build! { @body build_strict, failure }
    }
}

macro_rules! integration_test {
    ($golden_dir:ident, $name:ident, [$($path:literal),+ $(,)?]$(, [$($arg:literal),+ $(,)?])?, $hf_outcome:ident$(,)?) => {
        mod $name {
            use assert_cmd::{Command, assert::Assert};
            use std::fs;
            use std::stringify;

            /// Retrieve golden output from file
            fn golden(file: &str) -> String {
                let desired = format!("tests/golden/{}/{}", stringify!($golden_dir), file);
                fs::read_to_string(&desired)
                    .expect(&format!("could not open golden output file: {}", &desired))
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
                cmd
                    .arg(op)
                    .args(&[$($path,)+$($($arg,)+)?])
                    .args(args)
                    .assert()
            }

            check! { $hf_outcome }
            build! { $hf_outcome }
        }
    };
    ($name:ident, [$($path:literal),+ $(,)?]$(, [$($arg:literal),+ $(,)?])?, $hf_outcome:ident$(,)?) => {
        integration_test! {
            $name,
            $name,
            [$($path,)+],
            $([$($arg,)*],)?
            $hf_outcome
        }
    }
}

// okay, now we can finally start declaring tests

integration_test! {
    simple,
    ["tests/fixtures/global_default.yaml"],
    succeeds,
}

integration_test! {
    shell_escaping,
    ["tests/fixtures/special_chars.yaml"],
    succeeds,
}

integration_test! {
    complex,
    [
        "tests/fixtures/global_default.yaml",
        "tests/fixtures/conf_dir",
    ],
    ["--env", "production"],
    succeeds,
}

integration_test! {
    customized,
    [
        "tests/fixtures/global_default.yaml",
        "tests/fixtures/conf_dir",
    ],
    [
        "--env",
        "production",
        "--separator",
        "___",
        "--array-separator",
        " ",
    ],
    succeeds,
}

integration_test! {
    warnings,
    [
        "tests/fixtures/conf_dir",
    ],
    ["--env", "staging"],
    fails_strict,
}

integration_test! {
    errors,
    [
        "tests/fixtures/conf_dir",
    ],
    ["--env", "development"],
    fails,
}

integration_test! {
    no_matching_env,
    [
        "tests/fixtures/conf_dir",
    ],
    ["--env", "missing"],
    succeeds,
}
