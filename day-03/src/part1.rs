use miette::miette;
use regex::Regex;

pub fn process(_input: &str) -> miette::Result<String> {
    let re =
        Regex::new(r"mul\((\d+),(\d+)\)").map_err(|e| miette!("Unable to compile regex: {}", e))?;

    let result: i32 = re
        .captures_iter(_input)
        .map(|cap| {
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )
        })
        .map(|(a, b)| a * b)
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
