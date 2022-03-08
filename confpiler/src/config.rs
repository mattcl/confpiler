use config::{Config, ConfigError, File, Value, ValueKind};
use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::error::{ConfpilerError, Result};

/// A representation of a flattened, compiled configuration.
///
/// When constructed via the builder, this produces a set of key/value pairs
/// where the keys are produced from flattening the nested structure of a config
/// file and converting the values into their string representations.
///
/// See the crate examples for more detailed usage.
///
/// # Examples
/// Given
/// ```text
/// ## default.yaml
/// foo:
///     bar: 10
///     baz: false
/// hoof: doof
///
/// ## production.yaml
/// foo:
///     baz: true
/// ```
///
/// then running the following
/// ```no_run
/// use confpiler::FlatConfig;
/// # use confpiler::error::ConfpilerError;
/// # fn main() -> Result<(), ConfpilerError> {
/// let (conf, warnings) = FlatConfig::builder()
///     .add_config("foo/default")
///     .add_config("foo/production")
///     .build()?;
///
/// // or equivalently
/// let mut builder = FlatConfig::builder();
/// builder.add_config("foo/default");
/// builder.add_config("foo/production");
/// let (conf, warnings) = builder.build()?;
/// # Ok(())
/// # }
/// ```
///
/// produces a mapping like
/// ```text
/// "FOO__BAR": "10"
/// "FOO__BAZ": "true"
/// "HOOF": "doof"
/// ```
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct FlatConfig {
    origin: String,

    items: HashMap<String, String>,
}

impl FlatConfig {
    /// Get a [FlatConfigBuilder] instance.
    ///
    /// # Examples
    /// ```
    /// use confpiler::{FlatConfig, FlatConfigBuilder};
    /// let builder = FlatConfig::builder();
    ///
    /// assert_eq!(builder, FlatConfigBuilder::default());
    /// ```
    pub fn builder() -> FlatConfigBuilder {
        FlatConfigBuilder::default()
    }

    /// Convenience method for getting reference to the internal key/value map.
    pub fn items(&self) -> &HashMap<String, String> {
        &self.items
    }

    /// Merge another [FlatConfig] into `self`.
    ///
    /// See [MergeWarning] for the kinds of warnings returned by this function
    /// and when/why they are generated.
    ///
    /// # Examples
    /// ```
    /// use confpiler::FlatConfig;
    ///
    /// // we're using default here for the example, making it pointless, since
    /// // they're both empty, but this is just for illustration
    /// let mut a = FlatConfig::default();
    /// let b = FlatConfig::default();
    ///
    /// let warnings = a.merge(&b);
    /// ```
    pub fn merge(&mut self, other: &Self) -> Vec<MergeWarning> {
        let mut warnings = Vec::new();

        for (k, v) in other.items.iter() {
            self.items
                .entry(k.to_string())
                .and_modify(|e| {
                    if e == v {
                        warnings.push(MergeWarning::RedundantValue {
                            overrider: other.origin.clone(),
                            key: k.to_string(),
                            value: e.clone(),
                        });
                    } else {
                        *e = v.to_string();
                    }
                })
                .or_insert(v.to_string());
        }

        warnings
    }
}

/// This is the builder for [FlatConfig].
///
/// An instance of this will normally be obtained by invoking [FlatConfig::builder]
///
/// # Examples
/// ```
/// // This example is included for reference, but prefer using
/// // FlatConfig::builder() to get a builder instance.
/// use confpiler::FlatConfigBuilder;
///
/// let mut builder = FlatConfigBuilder::default();
/// builder.add_config("foo/default");
/// builder.add_config("foo/production");
/// builder.with_separator("__"); // this is the default
/// builder.with_array_separator(","); // this is the default
///
/// // let's not actually do this in the docs
/// // let (conf, warnings) = builder.build()?;
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FlatConfigBuilder {
    prefix: Option<String>,
    configs: Vec<String>,
    separator: String,
    array_separator: String,
}

impl FlatConfigBuilder {
    pub const DEFAULT_SEPARATOR: &'static str = "__";
    pub const DEFAULT_ARRAY_SEPARATOR: &'static str = ",";

    /// Adds the given config path to the list of configs.
    ///
    ///
    /// * Ordering is important here, as values in the last added config will
    /// overwrite those in the previously added configs.
    /// * Actual loading of the specified config files does not happen until
    /// [build()](FlatConfigBuilder::build) is invoked.
    /// * The supported config names are the same as supported by the `config-rs`
    /// * Specifying the same config twice will result in an error when
    /// [build()](FlatConfigBuilder::build) is invoked.
    /// crate.
    ///
    /// # Examples
    /// ```
    /// use confpiler::FlatConfig;
    /// let mut builder = FlatConfig::builder();
    /// builder.add_config("foo/default");
    /// ```
    pub fn add_config(&mut self, config: &str) -> &mut Self {
        self.configs.push(config.to_string());
        self
    }

    /// Specifies the separator to use when flattening nested structures.
    ///
    /// The default separator is `__`, and is used to join the keys of a
    /// nested structure into a single, top-level key.
    ///
    /// # Examples
    /// ```
    /// use confpiler::FlatConfig;
    /// let mut builder = FlatConfig::builder();
    /// builder.with_separator("__"); // this is the default
    /// ```
    pub fn with_separator(&mut self, separator: &str) -> &mut Self {
        self.separator = separator.to_string();
        self
    }

    /// Specifies the separator to use when joining arrays
    ///
    /// This default array separator is `,`, and is used to join the values of
    /// an array into a single [String]. As a reminder, this crate only supports
    /// "simple" arrays that do not contain additional nested structures.
    ///
    /// # Examples
    /// ```
    /// use confpiler::FlatConfig;
    /// let mut builder = FlatConfig::builder();
    /// builder.with_array_separator(","); // this is the default
    /// ```
    pub fn with_array_separator(&mut self, separator: &str) -> &mut Self {
        self.array_separator = separator.to_string();
        self
    }

    /// Specifies a prefix to be prepended to all generated keys.
    ///
    /// This prefix will **always** be converted to ascii uppercase and will be
    /// be separated from the rest of the generated key by the separator used
    /// by the builder.
    ///
    /// # Examples
    /// ```
    /// use confpiler::FlatConfig;
    /// let mut builder = FlatConfig::builder();
    /// builder.with_prefix("foo"); // this is the default
    /// ```
    pub fn with_prefix(&mut self, prefix: &str) -> &mut Self {
        self.prefix = Some(prefix.to_ascii_uppercase());
        self
    }

    /// Attempt to produce a [FlatConfig] without consuming the builder.
    ///
    /// This results in an error in the following scenarios:
    /// * No configs were specified.
    /// * Flattening any given config results in a duplicate key within the same
    /// file (`foo:` and `Foo:` in the same file, `foo_bar:` and `foo: bar:` in
    /// the same file, etc.).
    /// * A config contains an array that itself contains some nested structure.
    /// * A config is invalid or not found as far as `config-rs` can determine.
    ///
    /// # Examples
    /// ```
    /// use confpiler::FlatConfig;
    /// ```
    pub fn build(&self) -> Result<(FlatConfig, Vec<MergeWarning>)> {
        if self.configs.is_empty() {
            return Err(ConfpilerError::NoConfigSpecified);
        }

        let mut seen_configs: HashSet<&str> = HashSet::new();

        // the origin for the overall config will be whatever was first in
        // the list
        let mut flat_config = FlatConfig {
            // this unwrap is safe because we just checked
            origin: self.configs.first().unwrap().to_string(),
            items: HashMap::new(),
        };
        let mut warnings = Vec::new();

        for conf_path in self.configs.iter() {
            // so this adds some complexity, but it's probably a better user
            // experience?
            if seen_configs.contains(conf_path.as_str()) {
                return Err(ConfpilerError::DuplicateConfig(conf_path.to_string()));
            } else {
                seen_configs.insert(&conf_path.as_str());
            }

            // attempt to load every specified config
            let conf = Config::builder()
                .add_source(File::with_name(&conf_path))
                .build()?;

            let input = conf.cache.into_table()?;

            let mut out = HashMap::new();
            flatten_into(&input, &mut out, self.prefix.as_ref(), &self.separator, &self.array_separator)?;
            let working_config = FlatConfig {
                origin: conf_path.to_string(),
                items: out,
            };

            let mut working_warnings = flat_config.merge(&working_config);
            warnings.append(&mut working_warnings);
        }

        Ok((flat_config, warnings))
    }
}

impl Default for FlatConfigBuilder {
    fn default() -> Self {
        Self {
            prefix: None,
            configs: Vec::new(),
            separator: Self::DEFAULT_SEPARATOR.to_string(),
            array_separator: Self::DEFAULT_ARRAY_SEPARATOR.to_string(),
        }
    }
}

/// An enumeration of possible warning values regarding config merging.
///
/// These warnings occur as the result of merging two [FlatConfig] instances
/// together. They are not necessarily errors, but are provided for the caller
/// to treat as such if they wish.
///
/// # Examples
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MergeWarning {
    /// This variant indicates that we attempted to set a value in a key but it
    /// already contained that value. This is useful for detecting when a
    /// configuration file specifies a value when it does not need to, because
    /// the value it is specifying was already set.
    ///
    /// This does not reliably that the _final_ value for a given key was
    /// unchanged, as merging files `A -> B -> C` where `B` contained the
    /// redundant value does not mean that `C` did not then change that value
    /// to something else.
    RedundantValue {
        overrider: String,
        key: String,
        value: String,
    },
}

impl fmt::Display for MergeWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RedundantValue {
                ref overrider,
                ref key,
                ref value,
            } => {
                write!(f, "'{overrider}' is attempting to override '{key}' with '{value}', but the key already contains that value")
            }
        }
    }
}

pub(crate) fn flatten_into(
    input: &HashMap<String, Value>,
    output: &mut HashMap<String, String>,
    prefix: Option<&String>,
    separator: &str,
    array_separator: &str,
) -> Result<()> {
    let mut components = Vec::new();
    if let Some(prefix) = prefix {
        components.push(prefix.clone());
    }
    flatten_into_inner(input, output, separator, array_separator, &mut components)
}

fn flatten_into_inner(
    input: &HashMap<String, Value>,
    output: &mut HashMap<String, String>,
    separator: &str,
    array_separator: &str,
    components: &mut Vec<String>,
) -> Result<()> {
    if input.is_empty() {
        return Ok(());
    }

    for (key, value) in input.iter() {
        // convert the current key to uppercase and add it to the list of
        // components so that we can form names with the current "path"
        let upper_key = key.to_ascii_uppercase();
        components.push(upper_key);
        match &value.kind {
            // omit these because they have no meaning
            ValueKind::Nil => {}

            // If we encounter another table, we just need to recurse
            ValueKind::Table(ref table) => {
                flatten_into_inner(table, output, separator, array_separator, components)?;
            }

            // Arrays are only supported if they contain primitive/str types
            // because what does it actually mean to flatten an array into
            // separate environment variables? We could do something like
            // FOO_0 = "a"
            // FOO_1 = "b"
            // FOO_2 = "c"
            // etc.
            // but consider what the parser consuming said variables would have
            // to look like? And what does it do when some arbitrary index is
            // a complex type like an array or a map?
            //
            // Instead, it's simpler if we just convert the array into a
            // sequence-separated string, which limits the kinds of things we
            // can store in an array
            ValueKind::Array(ref array) => {
                let canidate = components.join(separator);

                if output.contains_key(&canidate) {
                    return Err(ConfpilerError::DuplicateKey(canidate.clone()));
                }

                let val = array
                    .iter()
                    .cloned()
                    .map(|e| e.into_string())
                    .collect::<std::result::Result<Vec<String>, ConfigError>>()
                    // TODO: this is actually an assumption about why this would fail - MCL - 2022-02-21
                    .map_err(|_| ConfpilerError::UnsupportedArray(canidate.clone()))?
                    .join(array_separator);

                output.insert(canidate, val);
            }

            // for everything else, we want to add the key/value to the output
            _ => {
                let canidate = components.join(separator);

                if output.contains_key(&canidate) {
                    return Err(ConfpilerError::DuplicateKey(canidate.clone()));
                }

                // this clone might be unnecessary and we could just convert
                // directly into a string, but I think I want the error to be
                // raised if the interface changes to not allow arbitrary things
                // to be converted to string.
                output.insert(canidate, value.clone().into_string()?);
            }
        }

        // we have to remove the key we pushed
        components.pop();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    mod flat_config {
        use super::super::*;

        #[test]
        fn builder_yields_a_default_builder() {
            assert_eq!(FlatConfig::builder(), FlatConfigBuilder::default());
        }

        #[test]
        fn merging() {
            let mut a = FlatConfig {
                origin: "origin1".to_string(),
                items: HashMap::from([
                    ("herp".to_string(), "derp".to_string()),
                    ("hoof".to_string(), "changeme".to_string()),
                ]),
            };
            let b = FlatConfig {
                origin: "origin2".to_string(),
                items: HashMap::from([
                    ("foo".to_string(), "bar".to_string()),
                    ("hoof".to_string(), "doof".to_string()),
                ]),
            };

            let expected = FlatConfig {
                origin: "origin1".to_string(),
                items: HashMap::from([
                    ("foo".to_string(), "bar".to_string()),
                    ("hoof".to_string(), "doof".to_string()),
                    ("herp".to_string(), "derp".to_string()),
                ]),
            };

            let warnings = a.merge(&b);

            assert_eq!(a, expected);
            assert!(warnings.is_empty());
        }

        #[test]
        fn merging_when_overriding_with_same_value_generates_warnings() {
            let mut a = FlatConfig {
                origin: "origin1".to_string(),
                items: HashMap::from([
                    ("herp".to_string(), "derp".to_string()),
                    ("hoof".to_string(), "changeme".to_string()),
                ]),
            };
            let b = FlatConfig {
                origin: "origin2".to_string(),
                items: HashMap::from([
                    ("foo".to_string(), "bar".to_string()),
                    ("herp".to_string(), "derp".to_string()),
                    ("hoof".to_string(), "changeme".to_string()),
                ]),
            };

            let expected = FlatConfig {
                origin: "origin1".to_string(),
                items: HashMap::from([
                    ("foo".to_string(), "bar".to_string()),
                    ("herp".to_string(), "derp".to_string()),
                    ("hoof".to_string(), "changeme".to_string()),
                ]),
            };

            let warnings = a.merge(&b);

            assert_eq!(a, expected);

            assert_eq!(warnings.len(), 2);

            // we're sensitive to ordering here because of the hashing, so just
            // assert individually
            assert!(warnings.contains(&MergeWarning::RedundantValue {
                overrider: "origin2".to_string(),
                key: "herp".to_string(),
                value: "derp".to_string(),
            }));

            assert!(warnings.contains(&MergeWarning::RedundantValue {
                overrider: "origin2".to_string(),
                key: "hoof".to_string(),
                value: "changeme".to_string(),
            }));
        }
    }

    mod flat_config_builder {
        use super::super::*;

        #[test]
        fn defaults() {
            let builder = FlatConfigBuilder::default();
            assert!(builder.configs.is_empty());
            assert_eq!(builder.separator, "__".to_string());
            assert_eq!(builder.array_separator, ",".to_string());
        }

        #[test]
        fn adding_configs() {
            let mut builder = FlatConfigBuilder::default();
            builder.add_config("foo/bar");
            builder.add_config("foo/baz");

            let expected = vec!["foo/bar".to_string(), "foo/baz".to_string()];

            assert_eq!(builder.configs, expected);
        }

        #[test]
        fn specifying_prefix() {
            let mut builder = FlatConfigBuilder::default();
            builder.with_prefix("foo");

            assert_eq!(builder.prefix, Some("FOO".to_string()));
        }

        #[test]
        fn specifying_separator() {
            let mut builder = FlatConfigBuilder::default();
            builder.with_separator("*");

            assert_eq!(builder.separator, "*".to_string());
        }

        #[test]
        fn specifying_array_separator() {
            let mut builder = FlatConfigBuilder::default();
            builder.with_array_separator("---");

            assert_eq!(builder.array_separator, "---".to_string());
        }
    }

    mod flatten_into {
        use super::super::*;

        // so this is a PITA to create, but it's probably? Better than trying
        // to load a real config file from disk. And I have more control over
        // the types
        fn valid_input() -> HashMap<String, Value> {
            let origin = "test".to_string();
            let input = HashMap::from([
                (
                    "foo".to_string(),
                    Value::new(Some(&origin), ValueKind::Float(10.2)),
                ),
                (
                    "bar".to_string(),
                    Value::new(Some(&origin), ValueKind::String("Hello".to_string())),
                ),
                (
                    "baz".to_string(),
                    Value::new(
                        Some(&origin),
                        ValueKind::Table(HashMap::from([
                            (
                                "herp".to_string(),
                                Value::new(Some(&origin), ValueKind::Boolean(false)),
                            ),
                            (
                                "derp".to_string(),
                                Value::new(Some(&origin), ValueKind::I64(15)),
                            ),
                            (
                                "hoof".to_string(),
                                Value::new(
                                    Some(&origin),
                                    ValueKind::Table(HashMap::from([(
                                        "doof".to_string(),
                                        Value::new(Some(&origin), ValueKind::I64(999)),
                                    )])),
                                ),
                            ),
                        ])),
                    ),
                ),
                (
                    "biz".to_string(),
                    Value::new(
                        Some(&origin),
                        ValueKind::Array(vec![
                            Value::new(Some(&origin), ValueKind::Boolean(false)),
                            Value::new(Some(&origin), ValueKind::I64(1111)),
                            Value::new(Some(&origin), ValueKind::String("Goodbye".to_string())),
                        ]),
                    ),
                ),
            ]);

            input
        }

        #[test]
        fn accepts_empty_input() {
            let mut out = HashMap::new();
            let input = HashMap::new();

            let res = flatten_into(&input, &mut out, None, "__", ",");

            assert!(res.is_ok());
            assert!(out.is_empty());
        }

        #[test]
        fn flattens_valid_input() {
            let mut out = HashMap::new();
            let input = valid_input();

            let expected: HashMap<String, String> = HashMap::from([
                ("FOO".to_string(), "10.2".to_string()),
                ("BAR".to_string(), "Hello".to_string()),
                ("BAZ__HERP".to_string(), "false".to_string()),
                ("BAZ__DERP".to_string(), "15".to_string()),
                ("BAZ__HOOF__DOOF".to_string(), "999".to_string()),
                ("BIZ".to_string(), "false,1111,Goodbye".to_string()),
            ]);

            let res = flatten_into(&input, &mut out, None, "__", ",");

            assert!(res.is_ok());
            assert_eq!(out, expected);
        }

        #[test]
        fn supports_prefixing() {
            let mut out = HashMap::new();
            let input = valid_input();

            let expected: HashMap<String, String> = HashMap::from([
                ("PRE__FOO".to_string(), "10.2".to_string()),
                ("PRE__BAR".to_string(), "Hello".to_string()),
                ("PRE__BAZ__HERP".to_string(), "false".to_string()),
                ("PRE__BAZ__DERP".to_string(), "15".to_string()),
                ("PRE__BAZ__HOOF__DOOF".to_string(), "999".to_string()),
                ("PRE__BIZ".to_string(), "false,1111,Goodbye".to_string()),
            ]);

            let prefix = Some("PRE".to_string());

            let res = flatten_into(&input, &mut out, prefix.as_ref(), "__", ",");

            assert!(res.is_ok());
            assert_eq!(out, expected);
        }

        #[test]
        fn uses_the_specified_separators() {
            let mut out = HashMap::new();
            let input = valid_input();

            let expected: HashMap<String, String> = HashMap::from([
                ("FOO".to_string(), "10.2".to_string()),
                ("BAR".to_string(), "Hello".to_string()),
                ("BAZ*HERP".to_string(), "false".to_string()),
                ("BAZ*DERP".to_string(), "15".to_string()),
                ("BAZ*HOOF*DOOF".to_string(), "999".to_string()),
                ("BIZ".to_string(), "false 1111 Goodbye".to_string()),
            ]);

            let res = flatten_into(&input, &mut out, None, "*", " ");

            assert!(res.is_ok());
            assert_eq!(out, expected);
        }

        #[test]
        fn errors_on_duplicate_keys() {
            let mut out = HashMap::new();
            let valid = valid_input();

            let mut invalid = valid.clone();
            invalid.insert(
                "fOo".to_string(),
                Value::new(Some(&"test".to_string()), ValueKind::Float(1.0)),
            );

            let res = flatten_into(&invalid, &mut out, None, "__", ",");

            assert!(res.is_err());

            match res.unwrap_err() {
                ConfpilerError::DuplicateKey(key) => assert_eq!(key, "FOO".to_string()),
                e => panic!("unexpected error variant: {}", e),
            };

            // including duplicates because of nesting
            let mut invalid = valid.clone();
            invalid.insert(
                "baz__herp".to_string(),
                Value::new(Some(&"test".to_string()), ValueKind::Boolean(true)),
            );

            let mut out = HashMap::new();
            let res = flatten_into(&invalid, &mut out, None, "__", ",");

            assert!(res.is_err());

            match res.unwrap_err() {
                ConfpilerError::DuplicateKey(key) => assert_eq!(key, "BAZ__HERP".to_string()),
                e => panic!("unexpected error variant: {}", e),
            };
        }

        #[test]
        fn errors_on_unsupported_array() {
            let mut out = HashMap::new();
            let valid = valid_input();

            let origin = "test".to_string();
            let mut invalid = valid.clone();
            invalid.insert(
                "biz".to_string(),
                Value::new(
                    Some(&"test".to_string()),
                    ValueKind::Array(vec![
                        Value::new(Some(&origin), ValueKind::Boolean(false)),
                        Value::new(Some(&origin), ValueKind::Table(HashMap::new())),
                        Value::new(Some(&origin), ValueKind::String("Goodbye".to_string())),
                    ]),
                ),
            );

            let res = flatten_into(&invalid, &mut out, None, "__", ",");

            assert!(res.is_err());

            match res.unwrap_err() {
                ConfpilerError::UnsupportedArray(key) => assert_eq!(key, "BIZ".to_string()),
                e => panic!("unexpected error variant: {}", e),
            };
        }
    }
}
