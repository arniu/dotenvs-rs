mod fixtures;
use fixtures::*;

use std::env;

#[test]
fn test_var() -> anyhow::Result<()> {
    let _t = with_basic_dotenv()?;

    assert!(env::var("BASIC").is_err());
    assert_eq!(dotenv::var("BASIC")?, "basic");

    Ok(())
}
