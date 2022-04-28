mod common;

use common::*;
use dotenv::*;

#[test]
fn test() {
    with_sample(|_| {
        let iter = try_from_filename(".env").unwrap();
        assert!(std::env::var("TEST_KEY").is_err());

        iter.load().expect("set env variables");
        assert_eq!(std::env::var("TEST_KEY").unwrap(), "test_val");
    });
}
