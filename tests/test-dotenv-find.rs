mod fixtures;
use fixtures::*;

use std::{env, fs};

#[test]
fn test_find() -> anyhow::Result<()> {
    let _t = with_basic_dotenv()?;

    fs::create_dir("child")?;
    env::set_current_dir("child")?;

    assert!(env::var("BASIC").is_err());
    assert_eq!(dotenv::var("BASIC")?, "basic");

    Ok(())
}
