mod fixtures;
use fixtures::*;

#[test]
fn test_propagate_env_parse_errors() -> anyhow::Result<()> {
    let (_t, mut exps) = with_basic_dotenv()?;

    // This is an example of how a consumer that cares about invalid `.env` files can handle them using the
    // `Iter::try_next` API (see: https://github.com/arniu/dotenvs-rs/issues/4)
    let env = dotenv::from_filename(".env")?;
    let mut iter = env.iter();
    while let Some((key, value)) = iter.try_next()? {
        let expected = exps.remove(key).unwrap();
        assert_eq!(expected, value, "check {}", key);
    }
    assert!(exps.is_empty());

    Ok(())
}
