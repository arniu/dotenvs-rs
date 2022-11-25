mod fixtures;
use fixtures::*;

#[test]
fn test_sample() -> anyhow::Result<()> {
    let (_t, exps) = with_basic_dotenv()?;
    for (key, value) in dotenv::from_filename(".env")?.iter() {
        let expected = exps.get(key).unwrap();
        assert_eq!(expected, &value, "check {}", key);
    }

    Ok(())
}
