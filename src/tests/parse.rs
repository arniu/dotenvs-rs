use super::super::Dotenv;
use crate::error::Result;

#[test]
fn test_parse_line_env() {
    // Note 5 spaces after 'KEY8=' below
    let actual_iter = Dotenv::new(
        r#"
KEY=1
KEY2="2"
KEY3='3'
KEY4='fo ur'
KEY5="fi ve"
KEY6=s\ ix
KEY7=
KEY8=
KEY9=   # foo
KEY10  ="whitespace before ="
KEY11=    "whitespace after ="
export="export as key"
export   SHELL_LOVER=1
"#
        .as_bytes(),
    );

    let expected_iter = vec![
        ("KEY", "1"),
        ("KEY2", "2"),
        ("KEY3", "3"),
        ("KEY4", "fo ur"),
        ("KEY5", "fi ve"),
        ("KEY6", "s ix"),
        ("KEY7", ""),
        ("KEY8", ""),
        ("KEY9", ""),
        ("KEY10", "whitespace before ="),
        ("KEY11", "whitespace after ="),
        ("export", "export as key"),
        ("SHELL_LOVER", "1"),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value.to_string()));

    let mut count = 0;
    for (expected, actual) in expected_iter.zip(actual_iter) {
        assert!(actual.is_ok());
        assert_eq!(expected, actual.ok().unwrap());
        count += 1;
    }

    assert_eq!(count, 13);
}

#[test]
fn test_parse_line_comment() {
    let result: Result<Vec<(String, String)>> = Dotenv::new(
        r#"
# foo=bar
#    "#
            .as_bytes(),
    )
    .collect();
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_parse_line_invalid() {
    // Note 4 spaces after 'invalid' below
    let actual_iter = Dotenv::new(
        r#"
  invalid
very bacon = yes indeed
=value"#
            .as_bytes(),
    );

    let mut count = 0;
    for actual in actual_iter {
        assert!(actual.is_err());
        count += 1;
    }
    assert_eq!(count, 3);
}

#[test]
fn test_parse_value_escapes() {
    let actual_iter = Dotenv::new(
        r#"
KEY=my\ cool\ value
KEY2=\$sweet
KEY3="awesome stuff \"mang\""
KEY4='sweet $\fgs'\''fds'
KEY5="'\"yay\\"\ "stuff"
KEY6="lol" #well you see when I say lol wh
KEY7="line 1\nline 2"
"#
        .as_bytes(),
    );

    let expected_iter = vec![
        ("KEY", r#"my cool value"#),
        ("KEY2", r#"$sweet"#),
        ("KEY3", r#"awesome stuff "mang""#),
        ("KEY4", r#"sweet $\fgs'fds"#),
        ("KEY5", r#"'"yay\ stuff"#),
        ("KEY6", "lol"),
        ("KEY7", "line 1\nline 2"),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value.to_string()));

    for (expected, actual) in expected_iter.zip(actual_iter) {
        assert!(actual.is_ok());
        assert_eq!(expected, actual.unwrap());
    }
}

#[test]
fn test_parse_value_escapes_invalid() {
    let actual_iter = Dotenv::new(
        r#"
KEY=my uncool value
KEY2="why
KEY3='please stop''
KEY4=h\8u
"#
        .as_bytes(),
    );

    for actual in actual_iter {
        assert!(actual.is_err());
    }
}
