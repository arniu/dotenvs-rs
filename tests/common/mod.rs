use std::{env, fs, io};
use tempfile::{tempdir, TempDir};

pub trait TempDirExt {
    fn ensure_current_dir(&self, path: &str);
}

impl TempDirExt for TempDir {
    fn ensure_current_dir(&self, path: &str) {
        let dirs = self.path().join(path);
        fs::create_dir_all(&dirs)
            .and(env::set_current_dir(&dirs))
            .unwrap()
    }
}

fn prepare_test_dotenv(contents: &str) -> io::Result<TempDir> {
    let dir = tempdir()?;
    env::set_current_dir(dir.path())?;
    fs::write(dir.path().join(".env"), contents)?;
    Ok(dir)
}

pub fn with_sample<F: Fn(TempDir)>(f: F) {
    with_dotenv(include_str!("sample.env"), f);
}

pub fn with_dotenv<F: Fn(TempDir)>(contents: &str, f: F) {
    prepare_test_dotenv(contents).map(f).unwrap();
}
