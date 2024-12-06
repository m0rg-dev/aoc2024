use std::error::Error;

use crate::common::Part;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }

    fn as_vec(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct DirectionSet {
    bitset: u8,
}

impl DirectionSet {
    fn new() -> Self {
        Self { bitset: 0 }
    }

    fn insert(&mut self, direction: Direction) {
        self.bitset |= 1 << (direction as u8);
    }

    fn contains(&self, direction: Direction) -> bool {
        (self.bitset & 1 << (direction as u8)) != 0
    }

    fn is_empty(&self) -> bool {
        self.bitset == 0
    }
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty { directions: DirectionSet },
    Obstructed,
}

impl Cell {
    fn is_visited(&self) -> bool {
        match self {
            Cell::Empty { directions } => !directions.is_empty(),
            Cell::Obstructed => false,
        }
    }
}

enum PathResult {
    Escape { cover_area: usize },
    Cycle,
}

fn pathfind((mut x, mut y): (usize, usize), width: usize, grid: &mut [Cell]) -> PathResult {
    let mut direction = Direction::Up;

    loop {
        // cycle check
        if let Cell::Empty { directions } = &grid[y * width + x] {
            if directions.contains(direction) {
                return PathResult::Cycle;
            }
        }

        if let Cell::Empty { directions } = &mut grid[y * width + x] {
            directions.insert(direction);
        }

        let (dx, dy) = direction.as_vec();
        if (dx < 0 && x == 0)
            || (dx > 0 && x == (width - 1))
            || (dy < 0 && y == 0)
            || (dy > 0 && y == (grid.len() / width) - 1)
        {
            break;
        }

        let (nx, ny) = (
            x.checked_add_signed(dx).unwrap(),
            y.checked_add_signed(dy).unwrap(),
        );

        match &mut grid[ny * width + nx] {
            Cell::Empty { .. } => {
                (x, y) = (nx, ny);
            }
            Cell::Obstructed => {
                direction = direction.turn_right();
            }
        }
    }

    let mut cover_area = 0;
    // significantly faster than iter().filter(...).count() in debug mode?
    for cell in grid {
        if cell.is_visited() {
            cover_area += 1;
        }
    }

    PathResult::Escape { cover_area }
}

pub(crate) fn day_06(input: &str, part: Part) -> Result<String, Box<dyn Error>> {
    let mut position: (usize, usize) = (0, 0);

    let grid: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Cell::Empty {
                        directions: DirectionSet::new(),
                    },
                    '#' => Cell::Obstructed,
                    '^' => {
                        position = (x, y);
                        Cell::Empty {
                            directions: DirectionSet::new(),
                        }
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let width = grid[0].len();
    let mut flattened_grid = grid.iter().flatten().cloned().collect::<Vec<_>>();

    match part {
        Part::PartOne => match pathfind(position, width, &mut flattened_grid) {
            PathResult::Escape { cover_area } => Ok(cover_area.to_string()),
            PathResult::Cycle => panic!(),
        },
        Part::PartTwo => {
            // opt: any cell not visited in part1 pathfind cannot be a solution to part 2
            let mut unmod_grid = flattened_grid.clone();
            pathfind(position, width, &mut unmod_grid);

            let mut cycle_count = 0;
            for oy in 0..grid.len() {
                for ox in 0..grid[0].len() {
                    if (ox, oy) == position {
                        continue;
                    }

                    if let Cell::Obstructed = unmod_grid[oy * width + ox] {
                        continue;
                    }

                    if let Cell::Empty { directions } = unmod_grid[oy * width + ox] {
                        if directions.is_empty() {
                            continue;
                        }
                    }

                    let mut new_grid = flattened_grid.clone();
                    new_grid[oy * width + ox] = Cell::Obstructed;
                    match pathfind(position, width, &mut new_grid) {
                        PathResult::Escape { .. } => {}
                        PathResult::Cycle => {
                            cycle_count += 1;
                        }
                    }
                }
            }
            Ok(cycle_count.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn example_input() {
        assert_eq!(&day_06(EXAMPLE_INPUT, Part::PartOne).unwrap(), "41");
        assert_eq!(&day_06(EXAMPLE_INPUT, Part::PartTwo).unwrap(), "6");
    }

    #[cfg(feature = "regression")]
    #[test]
    fn regression() {
        assert_eq!(
            &day_06(include_str!("../inputs/06.txt"), Part::PartOne).unwrap(),
            "4778"
        );
        assert_eq!(
            &day_06(include_str!("../inputs/06.txt"), Part::PartTwo).unwrap(),
            "1618"
        );
    }
}
