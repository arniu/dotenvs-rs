use crate::error::Error::LineParse;
use crate::Dotenv;

#[test]
fn should_not_parse_unfinished_substitutions() {
    let wrong_value = ">${KEY{<";

    let parsed_values: Vec<_> = Dotenv::new(
        format!(
            r#"
    KEY=VALUE
    KEY1={}
    "#,
            wrong_value
        )
        .as_bytes(),
    )
    .collect();

    assert_eq!(parsed_values.len(), 2);

    if let Ok(first_line) = &parsed_values[0] {
        assert_eq!(first_line, &(String::from("KEY"), String::from("VALUE")))
    } else {
        assert!(false, "Expected the first value to be parsed")
    }

    if let Err(LineParse(second_value, index)) = &parsed_values[1] {
        assert_eq!(second_value, wrong_value);
        assert_eq!(*index, wrong_value.len() - 1)
    } else {
        assert!(false, "Expected the second value not to be parsed")
    }
}

#[test]
fn should_not_allow_dot_as_first_character_of_key() {
    let wrong_key_value = ".Key=VALUE";

    let parsed_values: Vec<_> = Dotenv::new(wrong_key_value.as_bytes()).collect();

    assert_eq!(parsed_values.len(), 1);

    if let Err(LineParse(second_value, index)) = &parsed_values[0] {
        assert_eq!(second_value, wrong_key_value);
        assert_eq!(*index, 0)
    } else {
        assert!(false, "Expected the second value not to be parsed")
    }
}

#[test]
fn should_not_parse_illegal_format() {
    let wrong_format = r"<><><>";
    let parsed_values: Vec<_> = Dotenv::new(wrong_format.as_bytes()).collect();

    assert_eq!(parsed_values.len(), 1);

    if let Err(LineParse(wrong_value, index)) = &parsed_values[0] {
        assert_eq!(wrong_value, wrong_format);
        assert_eq!(*index, 0)
    } else {
        assert!(false, "Expected the second value not to be parsed")
    }
}

#[test]
fn should_not_parse_illegal_escape() {
    let wrong_escape = r">\f<";
    let parsed_values: Vec<_> = Dotenv::new(format!("VALUE={}", wrong_escape).as_bytes()).collect();

    assert_eq!(parsed_values.len(), 1);

    if let Err(LineParse(wrong_value, index)) = &parsed_values[0] {
        assert_eq!(wrong_value, wrong_escape);
        assert_eq!(*index, wrong_escape.find('\\').unwrap() + 1)
    } else {
        assert!(false, "Expected the second value not to be parsed")
    }
}
