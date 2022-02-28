//! Crate error definitions and associated conversions
use config::ConfigError;

use crate::MergeWarning;

/// Convenience alias for Results returned by this crate
pub type Result<T> = std::result::Result<T, ConfpilerError>;

/// ConfpilerError enumerates all possible errors returned by this library
#[derive(Debug)]
pub enum ConfpilerError {
    /// All other instances of [ConfigError].
    ConfigError(ConfigError),

    /// Indicates a config file was specified more than once.
    DuplicateConfig(String),

    /// Indicates a config file would result in duplicated flattened keys.
    DuplicateKey(String),

    /// Indicates no config files specified when building a
    /// [FlatConfig](crate::FlatConfig).
    NoConfigSpecified,

    /// Indicates a config contains an array that is unsupported.
    ///
    /// An unsupported array contains nested values.
    UnsupportedArray(String),

    /// This is a convenience wrapper for treating warnings as erros.
    Warnings(Vec<MergeWarning>),
}

impl std::error::Error for ConfpilerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ConfpilerError::ConfigError(_) => None,
            ConfpilerError::DuplicateConfig(_) => None,
            ConfpilerError::DuplicateKey(_) => None,
            ConfpilerError::NoConfigSpecified => None,
            ConfpilerError::UnsupportedArray(_) => None,
            ConfpilerError::Warnings(_) => None,
        }
    }
}

impl std::fmt::Display for ConfpilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ConfpilerError::ConfigError(ref err) => err.fmt(f),
            ConfpilerError::DuplicateConfig(ref config) => {
                write!(f, "the config \"{config}\" was specified twice")
            }
            ConfpilerError::DuplicateKey(ref key) => {
                write!(f, "the key \"{key}\" would be overwritten by another value in the same configuration file")
            }
            ConfpilerError::NoConfigSpecified => {
                write!(
                    f,
                    "must specify at least one config path via `builder.add_config`"
                )
            }
            ConfpilerError::UnsupportedArray(ref key) => {
                write!(f, "the array at \"{key}\" is unsupported (arrays must not contain arrays or maps to be condidered valid)")
            }
            ConfpilerError::Warnings(ref warnings) => {
                let mut out = warnings.iter().map(|w| w.to_string()).collect::<Vec<String>>();
                out.sort();
                write!(f, "{}", out.join("\n"))
            }
        }
    }
}

impl From<ConfigError> for ConfpilerError {
    fn from(err: ConfigError) -> ConfpilerError {
        ConfpilerError::ConfigError(err)
    }
}

impl From<Vec<MergeWarning>> for ConfpilerError {
    fn from(value: Vec<MergeWarning>) -> ConfpilerError {
        ConfpilerError::Warnings(value)
    }
}

