use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use clap::{Args, Parser, Subcommand};
use confpiler::{FlatConfig, MergeWarning};

const EXAMPLES: &str = "
Examples:
compiling a single configuration:

    confpiler build myconfig.yaml


compiling multiple configurations into one:

    confpiler build myconfig.yaml myotherconfig.yaml


compiling configurations from a directory
given:
   mydir/
       default.json
       production.json
       staging.json

compile into a configuration made up of default.json + production.json:

    confpiler build mydir --env production


getting the output as json:

    confpiler build mydir --env production --json
";

/// A configuration compiler and exporter use --help (long help) for more
///
/// This utility "compiles" one or many configuration files into a single,
/// flattened mapping of strings suitable for exporting as environment
/// variables. It is intended to allow for defining configuration variables
/// in a more human-manageable form by then exporting those variables in a way
/// your standard "twelve-factor" app would consume.
///
/// JSON, TOML, YAML, and INI are supported formats, and, while not recommended,
/// you can mix and match.
///
/// This DOES NOT support array values of complex types (like other arrays or
/// dictionaries), as those do not translate well to environment variables
/// without some additional encoding. Arrays of simple types are joined by a
/// separator into a single string.
///
#[derive(Parser)]
#[clap(
    name = "confpiler",
    author,
    version,
    mut_arg("help", |a| a.help("Print the short (-h) or long (--help) help message")),
    after_long_help = EXAMPLES
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: TopLevel,
}

#[derive(Subcommand)]
pub enum TopLevel {
    /// Compiles a configuration to stdout
    Build(BuildArgs),
    /// Checks if a configuration would be valid and exits nonzero if not
    Check(CheckArgs),
}

#[derive(Args)]
pub struct BuildArgs {
    #[clap(flatten)]
    pub common: CommonConfigArgs,

    /// Output as json instead of .env format
    #[clap(short, long)]
    pub json: bool,

    /// Disable sorting for .env-style output (json is always unsorted)
    #[clap(short = 'N', long = "no-sort")]
    pub no_sort: bool,

    /// Perform no quoting or escaping. Does not work with --json
    #[clap(short, long, conflicts_with = "json")]
    pub raw: bool,
}

#[derive(Args)]
pub struct CheckArgs {
    #[clap(flatten)]
    pub common: CommonConfigArgs,
}

#[derive(Args)]
pub struct CommonConfigArgs {
    /// Path(s) to load configuration from
    ///
    /// The order of paths specified determines the order of compilation for
    /// the configuration with the following rules:
    ///
    /// 1) If PATH is a an explicit file, this file is always loaded.
    ///
    /// 2) If PATH is a directory, the DEFAULT is loaded followed by the config
    /// corresponding to the specified ENVIRONMENT, if any. If no environment
    /// is set, ONLY the default is loaded.
    ///
    /// This means that if specifying two or more directories, a directory is
    /// processed completely before moving on to the next directory, meaning
    /// that configurations specified in a subsequent directory will ALWAYS
    /// take precedence.
    #[clap(required = true, parse(from_os_str))]
    pub path: Vec<PathBuf>,

    /// The environment to compile (has no effect unless specifying a directory).
    ///
    /// If no corresponding file exists, it is ignored.
    #[clap(short, long = "env")]
    pub environment: Option<String>,

    /// Basename of file(s) to consider default when operating on directories
    #[clap(short, long, default_value = "default")]
    pub default: String,

    /// A prefix to prepend to all generated keys.
    ///
    /// This value will always be converted to uppercase.
    #[clap(short, long)]
    pub prefix: Option<String>,

    /// The separator to use when flattening keys from config files
    #[clap(short, long, default_value = "__")]
    pub separator: String,

    /// The separator to use when flattening keys from config files
    #[clap(short = 'a', long, default_value = ",")]
    pub array_separator: String,

    /// Error on warnings
    #[clap(long)]
    pub strict: bool,
}

impl CommonConfigArgs {
    pub fn try_make_config(&self) -> Result<(FlatConfig, Vec<MergeWarning>)> {
        let mut builder = FlatConfig::builder();
        builder.with_separator(&self.separator);
        builder.with_array_separator(&self.array_separator);

        if let Some(ref prefix) = self.prefix {
            builder.with_prefix(prefix);
        }

        for p in self.path.iter() {
            let path = p.as_path();

            if !path.exists() {
                bail!("Path '{}' does not exist", path.display());
            }

            // we have to add two sources: the "default" and the "env", if it
            // exists
            if path.is_dir() {
                let def = path.join(&self.default);
                let def_str = def
                    .to_str()
                    .ok_or_else(|| anyhow!("Path does not contain valid characters"))?;
                builder.add_config(def_str);

                if let Some(ref environment) = self.environment {
                    let env = path.join(environment);

                    // we allow either specifying the full filename or just the
                    // stem as env
                    if env.exists() || check_stem_exists(path, environment)? {
                        let env_str = env
                            .to_str()
                            .ok_or_else(|| anyhow!("Path does not contain valid characters"))?;

                        builder.add_config(env_str);
                    }
                }
            } else {
                builder.add_config(
                    path.to_str()
                        .ok_or_else(|| anyhow!("Path does not contain valid characters"))?,
                );
            }
        }

        Ok(builder.build()?)
    }
}

fn check_stem_exists(path: &Path, desired: &str) -> Result<bool> {
    if path.is_dir() {
        Ok(path
            .read_dir()
            .with_context(|| format!("Failed to read {}", path.display()))?
            .any(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(stem) = path.file_stem() {
                            return stem.to_str() == Some(desired);
                        }
                    }
                }
                false
            }))
    } else {
        Ok(false)
    }
}
