use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

pub enum Order {
    Increasing,
    Decreasing,
}

#[derive(PartialEq, Eq)]
pub enum Safety {
    Safe,
    Unsafe,
}

pub fn process(_input: &str) -> miette::Result<String> {
    let (_, parsed) = parse(_input).map_err(|e| miette!("Parse error {}", e))?;
    let n = parsed
        .iter()
        .map(|report| is_safe(report))
        .filter(|safety| *safety == Safety::Safe)
        .count();
    Ok(n.to_string())
}

fn is_safe(report: &Vec<i32>) -> Safety {
    let mut order: Option<Order> = None;
    for (a, b) in report.iter().tuple_windows() {
        let delta = a - b;
        match delta.abs() {
            1..=3 => match delta.signum() {
                1 => match order {
                    Some(Order::Increasing) => continue,
                    Some(Order::Decreasing) => return Safety::Unsafe,
                    None => order = Some(Order::Increasing),
                },
                -1 => match order {
                    Some(Order::Decreasing) => continue,
                    Some(Order::Increasing) => return Safety::Unsafe,
                    None => order = Some(Order::Decreasing),
                },
                0 => return Safety::Unsafe,
                _ => panic!("The sign can only be +1,-1,0."),
            },
            _ => return Safety::Unsafe,
        }
    }
    Safety::Safe
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline, separated_list1(space1, complete::i32))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
