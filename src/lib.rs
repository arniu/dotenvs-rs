//! Load and use environment variables from `.env` files.
//!
//! # Overview
//!
//! This crate parses `.env` files with variable substitution support:
//! `$VAR`, `${VAR}`, `${VAR:-default}`. It provides both convenience
//! functions (for typical usage) and a builder-style [`Dotenv`] type
//! (for fine-grained control).
//!
//! # Quick start
//!
//! ```no_run
//! dotenv::load().ok();
//!
//! for (key, value) in dotenv::vars() {
//!     println!("{key}={value}");
//! }
//! ```
//!
//! # Available entry points
//!
//! | Task | Function / Type |
//! |---|---|
//! | Load `.env` automatically | [`load()`], [`var()`], [`vars()`] |
//! | Load a specific file | [`from_path()`], [`from_filename()`] |
//! | Load from a custom source | [`from_read()`] |
//! | Builder with control over behaviour | [`Dotenv`] |

mod dotenv;
mod error;
mod parse;

pub use crate::dotenv::Dotenv;
pub use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

use std::env;
use std::ffi;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Once;

static LOAD: Once = Once::new();

/// Get the value for an environment variable.
///
/// Automatically loads `.env` on first call (if present and not yet loaded).
/// Values are read from the **current process environment**, not directly from
/// the `.env` file — so any value set by a previous caller or the shell is visible.
///
/// # Errors
///
/// Returns [`Error::Env`] if the variable contains non-unicode data.
///
/// # Examples
///
/// ```no_run
/// let value = dotenv::var("HOME").unwrap();
/// println!("{}", value);
/// ```
pub fn var<K: AsRef<ffi::OsStr>>(key: K) -> Result<String> {
    LOAD.call_once(|| {
        load().ok();
    });

    env::var(key).map_err(Error::from)
}

/// Return an iterator of `(key, value)` pairs for all environment variables
/// of the current process.
///
/// Automatically loads `.env` on first call. The returned iterator is a
/// snapshot of the process environment at the time of invocation —
/// subsequent modifications are not reflected.
///
/// # Examples
///
/// ```no_run
/// use std::io;
///
/// for (key, value) in dotenv::vars() {
///     println!("{key}={value}");
/// }
/// ```
pub fn vars() -> env::Vars {
    LOAD.call_once(|| {
        load().ok();
    });

    env::vars()
}

fn find<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    let filename = filename.as_ref();
    env::current_dir()?
        .ancestors()
        .map(|dir| dir.join(filename))
        .find(|path| path.is_file())
        .ok_or_else(Error::not_found)
}

/// Load the `.env` file from the current directory or its parents.
///
/// Searches upward from the current working directory until `.env` is found.
/// The first call loads variables; subsequent calls are no-ops through
/// [`Dotenv::load`] (existing variables are preserved).
///
/// # Errors
///
/// Returns [`Error::Io`] if no `.env` file is found or it cannot be read.
///
/// # Examples
///
/// ```no_run
/// dotenv::load().ok();
/// ```
pub fn load() -> Result<PathBuf> {
    let path = find(".env")?;
    from_path(&path).map(|vars| {
        vars.load();
        path
    })
}

/// Create [`Dotenv`] from the specified file.
///
/// Searches upward from the current working directory for the given filename.
///
/// # Errors
///
/// Returns [`Error::Io`] if the file is not found or cannot be read.
pub fn from_filename<P: AsRef<Path>>(filename: P) -> Result<Dotenv> {
    let path = find(filename)?;
    from_path(path)
}

/// Create [`Dotenv`] from the specified path.
///
/// # Errors
///
/// Returns [`Error::Io`] if the file cannot be read.
pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Dotenv> {
    let file = fs::File::open(path)?;
    from_read(file)
}

/// Create [`Dotenv`] from any [`Read`] implementor.
///
/// This is useful for loading environment variables from in-memory buffers,
/// IPC streams, or network connections.
///
/// # Errors
///
/// Returns [`Error::Io`] if reading from the source fails.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
///
/// let input = Cursor::new(b"FOO=bar\nBAZ=qux\n");
/// let dotenv = dotenv::from_read(input).unwrap();
/// for (key, value) in dotenv.iter() {
///     println!("{key}={value}");
/// }
/// ```
pub fn from_read<R: Read>(read: R) -> Result<Dotenv> {
    let mut buf = String::new();
    let mut reader = io::BufReader::new(read);
    reader.read_to_string(&mut buf)?;

    Ok(Dotenv::new(buf))
}
