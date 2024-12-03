use std::error::Error;

use itertools::Itertools;
use regex::Regex;

use crate::common::Part;

pub(crate) fn day_03(input: &str, part: Part) -> Result<String, Box<dyn Error>> {
    match part {
        Part::PartOne => {
            let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

            let mut ac = 0;
            for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
                ac += a.parse::<u32>()? * b.parse::<u32>()?;
            }
            Ok(ac.to_string())
        }
        Part::PartTwo => {
            let cond_re = Regex::new(
                r"(?s:(?:^(.*?)(?:don't\(\)))|(?:(?:do\(\))(.*?)(?:don't\(\)))|(?:(?:do\(\))(.*?)$))",
            )
            .unwrap();

            let x = day_03(
                &cond_re
                    .captures_iter(input)
                    .map(|c| {
                        c.get(1)
                            .or_else(|| c.get(2))
                            .or_else(|| c.get(3))
                            .unwrap()
                            .as_str()
                    })
                    .join(""),
                Part::PartOne,
            );
            x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const EXAMPLE_INPUT_2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn example_input() {
        assert_eq!(&day_03(EXAMPLE_INPUT, Part::PartOne).unwrap(), "161");
        assert_eq!(&day_03(EXAMPLE_INPUT_2, Part::PartTwo).unwrap(), "48");
    }

    #[cfg(feature = "regression")]
    #[test]
    fn regression() {
        assert_eq!(
            &day_03(include_str!("../inputs/03.txt"), Part::PartOne).unwrap(),
            "183788984"
        );
        assert_eq!(
            &day_03(include_str!("../inputs/03.txt"), Part::PartTwo).unwrap(),
            "62098619"
        );
    }
}
