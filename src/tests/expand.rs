use crate::Dotenv;

fn assert_parsed_string(input_string: &str, expected_parse_result: Vec<(&str, &str)>) {
    let actual_iter = Dotenv::new(input_string.as_bytes());
    let expected_count = &expected_parse_result.len();

    let expected_iter = expected_parse_result
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()));

    let mut count = 0;
    for (expected, actual) in expected_iter.zip(actual_iter) {
        assert!(actual.is_ok());
        assert_eq!(expected, actual.ok().unwrap());
        count += 1;
    }

    assert_eq!(count, *expected_count);
}

#[test]
fn variable_in_parenthesis_surrounded_by_quotes() {
    assert_parsed_string(
        r#"
            KEY=test
            KEY1="${KEY}"
            "#,
        vec![("KEY", "test"), ("KEY1", "test")],
    );
}

#[test]
fn substitute_undefined_variables_to_empty_string() {
    assert_parsed_string(r#"KEY=">$KEY1<>${KEY2}<""#, vec![("KEY", "><><")]);
}

#[test]
fn do_not_substitute_variables_with_dollar_escaped() {
    assert_parsed_string(
        "KEY=>\\$KEY1<>\\${KEY2}<",
        vec![("KEY", ">$KEY1<>${KEY2}<")],
    );
}

#[test]
fn do_not_substitute_variables_in_weak_quotes_with_dollar_escaped() {
    assert_parsed_string(
        r#"KEY=">\$KEY1<>\${KEY2}<""#,
        vec![("KEY", ">$KEY1<>${KEY2}<")],
    );
}

#[test]
fn do_not_substitute_variables_in_strong_quotes() {
    assert_parsed_string("KEY='>${KEY1}<>$KEY2<'", vec![("KEY", ">${KEY1}<>$KEY2<")]);
}

#[test]
fn same_variable_reused() {
    assert_parsed_string(
        r#"
    KEY=VALUE
    KEY1=$KEY$KEY
    "#,
        vec![("KEY", "VALUE"), ("KEY1", "VALUEVALUE")],
    );
}

#[test]
fn with_dot() {
    assert_parsed_string(
        r#"
    KEY.Value=VALUE
    "#,
        vec![("KEY.Value", "VALUE")],
    );
}

#[test]
fn recursive_substitution() {
    assert_parsed_string(
        r#"
            KEY=${KEY1}+KEY_VALUE
            KEY1=${KEY}+KEY1_VALUE
            "#,
        vec![("KEY", "+KEY_VALUE"), ("KEY1", "+KEY_VALUE+KEY1_VALUE")],
    );
}

#[test]
fn variable_without_parenthesis_is_substituted_before_separators() {
    assert_parsed_string(
        r#"
            KEY1=test_user
            KEY1_1=test_user_with_separator
            KEY=">$KEY1_1<>$KEY1}<>$KEY1{<"
            "#,
        vec![
            ("KEY1", "test_user"),
            ("KEY1_1", "test_user_with_separator"),
            ("KEY", ">test_user_1<>test_user}<>test_user{<"),
        ],
    );
}

#[test]
fn substitute_variable_from_env_variable() {
    std::env::set_var("KEY11", "test_user_env");

    assert_parsed_string(r#"KEY=">${KEY11}<""#, vec![("KEY", ">test_user_env<")]);
}

#[test]
fn substitute_variable_env_variable_overrides_dotenv_in_substitution() {
    std::env::set_var("KEY11", "test_user_env");

    assert_parsed_string(
        r#"
    KEY11=test_user
    KEY=">${KEY11}<"
    "#,
        vec![("KEY11", "test_user"), ("KEY", ">test_user_env<")],
    );
}

#[test]
fn consequent_substitutions() {
    assert_parsed_string(
        r#"
    KEY1=test_user
    KEY2=$KEY1_2
    KEY=>${KEY1}<>${KEY2}<
    "#,
        vec![
            ("KEY1", "test_user"),
            ("KEY2", "test_user_2"),
            ("KEY", ">test_user<>test_user_2<"),
        ],
    );
}

#[test]
fn consequent_substitutions_with_one_missing() {
    assert_parsed_string(
        r#"
    KEY2=$KEY1_2
    KEY=>${KEY1}<>${KEY2}<
    "#,
        vec![("KEY2", "_2"), ("KEY", "><>_2<")],
    );
}
