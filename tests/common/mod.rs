use std::{env, fs, io};
use tempfile::TempDir;

pub trait TempDirExt {
    fn ensure_current_dir(&self, path: &str);
}

impl TempDirExt for TempDir {
    fn ensure_current_dir(&self, path: &str) {
        let path = self.path().join(path);
        fs::create_dir_all(&path)
            .and(env::set_current_dir(&path))
            .unwrap()
    }
}

fn prepare_test_dotenv(contents: &str, path: &str) -> io::Result<TempDir> {
    tempfile::tempdir().and_then(|dir| {
        env::set_current_dir(dir.path())
            .and(fs::write(dir.path().join(path), contents))
            .and(Ok(dir))
    })
}

pub fn with_dotenv<F: Fn(TempDir)>(contents: &str, f: F) {
    prepare_test_dotenv(contents, ".env").map(f).unwrap();
}

pub fn with_sample<F: Fn(TempDir)>(f: F) {
    with_dotenv(include_str!("sample.env"), f);
}
