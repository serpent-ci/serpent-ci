use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, alphanumeric1, line_ending, multispace0, space0, space1},
    combinator::{all_consuming, eof, map, opt, recognize},
    error::{context, ErrorKind},
    multi::{many0, many_till, separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair, tuple},
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
                name: name.fragment().to_string(),
                body,
            },
        ))
    }

    fn inline_body(input: Span) -> ParseResult<Vec<Statement>> {
        let (input, statement) = context("inline body", Statement::parse)(input)?;

        Ok((input, vec![statement]))
    }

    fn block_body(input: Span) -> ParseResult<Vec<Statement>> {
        let (input, _) = discard(pair(eol, blank_lines))(input)?;
        let (input, prefix) = space0(input)?;

        // TODO: Is error reporting friendly enough?
        separated_list1(
            tuple((eol, blank_lines, tag(*prefix.fragment()))),
            Statement::parse,
        )(input)
    }
}

fn blank_lines(input: Span) -> ParseResult<()> {
    discard(many0(pair(space0, eol)))(input)
}

fn eol(input: Span) -> ParseResult<()> {
    discard(tuple((
        space0,
        opt(pair(tag("#"), is_not("\r\n"))),
        line_ending,
    )))(input)
}

#[derive(Eq, PartialEq, Debug)]
pub enum Statement {
    Pass,
    Expression(Expression),
    // TODO: Loops
}

impl Statement {
    fn parse(input: Span) -> ParseResult<Self> {
        let (input, stmt) = context(
            "statement",
            alt((
                map(pass, |_| Statement::Pass),
                map(Expression::parse, Statement::Expression),
            )),
        )(input)?;

        Ok((input, stmt))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Expression {
    Variable { name: String },
    Call { name: String, args: Vec<Expression> },
}

impl Expression {
    fn parse(input: Span) -> ParseResult<Self> {
        alt((Self::call, Self::variable, Self::parenthasized))(input)
    }

    fn variable(input: Span) -> ParseResult<Self> {
        map(identifier, |name| Self::Variable {
            name: name.fragment().to_string(),
        })(input)
    }

    fn call(input: Span) -> ParseResult<Self> {
        let (input, (name, args)) = context(
            "call",
            separated_pair(
                identifier,
                space0,
                delimited(
                    tag("("),
                    separated_list0(tag(","), multiline_ws(Self::parse)),
                    tag(")"),
                ),
            ),
        )(input)?;

        Ok((
            input,
            Self::Call {
                name: name.fragment().to_string(),
                args,
            },
        ))
    }

    fn parenthasized(input: Span) -> ParseResult<Self> {
        context(
            "parenthesized",
            delimited(tag("("), multiline_ws(Expression::parse), tag(")")),
        )(input)
    }
}

pub enum BinOp {
    Plus,
    Minus,
    Mulitply,
    Divide,
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
    use indoc::indoc;

    use crate::{parse, Function, Module, Statement};

    #[test]
    fn basic_parsing() {
        let input = indoc! {"
            def test():
                pass
        "};
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
