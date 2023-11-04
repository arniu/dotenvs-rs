mod fixtures;
use fixtures::*;

#[test]
fn test_sample() -> anyhow::Result<()> {
    let (_t, mut exps) = with_expand_dotenv()?;
    for (key, value) in dotenv::from_filename(".env")?.iter() {
        let expected = exps.remove(key).unwrap();
        assert_eq!(expected, value, "check {}", key);
    }
    assert!(exps.is_empty());

    Ok(())
}
