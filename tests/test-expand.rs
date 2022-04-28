mod common;

use common::*;
use dotenv::*;

#[test]
fn test() {
    std::env::set_var("KEY", "value");
    std::env::set_var("KEY1", "value1");

    let expr_list = [
        "$ZZZ", "$KEY", "$KEY1", "${KEY}1", "$KEY_U", "${KEY_U}", "\\$KEY",
    ];

    let expected_list = [
        "",
        "value",
        "value1",
        "value1",
        "value_U",
        "value+UUID",
        "$KEY",
    ];

    let expr = expr_list.join(">>");
    let expected = expected_list.join(">>");

    let contents = format!(
        r#"
KEY1=new_value1
KEY_U=$KEY+UUID

WITHOUT_QUOTES={}
WEAK_QUOTED="{}"
STRONG_QUOTED='{}'
"#,
        expr, expr, expr
    );

    with_dotenv(&contents, |_| {
        assert_eq!(var("KEY").unwrap(), "value");
        assert_eq!(var("KEY1").unwrap(), "value1");
        assert_eq!(var("KEY_U").unwrap(), "value+UUID");
        assert_eq!(var("WITHOUT_QUOTES").unwrap(), expected);
        assert_eq!(var("WEAK_QUOTED").unwrap(), expected);
        assert_eq!(var("STRONG_QUOTED").unwrap(), expr);
    });
}
