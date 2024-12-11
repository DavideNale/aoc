pub fn process(_input: &str) -> miette::Result<String> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in _input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    }

    let mut score: i32 = 0;

    for num in left {
        let count = right.iter().filter(|&&x| x == num).count() as i32;
        score += num * count;
    }
    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
