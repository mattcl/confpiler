use std::collections::HashMap;
use confpiler::{FlatConfig, MergeWarning};

// These are all effectively "happy path" tests that serve as smoke tests

#[test]
fn loading_single_file() {
    let expected = HashMap::from([
        ("FOO__BAR".to_string(), "10".to_string()),
        ("FOO__BAZ".to_string(), "99.9".to_string()),
        ("HOOF".to_string(), "true,false,hello".to_string()),
        ("DOOF__HERP__DERP".to_string(), "goodbye".to_string()),
        ("UNDER_SCORED__KEY".to_string(), "https://foo.bar".to_string()),
    ]);

    let (config, warnings) = FlatConfig::builder()
        .add_config("tests/fixtures/file_one")
        .build()
        .expect("Failed to construct config");

    assert!(warnings.is_empty());

    assert_eq!(config.items(), &expected);
}

#[test]
fn specifying_separators() {
    let expected = HashMap::from([
        ("FOO_BAR".to_string(), "10".to_string()),
        ("FOO_BAZ".to_string(), "99.9".to_string()),
        ("HOOF".to_string(), "true false hello".to_string()),
        ("DOOF_HERP_DERP".to_string(), "goodbye".to_string()),
        ("UNDER_SCORED_KEY".to_string(), "https://foo.bar".to_string()),
    ]);

    let (config, warnings) = FlatConfig::builder()
        .add_config("tests/fixtures/file_one")
        .with_separator("_")
        .with_array_separator(" ")
        .build()
        .expect("Failed to construct config");

    assert!(warnings.is_empty());
    assert_eq!(config.items(), &expected);
}

#[test]
fn multiple_files() {
    let expected = HashMap::from([
        ("FOO__BAR".to_string(), "10".to_string()),
        ("FOO__BAZ".to_string(), "222.2".to_string()),
        ("HOOF".to_string(), "arrays,are,replaced,not,merged".to_string()),
        ("DOOF__HERP__DERP".to_string(), "goodbye".to_string()),
        ("UNDER_SCORED__KEY".to_string(), "https://foo.bar".to_string()),
        ("ANOTHER".to_string(), "one".to_string()),
    ]);

    let (config, warnings) = FlatConfig::builder()
        .add_config("tests/fixtures/file_one")
        .add_config("tests/fixtures/file_two")
        .add_config("tests/fixtures/file_three")
        .build()
        .expect("Failed to construct config");

    assert!(warnings.is_empty());
    assert_eq!(config.items(), &expected);
}

#[test]
fn generating_warnings() {
    let expected = HashMap::from([
        ("FOO__BAR".to_string(), "10".to_string()),
        ("FOO__BAZ".to_string(), "333.3".to_string()),
        ("HOOF".to_string(), "true,false,hello".to_string()),
        ("DOOF__HERP__DERP".to_string(), "goodbye".to_string()),
        ("UNDER_SCORED__KEY".to_string(), "https://foo.bar".to_string()),
    ]);

    let (config, warnings) = FlatConfig::builder()
        .add_config("tests/fixtures/file_one")
        .add_config("tests/fixtures/file_two_warnings")
        .add_config("tests/fixtures/file_three_warnings")
        .build()
        .expect("Failed to construct config");

    assert_eq!(warnings.len(), 2);

    assert!(warnings.contains(&MergeWarning::RedundantValue {
        overrider: "tests/fixtures/file_two_warnings".to_string(),
        key: "FOO__BAR".to_string(),
        value: "10".to_string(),
    }));

    assert!(warnings.contains(&MergeWarning::RedundantValue {
        overrider: "tests/fixtures/file_three_warnings".to_string(),
        key: "DOOF__HERP__DERP".to_string(),
        value: "goodbye".to_string(),
    }));

    assert_eq!(config.items(), &expected);
}

#[test]
fn invalid_configurations() {
    let res = FlatConfig::builder()
        .add_config("tests/fixtures/duplicate_key")
        .build();

    assert!(res.is_err());

    let res = FlatConfig::builder()
        .add_config("tests/fixtures/invalid_array")
        .build();

    assert!(res.is_err());
}
