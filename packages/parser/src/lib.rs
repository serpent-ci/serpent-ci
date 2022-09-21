use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0, space0, space1},
    combinator::{all_consuming, eof, map, recognize},
    error::{context, ErrorKind},
    multi::{many0, many_till},
    sequence::{delimited, pair, tuple},
    Finish, IResult,
};
use nom_greedyerror::{convert_error, GreedyError};
use nom_locate::LocatedSpan;
use thiserror::Error;

type Span<'a> = LocatedSpan<&'a str>;

type ParseResult<'a, T> = IResult<Span<'a>, T, GreedyError<Span<'a>, ErrorKind>>;

#[derive(PartialEq, Eq, Debug)]
pub struct Module {
    functions: Vec<Function>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Function {
    name: String,
}

#[derive(Error, Debug)]
#[error("Parse error:\n{0}")]
pub struct ParseError(String);

impl ParseError {
    pub fn text(&self) -> &str {
        &self.0
    }
}

pub fn parse(input: &str) -> Result<Module, ParseError> {
    match all_consuming(module)(Span::new(input)).finish() {
        Ok((_, module)) => Ok(module),
        Err(e) => Err(ParseError(convert_error(input, e))),
    }
}

fn module(input: Span) -> ParseResult<Module> {
    let (input, (functions, _)) = context("module", many_till(multiline_ws(function), eof))(input)?;

    Ok((input, Module { functions }))
}

fn function(input: Span) -> ParseResult<Function> {
    let (input, (_def, _, name, _params, _colon, _pass)) = context(
        "function",
        tuple((def, space1, identifier, ws(tag("()")), colon, pass)),
    )(input)?;

    Ok((
        input,
        Function {
            name: (*name.fragment()).to_owned(),
        },
    ))
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
    use crate::{parse, Function, Module};

    #[test]
    fn basic_parsing() {
        let input = include_str!("../../../tests/parse.py");
        let module = parse(input).unwrap();
        assert_eq!(
            module,
            Module {
                functions: vec![Function {
                    name: "test".to_owned()
                }]
            }
        );
    }
}
