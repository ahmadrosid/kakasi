use std::fmt;
use crate::cli::Shell;
use crate::cli::shell::Verbosity::Verbose;

use log::debug;
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

pub struct InternalError {
    inner: anyhow::Error,
}

impl InternalError {
    pub fn new(inner: anyhow::Error) -> InternalError {
        InternalError { inner }
    }
}

impl std::error::Error for InternalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}

impl fmt::Debug for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

pub fn internal<S: fmt::Display>(error: S) -> anyhow::Error {
    InternalError::new(anyhow::format_err!("{}", error)).into()
}


pub struct VerboseError {
    inner: anyhow::Error,
}

impl VerboseError {
    pub fn new(inner: anyhow::Error) -> VerboseError {
        VerboseError { inner }
    }
}

impl std::error::Error for VerboseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}

impl fmt::Debug for VerboseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Display for VerboseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}


pub struct AlreadyPrintedError {
    inner: anyhow::Error,
}

impl AlreadyPrintedError {
    pub fn new(inner: anyhow::Error) -> Self {
        AlreadyPrintedError { inner }
    }
}

impl std::error::Error for AlreadyPrintedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}

impl fmt::Debug for AlreadyPrintedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Display for AlreadyPrintedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

pub fn exit_with_error(err: CliError, shell: &mut Shell) -> ! {
    debug!("exit_with_error; err={:?}", err);

    if let Some(ref err) = err.error {
        if let Some(clap_err) = err.downcast_ref::<clap::Error>() {
            let exit_code = if clap_err.use_stderr() { 1 } else { 0 };
            let _ = clap_err.print();
            std::process::exit(exit_code)
        }
    }

    let CliError { error, exit_code } = err;
    if let Some(error) = error {
        display_error(&error, shell);
    }

    std::process::exit(exit_code)
}

/// Displays an error, and all its causes, to stderr.
pub fn display_error(err: &anyhow::Error, shell: &mut Shell) {
    debug!("display_error; err={:?}", err);
    _display_error(err, shell, true);
    if err
        .chain()
        .any(|e| e.downcast_ref::<InternalError>().is_some())
    {
        drop(shell.note("this is an unexpected cargo internal error"));
        drop(
            shell.note(
                "we would appreciate a bug report: https://github.com/ahmadrosid/kakasi/issues/",
            ),
        );
        drop(shell.note(format!("kakasi {}", "v0.1")));
        // Once backtraces are stabilized, this should print out a backtrace
        // if it is available.
    }
}

fn _display_error(err: &anyhow::Error, shell: &mut Shell, as_err: bool) -> bool {
    for (i, err) in err.chain().enumerate() {
        // If we're not in verbose mode then only print cause chain until one
        // marked as `VerboseError` appears.
        //
        // Generally the top error shouldn't be verbose, but check it anyways.
        if shell.verbosity() != Verbose && err.is::<VerboseError>() {
            return true;
        }
        if err.is::<AlreadyPrintedError>() {
            break;
        }
        if i == 0 {
            if as_err {
                drop(shell.error(&err));
            } else {
                drop(writeln!(shell.err(), "{}", err));
            }
        } else {
            drop(writeln!(shell.err(), "\nCaused by:"));
            drop(write!(shell.err(), "{}", indented_lines(&err.to_string())));
        }
    }
    false
}


pub fn indented_lines(text: &str) -> String {
    text.lines()
        .map(|line| {
            if line.is_empty() {
                String::from("\n")
            } else {
                format!("  {}\n", line)
            }
        })
        .collect()
}
