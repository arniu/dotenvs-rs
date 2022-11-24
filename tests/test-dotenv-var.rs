mod fixtures;
use fixtures::*;

use std::env;

#[test]
fn test_var() -> anyhow::Result<()> {
    let _cwd = make_test_dotenv()?;

    assert!(env::var("TEST_KEY").is_err());
    assert_eq!(dotenv::var("TEST_KEY")?, "test_val");

    Ok(())
}
