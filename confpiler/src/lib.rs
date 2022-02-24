#![doc = include_str!("../README.md")]
mod config;
pub mod error;

pub use crate::config::FlatConfig;
pub use crate::config::FlatConfigBuilder;
pub use crate::config::MergeWarning;
