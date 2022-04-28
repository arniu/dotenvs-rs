//! This crate provides a configuration loader in the style of the [ruby dotenv
//! gem](https://github.com/bkeepers/dotenv). This library is meant to be used
//! on development or testing environments in which setting environment
//! variables is not practical. It loads environment variables from a .env
//! file, if available, and mashes those with the actual environment variables
//! provided by the operating system.

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

/// After loading the dotenv file, fetches the environment variable key from the current process.
///
/// The returned result is Ok(s) if the environment variable is present and is valid unicode. If the
/// environment variable is not present, or it is not valid unicode, then Err will be returned.
///
/// Examples:
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

/// After loading the dotenv file, returns an iterator of (variable, value) pairs of strings,
/// for all the environment variables of the current process.
///
/// The returned iterator contains a snapshot of the process's environment variables at the
/// time of this invocation, modifications to environment variables afterwards will not be
/// reflected in the returned iterator.
///
/// Examples:
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

/// Loads the file at the specified absolute path.
///
/// Examples
///
/// ```
/// use std::env;
///
/// let my_path = env::home_dir().map(|dir| dir.join(".env")).unwrap();
/// dotenv::load_path(my_path.as_path());
/// ```
pub fn load_path<P: AsRef<Path>>(path: P) -> Result<()> {
    try_from_path(path).and_then(Dotenv::load)
}

/// Like `from_path`, but returns an iterator over variables instead of loading into environment.
///
/// Examples
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

/// Loads the specified file from the environment's current directory or its parents in sequence.
///
/// # Examples
///
/// ```
/// dotenv::load_filename(".env.local").ok();
/// ```
pub fn load_filename<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    find(filename.as_ref()).and_then(|path| load_path(&path).and(Ok(path)))
}

/// Like `from_filename`, but returns an iterator over variables instead of loading into environment.
///
/// # Examples
///
/// ```no_run
/// let iter = dotenv::try_from_filename(".env.local").unwrap();
///
/// for item in iter {
///   let (key, val) = item.unwrap();
///   println!("{}={}", key, val);
/// }
/// ```
pub fn try_from_filename<P: AsRef<Path>>(filename: P) -> Result<Dotenv> {
    find(filename.as_ref()).and_then(try_from_path)
}

/// This is usually what you want.
/// It loads the .env file located in the environment's current directory or its parents in sequence.
///
/// # Examples
///
/// ```
/// dotenv::load().ok();
/// ```
pub fn load() -> Result<PathBuf> {
    load_filename(".env")
}

/// Like `dotenv`, but returns an iterator over variables instead of loading into environment.
///
/// # Examples
///
/// ```no_run
/// for item in dotenv::try_init().unwrap() {
///   let (key, val) = item.unwrap();
///   println!("{}={}", key, val);
/// }
/// ```
pub fn try_init() -> Result<Dotenv> {
    try_from_filename(".env")
}

/// Searches for `filename` in current dir and its ancestors
fn find(filename: &Path) -> Result<PathBuf> {
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
