mod parser;

pub use clap::{value_parser, AppSettings, Arg, ArgAction, ArgMatches};
pub type App = clap::Command<'static>;

pub use parser::parse;
pub use parser::print_help;

#[derive(Debug)]
pub struct Config {
}

pub fn subcommand(name: &'static str) -> App {
    App::new(name)
        .dont_collapse_args_in_usage(true)
        .setting(AppSettings::DeriveDisplayOrder)
}

pub trait AppExt: Sized {
    fn _arg(self, arg: Arg<'static>) -> Self;

    fn arg_new_opts(self) -> Self {
        self
    }
    fn arg_quiet(self) -> Self {
        self._arg(flag("quiet", "Do not print log messages").short('q'))
    }
}

impl AppExt for App {
    fn _arg(self, arg: Arg<'static>) -> Self {
        self.arg(arg)
    }
}

pub fn flag(name: &'static str, help: &'static str) -> Arg<'static> {
    Arg::new(name)
        .long(name)
        .help(help)
        .action(ArgAction::SetTrue)
}

pub fn opt(name: &'static str, help: &'static str) -> Arg<'static> {
    Arg::new(name).long(name).help(help)
}

pub type CliResult = Result<(), CliError>;

#[derive(Debug)]
pub struct CliError {
    pub error: Option<anyhow::Error>,
    pub exit_code: i32,
}

impl CliError {
    pub fn new(error: anyhow::Error, code: i32) -> CliError {
        CliError {
            error: Some(error),
            exit_code: code,
        }
    }
}

impl From<anyhow::Error> for CliError {
    fn from(err: anyhow::Error) -> CliError {
        CliError::new(err, 101)
    }
}

impl From<clap::Error> for CliError {
    fn from(err: clap::Error) -> CliError {
        let code = if err.use_stderr() { 1 } else { 0 };
        CliError::new(err.into(), code)
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> CliError {
        CliError::new(err.into(), 1)
    }
}

