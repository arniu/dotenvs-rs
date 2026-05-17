use winnow::ascii::{multispace0, space0, space1, till_line_ending};
use winnow::combinator::{alt, delimited, opt, preceded, repeat, separated_pair};
use winnow::error::ContextError;
use winnow::token::{one_of, take_till, take_while};
use winnow::Parser;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Value<'a> {
    Lit(&'a str),
    Sub(&'a str, Option<Box<Value<'a>>>),
    List(Vec<Value<'a>>),
}

pub(crate) type Pair<'a> = (&'a str, Value<'a>);

pub(crate) fn parse<'a>(input: &mut &'a str) -> winnow::Result<Option<Pair<'a>>> {
    delimited(
        multispace0,
        alt((
            comment.map(|_| None),
            kv_line.map(Some),
        )),
        multispace0,
    )
    .parse_next(input)
}

fn comment<'a>(input: &mut &'a str) -> winnow::Result<&'a str> {
    preceded("#", till_line_ending).parse_next(input)
}

fn kv_line<'a>(input: &mut &'a str) -> winnow::Result<Pair<'a>> {
    preceded(
        opt(("export", space1)),
        separated_pair(key, (space0, "=", space0), value),
    )
    .parse_next(input)
}

fn key<'a>(input: &mut &'a str) -> winnow::Result<&'a str> {
    take_while(1.., |c: char| c.is_ascii_alphanumeric() || c == '_')
        .verify(|s: &str| {
            let first = s.chars().next().unwrap();
            first.is_ascii_alphabetic() || first == '_'
        })
        .parse_next(input)
}

fn value<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    if input.is_empty() {
        return Ok(Value::List(vec![]));
    }
    match input.as_bytes()[0] as char {
        '\'' => single_quoted.parse_next(input),
        '"' => double_quoted.parse_next(input),
        '`' => backtick_quoted.parse_next(input),
        _ => unquoted_value.parse_next(input),
    }
}

fn backtick_quoted<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    delimited("`", take_till(0.., |c| c == '`'), "`")
        .map(Value::Lit)
        .parse_next(input)
}

fn single_quoted<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    delimited("'", take_till(0.., |c| c == '\''), "'")
        .map(Value::Lit)
        .parse_next(input)
}

fn double_quoted<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    let content = delimited("\"", take_till(0.., |c| c == '"'), "\"").parse_next(input)?;
    let mut content_input = content;
    expand(true).parse_next(&mut content_input)
}

fn unquoted_value<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    let raw = take_till(0.., |c: char| c == '\n' || c == '\r' || c == '#')
        .map(|s: &str| s.trim())
        .parse_next(input)?;
    let mut raw_input = raw;
    expand(false).parse_next(&mut raw_input)
}

fn expand<'a>(expand_new_lines: bool) -> impl Parser<&'a str, Value<'a>, ContextError> {
    repeat(
        0..,
        alt((
            substitution,
            escape(expand_new_lines),
            take_till(1.., |c: char| c == '\\' || c == '$').map(Value::Lit),
        )),
    )
    .map(Value::List)
}

fn escape<'a>(expand_new_lines: bool) -> impl Parser<&'a str, Value<'a>, ContextError> {
    let new_line = if expand_new_lines { "\n" } else { "\\n" };
    // double-quoted: \n \r \\ \$; unquoted: \\ \$ only (\n handled as literal)
    preceded("\\", one_of(if expand_new_lines { &['\\', '$', 'n', 'r'][..] } else { &['\\', '$', 'n'][..] })).map(move |c: char| {
        Value::Lit(match c {
            '\\' => "\\",
            '$' => "$",
            'n' => new_line,
            'r' => "\r",
            _ => unreachable!(),
        })
    })
}

fn substitution<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    alt((substitution_braces, substitution_simple)).parse_next(input)
}

fn substitution_simple<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    preceded("$", key)
        .map(|name| Value::Sub(name, None))
        .parse_next(input)
}

fn substitution_braces<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    delimited(
        "${",
        (key, opt(preceded(":-", fallback))),
        "}",
    )
    .map(|(name, maybe): (&str, Option<Value>)| Value::Sub(name, maybe.map(Box::new)))
    .parse_next(input)
}

fn fallback<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    repeat(
        0..,
        alt((
            substitution,
            take_till(1.., |c: char| c == '$' || c == '}').map(Value::Lit),
        )),
    )
    .map(Value::List)
    .parse_next(input)
}
