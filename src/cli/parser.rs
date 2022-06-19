use crate::cli::*;
use crate::commands;

pub fn parse() -> App {
    let usage = "kakasi [OPTIONS] [SUBCOMMAND]";
    App::new("kakasi")
        .allow_external_subcommands(true)
        .setting(AppSettings::DeriveDisplayOrder)
        .disable_colored_help(false)
        .override_usage(usage)
        .help_template(get_help_text())
        .arg(flag("version", "Print version info and exit").short('V'))
        .arg(flag("help", "List command"))
        .subcommands(commands::builtin())
}

pub fn print_help() {
    println!(
        "{}",
        get_help_text()
            .replace("{usage}", "kakasi [OPTIONS] [SUBCOMMAND]")
            .replace("{options}", "\t--help")
    );
}

fn get_help_text() -> &'static str {
    "\
Kakasi fullstack web framework v0.1
USAGE:
    {usage}
OPTIONS:
{options}
Some common kakasi commands are (see all commands with --list):
    new           Create a new project
    controller    Create a new controller in an existing directory
    model         Create a model layer to your database
    serve         Run local development server
See 'kakasi help <command>' for more information on a specific command.\n"
}
