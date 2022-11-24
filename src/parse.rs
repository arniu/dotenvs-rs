use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::combinator::map_parser;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::multi::many0_count;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Value<'a> {
    Lit(&'a str),
    Var(&'a str, Option<Box<Value<'a>>>),
    List(Vec<Value<'a>>),
}

pub(crate) type Pair<'a> = (&'a str, Value<'a>);

pub(crate) fn parse(input: &str) -> IResult<&str, Option<Pair<'_>>> {
    delimited(
        multispace0,
        alt((map(comment, |_| None), map(kv_pair, Some))),
        multispace0,
    )(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    preceded(char('#'), is_not("\n\r"))(input)
}

fn kv_pair(input: &str) -> IResult<&str, Pair<'_>> {
    let export_ = pair(tag("export"), space1);
    let _eq_ = delimited(space0, char('='), space0);
    preceded(opt(export_), separated_pair(key, _eq_, value))(input)
}

fn key(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_"), tag(".")))),
    ))(input)
}

fn value(input: &str) -> IResult<&str, Value<'_>> {
    alt((
        map(quoted_with('`'), Value::Lit),
        map(quoted_with('\''), Value::Lit),
        map_parser(quoted_with('\"'), expand(true)),
        map_parser(simple_value, expand(false)), // LAST one
    ))(input)
}

fn simple_value(input: &str) -> IResult<&str, &str> {
    not_line_ending(input).map(|(_, text)| {
        let idx = text.find('#').unwrap_or(text.len());
        (&input[idx..], input[..idx].trim())
    })
}

fn quoted_with<'a>(mark: char) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(char(mark), take_till(move |c| c == mark), char(mark))
}

fn expand<'a>(expand_new_lines: bool) -> impl FnMut(&'a str) -> IResult<&'a str, Value<'a>> {
    map(
        many0(alt((
            substitution,
            escape(expand_new_lines),
            map(is_not("\\$"), Value::Lit), // LAST one
        ))),
        Value::List,
    )
}

fn escape<'a>(expand_new_lines: bool) -> impl FnMut(&'a str) -> IResult<&'a str, Value<'a>> {
    let new_line = if expand_new_lines { "\n" } else { "\\n" };
    map(preceded(char('\\'), one_of("\\$n")), move |c| {
        Value::Lit(match c {
            '\\' => "\\",
            '$' => "$",
            'n' => new_line,
            _ => unreachable!(),
        })
    })
}

fn substitution(input: &str) -> IResult<&str, Value<'_>> {
    let default = alt((substitution, map(is_not("}"), Value::Lit)));

    alt((
        map(preceded(char('$'), key), |name| Value::Var(name, None)),
        map(
            delimited(
                tag("${"),
                pair(key, opt(preceded(tag(":-"), default))),
                tag("}"),
            ),
            |(name, maybe)| Value::Var(name, maybe.map(Box::new)),
        ),
    ))(input)
}
