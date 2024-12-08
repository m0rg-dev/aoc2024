use std::error::Error;

use crate::common::Part;

// I'm sure there's some kind of cool ring theory thing you can do on this one. No idea what it is.

// Surprised this approach is fast enough but it actually works. I assume the early stop if acc > target
// is doing a lot of lifting here.

fn proc_row(acc: u64, ops: &[u64], target: u64, part: Part) -> bool {
    if ops.is_empty() {
        return acc == target;
    }

    if acc > target {
        return false;
    }

    if part == Part::PartTwo {
        let concat = acc * 10u64.pow(ops[0].ilog10() + 1) + ops[0];
        if proc_row(concat, &ops[1..], target, part) {
            return true;
        }
    }

    if proc_row(acc * ops[0], &ops[1..], target, part) {
        return true;
    }

    proc_row(acc + ops[0], &ops[1..], target, part)
}

pub(crate) fn day_07(input: &str, part: Part) -> Result<String, Box<dyn Error>> {
    let mut sum = 0;

    for line in input.lines() {
        let (first, rest) = line.split_once(": ").unwrap();
        let first: u64 = first.parse()?;
        let operands = rest
            .split_whitespace()
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;

        if proc_row(operands[0], &operands[1..], first, part) {
            sum += first;
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn example_input() {
        assert_eq!(&day_07(EXAMPLE_INPUT, Part::PartOne).unwrap(), "3749");
        assert_eq!(&day_07(EXAMPLE_INPUT, Part::PartTwo).unwrap(), "11387");
    }

    #[cfg(feature = "regression")]
    #[test]
    fn regression() {
        assert_eq!(
            &day_07(include_str!("../inputs/07.txt"), Part::PartOne).unwrap(),
            "28730327770375"
        );
        assert_eq!(
            &day_07(include_str!("../inputs/07.txt"), Part::PartTwo).unwrap(),
            "424977609625985"
        );
    }
}
