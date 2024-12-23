use std::{collections::BTreeMap, error::Error};

use crate::common::Part;

pub(crate) fn day_01(input: &str, part: Part) -> Result<String, Box<dyn Error>> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse()?);
        right.push(split.next().unwrap().parse()?);
    }

    match part {
        Part::PartOne => {
            left.sort();
            right.sort();

            Ok(left
                .iter()
                .zip(right.iter())
                .map(|(l, r)| l.abs_diff(*r))
                .sum::<u32>()
                .to_string())
        }
        Part::PartTwo => {
            let mut frequency: BTreeMap<u32, u32> = BTreeMap::new();
            for entry in &right {
                *frequency.entry(*entry).or_default() += 1;
            }

            Ok(left
                .iter()
                .map(|l| l * frequency.get(l).copied().unwrap_or_default())
                .sum::<u32>()
                .to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn example_input() {
        assert_eq!(&day_01(EXAMPLE_INPUT, Part::PartOne).unwrap(), "11");
        assert_eq!(&day_01(EXAMPLE_INPUT, Part::PartTwo).unwrap(), "31");
    }

    #[cfg(feature = "regression")]
    #[test]
    fn regression() {
        assert_eq!(
            &day_01(include_str!("../inputs/01.txt"), Part::PartOne).unwrap(),
            "1603498"
        );
        assert_eq!(
            &day_01(include_str!("../inputs/01.txt"), Part::PartTwo).unwrap(),
            "25574739"
        );
    }
}
