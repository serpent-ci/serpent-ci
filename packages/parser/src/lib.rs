use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::recognize,
    error::ParseError,
    multi::{many0, many0_count},
    sequence::{delimited, pair, tuple},
    IResult,
};

#[derive(PartialEq, Debug)]
pub struct Module {
    functions: Vec<Function>,
}

#[derive(PartialEq, Debug)]
pub struct Function {
    name: String,
}

pub fn module(input: &str) -> IResult<&str, Module> {
    let (input, functions) = many0(function)(input)?;

    Ok((input, Module { functions }))
}

fn function(input: &str) -> IResult<&str, Function> {
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
            name: name.to_owned(),
        },
    ))
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod tests {
    use crate::{module, Function, Module};

    #[test]
    fn basic_parsing() {
        let input = include_str!("../../../tests/parse.py");
        let (_, module) = module(input).unwrap();
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
