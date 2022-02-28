use anyhow::{Result, Context};
use clap::Parser;
use cli::{Cli, TopLevel, CommonConfigArgs};
use confpiler::{error::ConfpilerError, FlatConfig, MergeWarning};

mod cli;


fn main() -> Result<()> {
    let args = Cli::parse();
    match &args.command {
        TopLevel::Build(build_args) => {
            let (conf, _) = get_config(&build_args.common)?;

            if build_args.json {
                println!("{}", serde_json::to_string(conf.items())?);
            } else {
                // I guess we could import itertools for the sorting, but eh
                let mut items = conf.items().iter().collect::<Vec<_>>();

                if !build_args.no_sort {
                    items.sort_by(|a, b| a.0.cmp(b.0));
                }

                for (k, v) in items {
                    println!("{}=\"{}\"", k, v);
                }
            }
        },
        TopLevel::Check(check_args) => {
            println!("Checking configuration...");
            let (_, warnings) = get_config(&check_args.common)?;

            if !warnings.is_empty() {
                // just print the warnings here, since we handled the strict
                // case in get_config
                println!("Warnings:");
                println!("{}", warnings_formatter(&warnings));
            }

            println!("\nok")
        },
    }

    Ok(())
}

fn get_config(args: &CommonConfigArgs) -> Result<(FlatConfig, Vec<MergeWarning>)> {
    let (conf, warnings) = args
        .try_make_config()
        .context("Configuration as specified is not valid")?;

    if !warnings.is_empty() && args.strict {
        // we turn the warnings into an error
        Err(ConfpilerError::from(warnings))
            .context("Configuration is not valid when treating warnings as errors")?
    } else {
        // we need to do this in an "else" block to avoid moving warnings in a
        // way the compiler can't deal with
        Ok((conf, warnings))
    }

}

// so doing this sort here is a little weird, but we already have sorted output
// if it's an error
//
// // TODO: maybe look at removing the duplication - MCL - 2022-02-27
fn warnings_formatter(warnings: &[MergeWarning]) -> String {
    let mut out = warnings.iter().map(|w| format!("    {}", w)).collect::<Vec<_>>();
    out.sort();
    out.join("\n")
}
