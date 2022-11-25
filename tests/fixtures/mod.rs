#![allow(dead_code)]

use std::fs;
use std::io;
use std::io::prelude::*;
use tempfile::{tempdir, TempDir};

type Exps = std::collections::HashMap<String, String>;

fn make_test(dotenv: &str, expect: &str) -> io::Result<(TempDir, Exps)> {
    let cwd = tempdir()?;
    std::env::set_current_dir(cwd.path())?;

    let dotenv_path = cwd.path().join(".env");
    let mut dotenv_file = fs::File::create(dotenv_path)?;
    dotenv_file.write_all(dotenv.as_bytes())?;
    dotenv_file.sync_all()?;

    Ok((cwd, serde_json::from_str(expect)?))
}

pub fn with_basic_dotenv() -> io::Result<(TempDir, Exps)> {
    make_test(
        include_str!("sample-basic.env"),
        include_str!("sample-basic.json"),
    )
}

pub fn with_expand_dotenv() -> io::Result<(TempDir, Exps)> {
    make_test(
        include_str!("sample-expand.env"),
        include_str!("sample-expand.json"),
    )
}

pub fn with_multiline_dotenv() -> io::Result<(TempDir, Exps)> {
    make_test(
        include_str!("sample-multiline.env"),
        include_str!("sample-multiline.json"),
    )
}
