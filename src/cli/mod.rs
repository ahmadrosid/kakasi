mod parser;
mod shell;
mod errors;

use anyhow::{anyhow, bail, format_err, Context as _, Error};
use std::cell::{RefCell, RefMut};
use std::env;
use std::path::{Path, PathBuf};
use std::fmt;

pub use clap::{value_parser, AppSettings, Arg, ArgAction, ArgMatches};
pub use parser::parse;
pub use parser::print_help;
pub use shell::Shell;
pub use errors::*;

pub type App = clap::Command<'static>;

#[derive(Debug)]
pub struct Config {
    shell: RefCell<Shell>,
    cwd: PathBuf,
}

impl Config {
    pub fn new(shell: Shell, cwd: PathBuf) -> Self {
        Self {
            shell: RefCell::new(shell),
            cwd,
        }
    }

    pub fn default() -> anyhow::Result<Config> {
        let shell = Shell::new();
        let cwd = env::current_dir()
            .with_context(|| "couldn't get the current directory of the process")?;
        Ok(Config::new(shell, cwd))
    }

    pub fn shell(&self) -> RefMut<'_, Shell> {
        self.shell.borrow_mut()
    }

    /// The current working directory.
    pub fn cwd(&self) -> &Path {
        &self.cwd
    }
}

pub fn subcommand(name: &'static str) -> App {
    App::new(name)
        .dont_collapse_args_in_usage(true)
        .setting(AppSettings::DeriveDisplayOrder)
}

pub trait AppExt: Sized {
    fn _arg(self, arg: Arg<'static>) -> Self;

    fn arg_new_opts(self) -> Self {
        self._arg(
            opt(
                "name",
                "Set the resulting package name, defaults to the directory name",
            )
            .value_name("NAME"),
        )
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
pub struct NewOptions {
    /// Absolute path to the directory for the new package
    pub path: PathBuf,
    pub name: Option<String>,
}

impl NewOptions {
    pub fn new(path: PathBuf, name: Option<String>) -> anyhow::Result<NewOptions> {
        let opts = NewOptions { path, name };
        Ok(opts)
    }
}

pub trait ArgMatchesExt {
    fn new_options(&self, config: &Config) -> anyhow::Result<NewOptions> {
        NewOptions::new(
            self.value_of_path("path", config).unwrap(),
            self._value_of("name").map(|s| s.to_string()),
        )
    }

    fn flag(&self, name: &str) -> bool;

    fn _value_of(&self, name: &str) -> Option<&str>;

    fn value_of_path(&self, name: &str, config: &Config) -> Option<PathBuf>;
}

impl<'a> ArgMatchesExt for ArgMatches {
    fn flag(&self, name: &str) -> bool {
        ignore_unknown(self.try_get_one::<bool>(name))
            .copied()
            .unwrap_or(false)
    }

    fn _value_of(&self, name: &str) -> Option<&str> {
        ignore_unknown(self.try_get_one::<String>(name)).map(String::as_str)
    }

    /// Returns value of the `name` command-line argument as an absolute path
    fn value_of_path(&self, name: &str, config: &Config) -> Option<PathBuf> {
        self._value_of(name).map(|path| config.cwd().join(path))
    }
}

#[track_caller]
fn ignore_unknown<T: Default>(r: Result<T, clap::parser::MatchesError>) -> T {
    match r {
        Ok(t) => t,
        Err(clap::parser::MatchesError::UnknownArgument { .. }) => Default::default(),
        Err(e) => {
            panic!("Mismatch between definition and access: {}", e);
        }
    }
}
