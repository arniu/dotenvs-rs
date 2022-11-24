mod fixtures;
use fixtures::*;

use std::collections::HashMap;

static SAMPLE_JSON: &str = include_str!("fixtures/sample-expand.json");

#[test]
fn test_sample() -> anyhow::Result<()> {
    let _cwd = make_expand_dotenv()?;

    let map: HashMap<String, String> = serde_json::from_str(SAMPLE_JSON)?;
    for (key, value) in dotenv::from_filename(".env")?.iter() {
        let expected = map.get(key).unwrap();
        assert_eq!(expected, &value, "check value for {}: ", key);
    }

    Ok(())
}
