use std::collections::HashMap;

use glam::IVec2;

enum Status {
    Free,
    Obstructed,
    Visited,
}

struct Directions {
    directions: Vec<IVec2>,
    current: usize,
}

impl Directions {
    fn new() -> Self {
        let directions = vec![
            IVec2::Y,     // Up
            IVec2::X,     // Right
            IVec2::NEG_Y, // Down
            IVec2::NEG_X, // Left
        ];

        Directions {
            directions,
            current: 0,
        }
    }

    fn next(&mut self) -> IVec2 {
        let dir = self.directions[self.current];
        self.current = (self.current + 1) % self.directions.len();
        dir
    }

    fn current(&self) -> IVec2 {
        self.directions[self.current]
    }
}

pub fn process(_input: &str) -> miette::Result<String> {
    let mut directions = Directions::new();

    let mut current = IVec2::new(0, 0);
    let (mut grid, start_pos) = _input
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars().enumerate().map(move |(x, c)| {
                let pos = IVec2::new(x as i32, -(y as i32));
                let status = match c {
                    '^' => (pos, Status::Visited, Some(pos)),
                    '.' => (pos, Status::Free, None),
                    '#' => (pos, Status::Obstructed, None),
                    _ => panic!("Unexpected character!"),
                };
                status
            })
        })
        .fold(
            (HashMap::new(), None),
            |(mut map, start), (pos, status, maybe_start)| {
                map.insert(pos, status);
                (map, start.or(maybe_start))
            },
        );

    if let Some(pos) = start_pos {
        current = pos;
    }

    dbg!(current);
    while grid.contains_key(&current) {
        let dir = directions.current();
        let next = current + dir;
        match grid.get(&next) {
            Some(cell) => match cell {
                Status::Free => {
                    current = next;
                    grid.insert(next, Status::Visited);
                }
                Status::Obstructed => {
                    directions.next();
                }
                Status::Visited => {
                    current = next;
                }
            },
            None => break,
        }
    }

    let visited = grid
        .values()
        .filter(|&status| matches!(status, Status::Visited))
        .count();

    Ok(visited.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
