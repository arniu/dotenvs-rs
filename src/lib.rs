mod dotenv;
mod errors;
mod parse;

pub use crate::dotenv::Dotenv;
pub use crate::errors::Error;
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
/// The value is `Ok(s)` if the environment variable is present and valid unicode.
///
/// Note: this function gets values from any visible environment variable key,
/// regardless of whether a *.env* file was loaded.
///
/// # Examples:
///
/// ```no_run
/// let value = dotenv::var("HOME").unwrap();
/// println!("{}", value);  // prints `/home/foo`
/// ```
pub fn var<K: AsRef<ffi::OsStr>>(key: K) -> Result<String> {
    LOAD.call_once(|| {
        load().ok();
    });

    env::var(key).map_err(Error::from)
}

/// Return an iterator of `(key, value)` pairs for all environment variables of the current process.
/// The returned iterator contains a snapshot of the process's environment variables at the time of invocation. Modifications to environment variables afterwards will not be reflected.
///
/// # Examples:
///
/// ```no_run
/// use std::io;
///
/// let result: Vec<(String, String)> = dotenv::vars().collect();
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

/// Load the *.env* file from the current directory or its parents.
///
/// Fails if the file is not found.
pub fn load() -> Result<PathBuf> {
    let path = find(".env")?;
    from_path(&path).map(|vars| {
        vars.load();
        path
    })
}

/// Create [Dotenv] from the specified file.
///
/// Fails if the file is not found.
pub fn from_filename<P: AsRef<Path>>(filename: P) -> Result<Dotenv> {
    let path = find(filename)?;
    from_path(path)
}

/// Create [Dotenv] from the specified path.
///
/// Fails if the file is not found.
pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Dotenv> {
    let file = fs::File::open(path)?;
    from_read(file)
}

/// Create [Dotenv] from [Read](std::io::Read).
///
/// This is useful for loading environment variables from from IPC or the network.
pub fn from_read<R: Read>(read: R) -> Result<Dotenv> {
    let mut buf = String::new();
    let mut reader = io::BufReader::new(read);
    reader.read_to_string(&mut buf)?;

    Ok(Dotenv::new(buf))
}
