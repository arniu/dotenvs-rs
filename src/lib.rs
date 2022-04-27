//! This crate provides a configuration loader in the style of the [ruby dotenv
//! gem](https://github.com/bkeepers/dotenv). This library is meant to be used
//! on development or testing environments in which setting environment
//! variables is not practical. It loads environment variables from a .env
//! file, if available, and mashes those with the actual environment variables
//! provided by the operating system.

mod dotenv;
mod error;
mod find;

use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Once;

pub use crate::dotenv::Dotenv;
pub use crate::error::*;

static START: Once = Once::new();

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
    START.call_once(|| {
        dotenv().ok();
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
    START.call_once(|| {
        dotenv().ok();
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
/// dotenv::from_path(my_path.as_path());
/// ```
pub fn from_path<P: AsRef<Path>>(path: P) -> Result<()> {
    from_path_iter(path).and_then(Dotenv::load)
}

/// Like `from_path`, but returns an iterator over variables instead of loading into environment.
///
/// Examples
///
/// ```no_run
/// use std::env;
///
/// let my_path = env::home_dir().map(|dir| dir.join(".env")).unwrap();
/// for item in dotenv::from_path_iter(my_path.as_path()).unwrap() {
///   let (key, val) = item.unwrap();
///   println!("{}={}", key, val);
/// }
/// ```
pub fn from_path_iter<P: AsRef<Path>>(path: P) -> Result<Dotenv> {
    Ok(Dotenv::new(File::open(path)?))
}

/// Loads the specified file from the environment's current directory or its parents in sequence.
///
/// # Examples
///
/// ```
/// dotenv::from_filename(".env.local").ok();
/// ```
pub fn from_filename<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    let path = find::find(filename.as_ref())?;
    from_path_iter(&path)?.load()?;

    Ok(path)
}

/// Like `from_filename`, but returns an iterator over variables instead of loading into environment.
///
/// # Examples
///
/// ```no_run
/// let iter = dotenv::from_filename_iter(".env.local").unwrap();
///
/// for item in iter {
///   let (key, val) = item.unwrap();
///   println!("{}={}", key, val);
/// }
/// ```
pub fn from_filename_iter<P: AsRef<Path>>(filename: P) -> Result<Dotenv> {
    find::find(filename.as_ref()).and_then(from_path_iter)
}

/// This is usually what you want.
/// It loads the .env file located in the environment's current directory or its parents in sequence.
///
/// # Examples
/// ```
/// use dotenv;
/// dotenv::dotenv().ok();
/// ```
pub fn dotenv() -> Result<PathBuf> {
    from_filename(".env")
}

/// Like `dotenv`, but returns an iterator over variables instead of loading into environment.
///
/// # Examples
/// ```no_run
/// use dotenv;
///
/// for item in dotenv::dotenv_iter().unwrap() {
///   let (key, val) = item.unwrap();
///   println!("{}={}", key, val);
/// }
/// ```
pub fn dotenv_iter() -> Result<Dotenv> {
    from_filename_iter(".env")
}
