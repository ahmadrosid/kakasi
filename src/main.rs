mod cli;
mod commands;
mod util;

use crate::cli::{CliResult, Config, Shell, parse, print_help};

fn main() -> CliResult {
    let mut config = match Config::default() {
        Ok(cfg) => cfg,
        Err(e) => {
            let mut shell = Shell::new();
            cli::exit_with_error(e.into(), &mut shell)
        }
    };

    let args = match parse().try_get_matches() {
        Ok(args) => args,
        Err(e) => {
            let mut shell = Shell::new();
            cli::exit_with_error(e.into(), &mut shell)
        }
    };

    if let Some((cmd, args)) = args.subcommand() {
        if let Some(exec) = commands::builtin_exec(cmd) {
            match exec(&mut config, args) {
                Ok(()) => {},
                Err(e) => {
                    let mut shell = Shell::new();
                    cli::exit_with_error(e.into(), &mut shell)
                }
            }
        }
    } else {
        print_help();
    }

    Ok(())
}
