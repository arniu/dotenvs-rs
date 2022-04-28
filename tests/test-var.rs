mod common;

use common::*;
use dotenv::*;

#[test]
fn test() {
    with_sample(|_| {
        assert!(std::env::var("TEST_KEY").is_err());
        assert_eq!(var("TEST_KEY").unwrap(), "test_val");
        assert_eq!(var("TEST_KEY").unwrap(), std::env::var("TEST_KEY").unwrap());
    });
}
