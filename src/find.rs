use std::{
    env, io,
    path::{Path, PathBuf},
};

use crate::error::*;

fn not_found() -> Error {
    io::Error::new(io::ErrorKind::NotFound, "path not found").into()
}

/// Searches for `filename` in current dir and its ancestors
pub fn find(filename: &Path) -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    current_dir
        .ancestors()
        .map(|dir| dir.join(filename))
        .find(|path| path.is_file())
        .ok_or_else(not_found)
}

/// Searches for `.env` in current dir and its ancestors
pub fn find_dotenv() -> Result<PathBuf> {
    find(".env".as_ref())
}
