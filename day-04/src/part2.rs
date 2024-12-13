use std::collections::HashMap;

type Direction = (i32, i32);

const DIRECTIONS: [[Direction; 2]; 4] = [
    [(1, 1), (-1, -1)],
    [(1, -1), (-1, 1)],
    [(-1, 1), (1, -1)],
    [(-1, -1), (1, 1)],
];

pub fn process(_input: &str) -> miette::Result<String> {
    let positions = _input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect::<HashMap<Direction, char>>();

    let mas = ['M', 'S'];
    let result: usize = positions
        .iter()
        .filter(|(_, c)| **c == 'A')
        .filter(|(position, _)| {
            DIRECTIONS
                .iter()
                .map(|direction| {
                    direction
                        .iter()
                        .map(|offset| {
                            positions.get(&((position.0 + offset.0, position.1 + offset.1)))
                        })
                        .enumerate()
                        .all(|(i, value)| mas.get(i) == value)
                })
                .filter(|b| *b)
                .count()
                == 2
        })
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
