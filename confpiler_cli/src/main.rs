use anyhow::Result;
use clap::Parser;
use cli::{Cli, TopLevel};
use confpiler::MergeWarning;
use snailquote::escape;

mod cli;

fn main() -> Result<()> {
    let args = Cli::parse();
    match &args.command {
        TopLevel::Build(build_args) => {
            let (conf, _) = build_args.common.get_config()?;

            if build_args.json {
                println!("{}", serde_json::to_string(conf.items())?);
            } else {
                // I guess we could import itertools for the sorting, but eh
                let mut items = conf.items().iter().collect::<Vec<_>>();

                if !build_args.no_sort {
                    items.sort_by(|a, b| a.0.cmp(b.0));
                }

                for (k, v) in items {
                    if build_args.raw {
                        println!("{}={}", k, v);
                        continue;
                    }

                    // So the behavior of the library being used to do the
                    // escaping automatically ads single/double quotes if
                    // necessary, and does not otherwise. So we have to do this
                    // ugly hack to work around that
                    //
                    // TODO: the current escaping behavior is not exactly what
                    // we want because we want to allow for $var substitutions
                    // to be defined but we currently are going to add single
                    // quotes. We might have to implement our own escaping
                    // for this... - MCL - 2022-03-03
                    let escaped = escape(v);
                    if v.as_str() != escaped {
                        println!("{}={}", k, escaped);
                    } else {
                        println!("{}=\"{}\"", k, v);
                    }
                }
            }
        }
        TopLevel::Check(check_args) => {
            println!("Checking configuration...");
            let (_, warnings) = check_args.common.get_config()?;

            if !warnings.is_empty() {
                // just print the warnings here, since we handled the strict
                // case in get_config
                println!("Warnings:");
                println!("{}", warnings_formatter(&warnings));
            }

            println!("\nok")
        }
        TopLevel::Update(update_args) => {
            update_args.update()?;
        }
    }

    Ok(())
}

// so doing this sort here is a little weird, but we already have sorted output
// if it's an error
//
// // TODO: maybe look at removing the duplication - MCL - 2022-02-27
fn warnings_formatter(warnings: &[MergeWarning]) -> String {
    let mut out = warnings
        .iter()
        .map(|w| format!("    {}", w))
        .collect::<Vec<_>>();
    out.sort();
    out.join("\n")
}
