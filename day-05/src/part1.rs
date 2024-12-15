use std::collections::HashMap;

pub fn process(_input: &str) -> miette::Result<String> {
    let parts: Vec<&str> = _input.split("\n\n").collect();

    let rules: Vec<(usize, usize)> = parts[0]
        .split("\n")
        .map(|line| {
            let mut split = line.split("|");
            let left = split.next().unwrap().trim().parse::<usize>().unwrap();
            let right = split.next().unwrap().trim().parse::<usize>().unwrap();
            (left, right)
        })
        .collect();

    let mut orders: HashMap<usize, Vec<usize>> = HashMap::new();
    for (x, y) in rules.iter() {
        orders.entry(*x).or_insert_with(Vec::new).push(*y);
    }

    let updates = parts[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let result: Vec<Vec<usize>> = updates
        .into_iter()
        .filter(|vec| is_safe(vec, &rules))
        .collect();

    let sum: usize = result
        .iter()
        .map(|v| {
            let middle = v.len() / 2;
            v[middle]
        })
        .sum();

    Ok(sum.to_string())
}

fn is_safe(vec: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    let mut index_map = HashMap::new();
    for (index, &val) in vec.iter().enumerate() {
        index_map.insert(val, index);
    }

    for &(x, y) in rules {
        if let (Some(&ix), Some(&iy)) = (index_map.get(&x), index_map.get(&y)) {
            if ix >= iy {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
