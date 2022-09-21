use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, line_ending, multispace0, space0, space1},
    combinator::{all_consuming, eof, map, recognize},
    error::{context, ErrorKind},
    multi::{many0, many_till, separated_list1},
    sequence::{delimited, pair, tuple},
    Finish, IResult,
};
use nom_greedyerror::{convert_error, GreedyError};
use nom_locate::LocatedSpan;
use thiserror::Error;

pub fn parse(input: &str) -> Result<Module, ParseError> {
    match all_consuming(Module::parse)(Span::new(input)).finish() {
        Ok((_, module)) => Ok(module),
        Err(e) => Err(ParseError(convert_error(input, e))),
    }
}

type Span<'a> = LocatedSpan<&'a str>;

type ParseResult<'a, T> = IResult<Span<'a>, T, GreedyError<Span<'a>, ErrorKind>>;

#[derive(PartialEq, Eq, Debug)]
pub struct Module {
    functions: Vec<Function>,
}

impl Module {
    fn parse(input: Span) -> ParseResult<Self> {
        let (input, (functions, _)) =
            context("module", many_till(multiline_ws(Function::parse), eof))(input)?;

        Ok((input, Module { functions }))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Function {
    name: String,
    body: Vec<Statement>,
}

impl Function {
    fn parse(input: Span) -> ParseResult<Self> {
        let (input, (_def, _, name, _params, _colon, body)) = context(
            "function",
            tuple((
                def,
                space1,
                identifier,
                ws(tag("()")),
                colon,
                alt((Self::inline_body, Self::block_body)),
            )),
        )(input)?;

        Ok((
            input,
            Function {
                name: (*name.fragment()).to_owned(),
                body,
            },
        ))
    }

    fn inline_body(input: Span) -> ParseResult<Vec<Statement>> {
        let (input, statement) = context("inline body", Statement::parse)(input)?;

        Ok((input, vec![statement]))
    }

    fn block_body(input: Span) -> ParseResult<Vec<Statement>> {
        let (input, _) = discard(pair(space0, line_ending))(input)?;
        let (input, prefix) = space1(input)?;

        separated_list1(pair(line_ending, tag(*prefix.fragment())), Statement::parse)(input)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Statement {
    Pass,
}

impl Statement {
    fn parse(input: Span) -> ParseResult<Self> {
        let (input, _pass) = context("statement", pass)(input)?;

        Ok((input, Statement::Pass))
    }
}

#[derive(Error, Debug)]
#[error("Parse error:\n{0}")]
pub struct ParseError(String);

impl ParseError {
    pub fn text(&self) -> &str {
        &self.0
    }
}

fn identifier(input: Span) -> ParseResult<Span> {
    context(
        "identifier",
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
    )(input)
}

fn ws<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> ParseResult<'a, O>
where
    F: 'a + FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    delimited(space0, inner, space0)
}

fn multiline_ws<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> ParseResult<'a, O>
where
    F: 'a + FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    delimited(multispace0, inner, multispace0)
}

fn discard<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> ParseResult<'a, ()>
where
    F: 'a + FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    map(inner, |_| ())
}

macro_rules! keywords {
    ($($kw:ident),*) => {
        $(
            fn $kw(input: Span) -> ParseResult<()> {
                discard(tag(stringify!($kw)))(input)
            }
        )*
    }
}

keywords!(def, pass);

macro_rules! operators {
    ($(($name:ident, $op:expr)),*) => {
        $(
            fn $name(input: Span) -> ParseResult<()> {
                ws(discard(tag($op)))(input)
            }
        )*
    }
}

operators!((colon, ":"));

#[cfg(test)]
mod tests {
    use crate::{parse, Function, Module, Statement};

    #[test]
    fn basic_parsing() {
        let input = include_str!("../../../tests/parse.py");
        let module = parse(input).unwrap();
        assert_eq!(
            module,
            Module {
                functions: vec![Function {
                    name: "test".to_owned(),
                    body: vec![Statement::Pass],
                }]
            }
        );
    }
}
