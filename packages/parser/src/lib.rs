use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::{all_consuming, recognize},
    error::ErrorKind,
    multi::many0,
    sequence::{delimited, pair, tuple},
    Finish, IResult,
};
use nom_greedyerror::{convert_error, GreedyError};
use nom_locate::LocatedSpan;
use thiserror::Error;

type Span<'a> = LocatedSpan<&'a str>;

type ParseResult<'a, T> = IResult<Span<'a>, T, GreedyError<Span<'a>, ErrorKind>>;

#[derive(PartialEq, Debug)]
pub struct Module {
    functions: Vec<Function>,
}

#[derive(PartialEq, Debug)]
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
    let (_, script) = match all_consuming(ws(module))(Span::new(input)).finish() {
        Ok(ok) => Ok(ok),
        Err(e) => Err(ParseError(convert_error(input, e))),
    }?;

    Ok(script)
}

fn module(input: Span) -> ParseResult<Module> {
    let (input, functions) = many0(function)(input)?;

    Ok((input, Module { functions }))
}

fn function(input: Span) -> ParseResult<Function> {
    let (input, (_def, name, _params, _colon, _pass)) = tuple((
        tag("def"),
        ws(identifier),
        ws(tag("()")),
        ws(tag(":")),
        tag("pass"),
    ))(input)?;

    Ok((
        input,
        Function {
            name: (*name.fragment()).to_owned(),
        },
    ))
}

fn identifier(input: Span) -> ParseResult<Span> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(Span<'a>) -> ParseResult<'a, O>
where
    F: FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    delimited(multispace0, inner, multispace0)
}

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
