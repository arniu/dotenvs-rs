use dotenv::Error;

const BAD_ENV: &str = r#"
A=foo bar
B="notenough
C='toomany''
D=valid
export NOT_SET
E=valid
"#;

#[test]
fn test_bad_env() -> anyhow::Result<()> {
    let env = dotenv::from_read(BAD_ENV.as_bytes())?;

    assert_eq!(
        vec![
            ("A", "foo bar".into()),
            ("B", "\"notenough".into()),
            ("C", "toomany".into())
        ],
        env.iter().collect::<Vec<_>>()
    );

    let mut iter = env.iter();
    assert_eq!(Some(("A", "foo bar".into())), iter.try_next()?);
    assert_eq!(Some(("B", "\"notenough".into())), iter.try_next()?);
    assert_eq!(Some(("C", "toomany".into())), iter.try_next()?);

    // TODO: Use assert_matches! when it stabilizes: https://github.com/rust-lang/rust/issues/82775
    assert!(matches!(
        iter.try_next().unwrap_err(),
        Error::Parse(err) if err == "Parsing Error: Error { input: \"'\\nD=valid\\nexport NOT_SET\\nE=valid\\n\", code: Tag }"
    ));

    Ok(())
}
