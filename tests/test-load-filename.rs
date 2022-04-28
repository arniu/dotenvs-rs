mod common;

use common::*;
use dotenv::*;

#[test]
fn test() {
    with_sample(|_| {
        assert!(std::env::var("TEST_KEY").is_err());

        load_filename(".env").expect("set env variables");
        assert_eq!(std::env::var("TEST_KEY").unwrap(), "test_val");
    });
}
