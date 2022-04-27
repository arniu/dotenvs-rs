mod common;

use common::*;
use dotenv::*;

use std::collections::HashMap;

#[test]
fn test_vars() {
    with_sample(|_| {
        let map: HashMap<String, String> = vars().collect();
        assert_eq!(map["TEST_KEY"], "test_val");
    });
}
