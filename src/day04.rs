use std::error::Error;

use itertools::Itertools;

use crate::common::Part;

fn check_part1<'a, I: IntoIterator<Item = &'a char>>(slice: I) -> bool {
    let tuple: (char, char, char, char) =
        slice.into_iter().copied().take(4).collect_tuple().unwrap();

    tuple == ('X', 'M', 'A', 'S') || tuple == ('S', 'A', 'M', 'X')
}

pub(crate) fn day_04(input: &str, part: Part) -> Result<String, Box<dyn Error>> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    match part {
        Part::PartOne => {
            for (y, x) in (0..grid.len()).cartesian_product(0..grid[0].len()) {
                // forwards and backwards
                if x <= grid[y].len() - 4 && check_part1(&grid[y][x..x + 4]) {
                    count += 1;
                }

                // up and down
                if y <= grid.len() - 4 && check_part1(grid[y..y + 4].iter().map(|row| &row[x])) {
                    count += 1
                }

                // forward diagonal
                if x <= grid.len() - 4
                    && y <= grid.len() - 4
                    && check_part1(
                        grid[y..y + 4]
                            .iter()
                            .enumerate()
                            .map(|(offset, row)| &row[x + offset]),
                    )
                {
                    count += 1;
                }

                // reverse diagonal
                if x >= 3
                    && y <= grid.len() - 4
                    && check_part1(
                        grid[y..y + 4]
                            .iter()
                            .enumerate()
                            .map(|(offset, row)| &row[x - offset]),
                    )
                {
                    count += 1;
                }
            }
        }
        Part::PartTwo => {
            for (y, x) in (1..grid.len() - 1).cartesian_product(1..grid[0].len() - 1) {
                if grid[y][x] == 'A'
                    && ((grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M')
                        || (grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S'))
                    && ((grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M')
                        || (grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S'))
                {
                    count += 1;
                }
            }
        }
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn example_input() {
        assert_eq!(&day_04(EXAMPLE_INPUT, Part::PartOne).unwrap(), "18");
        assert_eq!(&day_04(EXAMPLE_INPUT, Part::PartTwo).unwrap(), "9");
    }

    #[cfg(feature = "regression")]
    #[test]
    fn regression() {
        assert_eq!(
            &day_04(include_str!("../inputs/04.txt"), Part::PartOne).unwrap(),
            "2344"
        );
        assert_eq!(
            &day_04(include_str!("../inputs/04.txt"), Part::PartTwo).unwrap(),
            "1815"
        );
    }
}
