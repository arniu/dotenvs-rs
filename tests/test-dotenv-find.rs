mod fixtures;
use fixtures::*;

use std::{env, fs};

#[test]
fn test_find() -> anyhow::Result<()> {
    let _cwd = make_test_dotenv()?;

    fs::create_dir("child")?;
    env::set_current_dir("child")?;

    assert!(env::var("TEST_KEY").is_err());
    assert_eq!(dotenv::var("TEST_KEY")?, "test_val");

    Ok(())
}
