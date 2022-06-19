use super::cli::*;
pub mod new;

pub fn builtin() -> Vec<App> {
    vec![
        new::cli()
    ]
}

pub fn builtin_exec(cmd: &str) -> Option<fn(&mut Config, &ArgMatches) -> CliResult> {
    let f = match cmd {
        "new" => new::exec,
        _ => return None,
    };
    Some(f)
}