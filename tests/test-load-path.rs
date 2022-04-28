mod common;

use common::*;
use dotenv::*;

#[test]
fn test() {
    with_sample(|t| {
        assert!(std::env::var("TEST_KEY").is_err());

        let path = t.path().join(".env");
        load_path(&path).expect("set env variables");
        assert_eq!(std::env::var("TEST_KEY").unwrap(), "test_val");
    });
}
