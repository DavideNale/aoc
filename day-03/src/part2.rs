use core::panic;

use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

enum Status {
    Run,
    Skip,
}

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

pub fn process(_input: &str) -> miette::Result<String> {
    let (_, parsed) = parse(_input).map_err(|e| miette!("Unable to parse input: {}", e))?;

    let result = parsed
        .iter()
        .fold((Status::Run, 0), |(status, acc), ins| match ins {
            Instruction::Mul(a, b) => match status {
                Status::Run => (status, acc + a * b),
                Status::Skip => (status, acc),
            },
            Instruction::Do => (Status::Run, acc),
            Instruction::Dont => (Status::Skip, acc),
        })
        .1;

    Ok(result.to_string())
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::i32, tag(","), complete::i32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
