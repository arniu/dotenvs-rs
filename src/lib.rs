//! It loads environment variables from a `.env` file, if available, and mashes
//! those with the environment variables provided by the operating system.

mod dotenv;
mod error;

#[cfg(test)]
mod tests;

use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Once;

pub use crate::dotenv::Dotenv;
pub use crate::error::*;

static LOAD: Once = Once::new();

/// Load the `.env` file and fetch the environment variable key from the current process.
///
/// For more details, please visit [`load`] or [`std::env::var`].
///
/// # NOTE
///
/// - The `.env` file will be loaded once, so it's cheap to call [`var`] again and again.
/// - The error occurs in loading the `.env` file will be ignored.
///
/// # Examples
///
/// ```no_run
/// let var = dotenv::var("FOO").unwrap();
/// ```
pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String> {
    LOAD.call_once(|| {
        load().ok();
    });

    env::var(key).map_err(Error::from)
}

/// Load the `.env` file and return an iterator of (variable, value) pairs of strings,
/// for all the environment variables of the current process.
///
/// The returned iterator contains a snapshot of the process's environment variables at the
/// time of this invocation, modifications to environment variables afterwards will not be
/// reflected in the returned iterator.
///
/// For more details, please visit [`load`] or [`std::env::vars`].
///
/// # Examples
///
/// ```no_run
/// let vars: Vec<(String, String)> = dotenv::vars().collect();
/// ```
pub fn vars() -> env::Vars {
    LOAD.call_once(|| {
        load().ok();
    });

    env::vars()
}

/// Load file at the given `path` and then set environment variables.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// let my_path = env::home_dir().map(|dir| dir.join(".env")).unwrap();
/// dotenv::load_path(my_path.as_path()).ok();
/// ```
pub fn load_path<P: AsRef<Path>>(path: P) -> Result<()> {
    try_from_path(path).and_then(Dotenv::load)
}

/// Create a [`Dotenv`] instance from the given `path`.
///
/// # Examples
///
/// ```no_run
/// use std::env;
///
/// let my_path = env::home_dir().map(|dir| dir.join(".env")).unwrap();
/// for item in dotenv::try_from_path(my_path.as_path()).unwrap() {
///   let (key, val) = item.unwrap();
///   println!("{}={}", key, val);
/// }
/// ```
pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Dotenv> {
    Ok(Dotenv::new(File::open(path)?))
}

/// Load the given `filename` and then set environment variables.
///
/// It will search for the given `filename` in the current directory or its parents in sequence.
///
/// # Examples
///
/// ```
/// dotenv::load_filename(".env.local").ok();
/// ```
pub fn load_filename<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    search(filename.as_ref()).and_then(|path| load_path(&path).and(Ok(path)))
}

/// Create a [`Dotenv`] instance from the given `filename`
///
/// # Examples
///
/// ```no_run
/// let envs = dotenv::try_from_filename(".env.local").unwrap();
///
/// for (key, value) in envs.flatten() {
///   println!("{}={}", key, value);
/// }
/// ```
pub fn try_from_filename<P: AsRef<Path>>(filename: P) -> Result<Dotenv> {
    search(filename.as_ref()).and_then(try_from_path)
}

/// Load `.env` file and then set environment variables.
///
/// It will search for `.env` file in the current directory or its parents in sequence.
///
/// # Examples
///
/// ```
/// dotenv::load().ok();
/// ```
pub fn load() -> Result<PathBuf> {
    load_filename(".env")
}

/// Create a [`Dotenv`] instance from the `.env` file.
///
/// # Examples
///
/// ```no_run
/// for item in dotenv::try_init().unwrap() {
///   let (key, value) = item.unwrap();
///   println!("{}={}", key, value);
/// }
/// ```
pub fn try_init() -> Result<Dotenv> {
    try_from_filename(".env")
}

/// Search for `filename` in the current directory or its parents in sequence.
fn search(filename: &Path) -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    fn path_not_found() -> Error {
        std::io::Error::new(std::io::ErrorKind::NotFound, "path not found").into()
    }

    current_dir
        .ancestors()
        .map(|dir| dir.join(filename))
        .find(|path| path.is_file())
        .ok_or_else(path_not_found)
}
