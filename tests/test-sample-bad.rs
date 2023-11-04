use dotenv::Error;
use std::collections::HashMap;
use std::iter::{IntoIterator, Iterator};

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
        ]
        .into_iter()
        .collect::<HashMap<_, _>>(),
        env.iter().collect::<HashMap<_, _>>()
    );

    let mut iter = env.iter();
    assert_eq!(Some(("A", "foo bar".into())), iter.try_next()?);
    assert_eq!(Some(("B", "\"notenough".into())), iter.try_next()?);
    assert_eq!(Some(("C", "toomany".into())), iter.try_next()?);

    // TODO: Use assert_matches! when it stabilizes: https://github.com/rust-lang/rust/issues/82775
    match iter.try_next().unwrap_err() {
        Error::Parse(err) => assert_eq!(
            "Parsing Error: Error { input: \"'\\nD=valid\\nexport NOT_SET\\nE=valid\\n\", code: Tag }",
            err
        ),
        err => panic!("Unexpected error variant: {err:?}", err = err),
    }

    Ok(())
}
