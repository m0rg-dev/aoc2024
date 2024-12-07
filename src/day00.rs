use std::error::Error;

use crate::common::Part;

pub(crate) fn day_00(input: &str, part: Part) -> Result<String, Box<dyn Error>> {
    Ok("".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"";

    #[test]
    fn example_input() {
        assert_eq!(&day_00(EXAMPLE_INPUT, Part::PartOne).unwrap(), "");
        assert_eq!(&day_00(EXAMPLE_INPUT, Part::PartTwo).unwrap(), "");
    }

    #[cfg(feature = "regression")]
    #[test]
    fn regression() {
        assert_eq!(
            &day_00(include_str!("../inputs/00.txt"), Part::PartOne).unwrap(),
            ""
        );
        assert_eq!(
            &day_00(include_str!("../inputs/00.txt"), Part::PartTwo).unwrap(),
            ""
        );
    }
}
