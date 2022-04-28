mod common;

use common::*;
use dotenv::*;

#[test]
fn test() {
    with_sample(|_| {
        load().expect("set env variables");
        assert_eq!(std::env::var("TEST_KEY").unwrap(), "test_val");
    });
}

#[test]
fn test_child_dir() {
    with_sample(|t| {
        t.ensure_current_dir("child");
        assert!(load().is_ok());
    });
}

#[test]
fn test_deep_dir() {
    with_sample(|t| {
        t.ensure_current_dir("deep/dir");
        assert!(load().is_ok());
    });
}
