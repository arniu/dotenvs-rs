#![allow(dead_code)]

use std::fs;
use std::io;
use std::io::prelude::*;
use tempfile::{tempdir, TempDir};

pub fn make_test_dotenv() -> io::Result<TempDir> {
    make_dotenv("TEST_KEY=test_val")
}

pub fn make_basic_dotenv() -> io::Result<TempDir> {
    make_dotenv(include_str!("sample-basic.env"))
}

pub fn make_expand_dotenv() -> io::Result<TempDir> {
    make_dotenv(include_str!("sample-expand.env"))
}

pub fn make_multiline_dotenv() -> io::Result<TempDir> {
    make_dotenv(include_str!("sample-multiline.env"))
}

fn make_dotenv(dotenv_text: &str) -> io::Result<TempDir> {
    let dir = tempdir()?;
    std::env::set_current_dir(dir.path())?;

    let dotenv_path = dir.path().join(".env");
    let mut dotenv_file = fs::File::create(dotenv_path)?;
    dotenv_file.write_all(dotenv_text.as_bytes())?;
    dotenv_file.sync_all()?;

    Ok(dir)
}
