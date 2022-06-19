mod commands;
mod cli;

use crate::cli::*;

fn main() -> CliResult {
    let mut config = Config{};
    let args = match cli::parse().try_get_matches() {
        Ok(args) => args,
        Err(e) => {
            return Err(e.into());
        }
    };

    if let Some((cmd, args)) = args.subcommand() {
        if let Some(cm) = commands::builtin_exec(cmd) {
            let _ = cm(&mut config, args);
        }
    } else {
        cli::print_help();
    }

    Ok(())
}
