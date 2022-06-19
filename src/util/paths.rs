use anyhow::{Context, Result};
use std::fs::{self, File, OpenOptions};
use std::path::{Component, Path, PathBuf};

pub fn get_template_file(name: &str) -> String {
    let main = include_str!("_template_stubs/main.rs.stub");

    match name {
        "main.rs.stub" => main.to_string(),
        _ => String::new()
    }
}

/// Writes a file to disk.
///
/// Equivalent to [`std::fs::write`] with better error messages.
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()> {
    let path = path.as_ref();
    fs::write(path, contents.as_ref())
        .with_context(|| format!("failed to write `{}`", path.display()))
}

/// Equivalent to [`std::fs::create_dir_all`] with better error messages.
pub fn create_dir_all(p: impl AsRef<Path>) -> Result<()> {
    _create_dir_all(p.as_ref())
}

fn _create_dir_all(p: &Path) -> Result<()> {
    fs::create_dir_all(p)
        .with_context(|| format!("failed to create directory `{}`", p.display()))?;
    Ok(())
}
