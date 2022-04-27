mod common;

use common::*;
use dotenv::*;

#[test]
fn test_from_filename_iter() {
    with_sample(|t| {
        let path = t.path().join(".env");
        let iter = from_path_iter(&path).unwrap();
        assert!(std::env::var("TEST_KEY").is_err());

        iter.load().expect("set env variables");
        assert_eq!(std::env::var("TEST_KEY").unwrap(), "test_val");
    });
}
