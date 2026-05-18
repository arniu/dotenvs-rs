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

// ---- §1 File Structure / Top-level ----

pub(crate) fn parse<'a>(input: &mut &'a str) -> winnow::Result<Option<Pair<'a>>> {
    delimited(
        multispace0,
        alt((comment_line.map(|_| None), kv_line.map(Some))),
        multispace0,
    )
    .parse_next(input)
}

// ---- §3 comment-line = "#" *NON-EOL ----

fn comment_line<'a>(input: &mut &'a str) -> winnow::Result<&'a str> {
    preceded("#", till_line_ending).parse_next(input)
}

// ---- §4 export = "export" 1*WSP ----

fn export_prefix<'a>(input: &mut &'a str) -> winnow::Result<&'a str> {
    preceded("export", space1).parse_next(input)
}

// ---- §1 kv-line = [ export ] key separator [ value ] [ inline-comment ] ----

fn kv_line<'a>(input: &mut &'a str) -> winnow::Result<Pair<'a>> {
    preceded(opt(export_prefix), separated_pair(key, separator, value)).parse_next(input)
}

// ---- §5 key = ( ALPHA / "_" ) *( ALPHA / DIGIT / "_" ) ----

fn key<'a>(input: &mut &'a str) -> winnow::Result<&'a str> {
    take_while(1.., |c: char| c.is_ascii_alphanumeric() || c == '_')
        .verify(|s: &str| {
            let first = s.chars().next().unwrap();
            first.is_ascii_alphabetic() || first == '_'
        })
        .parse_next(input)
}

// ---- §6 separator = *WSP "=" *WSP ----

fn separator<'a>(input: &mut &'a str) -> winnow::Result<()> {
    (space0, "=", space0).void().parse_next(input)
}

// ---- §7 Values ----

fn value<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    if input.is_empty() {
        return Ok(Value::List(vec![]));
    }

    match input.as_bytes()[0] as char {
        '\'' => single_quoted.parse_next(input),
        '"' => double_quoted.parse_next(input),
        '`' => backtick_quoted.parse_next(input),
        _ => unquoted.parse_next(input),
    }
}

// §7.1 single-quoted = "'" *single-char "'"
fn single_quoted<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    delimited("'", take_till(0.., |c| c == '\''), "'")
        .map(Value::Lit)
        .parse_next(input)
}

// §7.2 double-quoted = DQUOTE *double-char DQUOTE
fn double_quoted<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    let content = delimited("\"", take_till(0.., |c| c == '"'), "\"").parse_next(input)?;
    let mut content_input = content;
    expand(true).parse_next(&mut content_input)
}

// §7.3 backtick-quoted = "`" *backtick-char "`"
fn backtick_quoted<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    delimited("`", take_till(0.., |c| c == '`'), "`")
        .map(Value::Lit)
        .parse_next(input)
}

// §7.4 unquoted = 1*unquoted-char
fn unquoted<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    let raw = take_till(0.., |c: char| c == '\n' || c == '\r' || c == '#')
        .map(|s: &str| s.trim())
        .parse_next(input)?;
    let mut raw_input = raw;
    expand(false).parse_next(&mut raw_input)
}

// ---- §7 Internal: content expander for quoted/unquoted values ----

fn expand<'a>(expand_new_lines: bool) -> impl Parser<&'a str, Value<'a>, ContextError> {
    repeat(
        0..,
        alt((
            substitution,
            escape_seq(expand_new_lines),
            take_till(1.., |c: char| c == '\\' || c == '$').map(Value::Lit),
        )),
    )
    .map(Value::List)
}

fn escape_seq<'a>(expand_new_lines: bool) -> impl Parser<&'a str, Value<'a>, ContextError> {
    let new_line = if expand_new_lines { "\n" } else { "\\n" };
    preceded(
        "\\",
        one_of(if expand_new_lines {
            &['\\', '$', 'n', 'r'][..]
        } else {
            &['\\', '$', 'n'][..]
        }),
    )
    .map(move |c: char| {
        Value::Lit(match c {
            '\\' => "\\",
            '$' => "$",
            'n' => new_line,
            'r' => "\r",
            _ => unreachable!(),
        })
    })
}

// ---- §9 Variable Substitution ----

fn substitution<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    alt((brace_sub, simple_sub)).parse_next(input)
}

// §9 simple-sub = "$" key
fn simple_sub<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    preceded("$", key)
        .map(|name| Value::Sub(name, None))
        .parse_next(input)
}

// §9 brace-sub = "${" key [ ":-" fallback ] "}"
fn brace_sub<'a>(input: &mut &'a str) -> winnow::Result<Value<'a>> {
    delimited("${", (key, opt(preceded(":-", fallback))), "}")
        .map(|(name, maybe): (&str, Option<Value>)| Value::Sub(name, maybe.map(Box::new)))
        .parse_next(input)
}

// §9 fallback = *( substitution / fallback-text )
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
