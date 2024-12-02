use std::error::Error;

use itertools::Itertools;

use crate::common::Part;

fn report_safe(report: &[i32]) -> bool {
    // monotonicity check. monotonicity is sufficient here (even though it will allow
    // differences of 0) since we'll do the differences in the next step
    let orderings = report
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.cmp(b))
        .collect::<Vec<_>>();

    if !orderings.iter().all_equal() {
        return false;
    }

    // rate check. symmetric differences must be within [1, 3]
    report
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.abs_diff(*b))
        .all(|diff| (1..=3).contains(&diff))
}

// stupid O(n * m^2) solution...
// on the other hand it still runs pretty fast. who's to say

fn report_variants(report: &[i32]) -> Vec<Vec<i32>> {
    let mut res = Vec::new();

    for i in 0..report.len() {
        let mut entry = Vec::from(report);
        entry.remove(i);
        res.push(entry);
    }

    res
}

fn report_safe_pt2(report: &[i32]) -> bool {
    if report_safe(report) {
        // can early exit here since removing no entries is a solution
        return true;
    }

    report_variants(report)
        .iter()
        .any(|report| report_safe(report))
}

pub(crate) fn day_02(input: &str, part: Part) -> Result<String, Box<dyn Error>> {
    let reports = input
        .lines()
        .map(|line| line.split_whitespace().map(|col| col.parse()).collect())
        .collect::<Result<Vec<Vec<i32>>, _>>()?;

    Ok(reports
        .iter()
        .map(|report| part.choose(report_safe, report_safe_pt2, report))
        .filter(|x| *x)
        .count()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn example_input() {
        assert_eq!(&day_02(EXAMPLE_INPUT, Part::PartOne).unwrap(), "2");
        assert_eq!(&day_02(EXAMPLE_INPUT, Part::PartTwo).unwrap(), "4");
    }

    #[test]
    fn example_rows() {
        assert!(report_safe(&[7, 6, 4, 2, 1]));
        assert!(!report_safe(&[1, 2, 7, 8, 9]));
        assert!(!report_safe(&[9, 7, 6, 2, 1]));
        assert!(!report_safe(&[1, 3, 2, 4, 5]));
        assert!(!report_safe(&[8, 6, 4, 4, 1]));
        assert!(report_safe(&[1, 3, 6, 7, 9]));

        assert!(report_safe_pt2(&[7, 6, 4, 2, 1]));
        assert!(!report_safe_pt2(&[1, 2, 7, 8, 9]));
        assert!(!report_safe_pt2(&[9, 7, 6, 2, 1]));
        assert!(report_safe_pt2(&[1, 3, 2, 4, 5]));
        assert!(report_safe_pt2(&[8, 6, 4, 4, 1]));
        assert!(report_safe_pt2(&[1, 3, 6, 7, 9]));
    }

    #[cfg(feature = "regression")]
    #[test]
    fn regression() {
        assert_eq!(
            &day_02(include_str!("../inputs/02.txt"), Part::PartOne).unwrap(),
            "524"
        );
        assert_eq!(
            &day_02(include_str!("../inputs/02.txt"), Part::PartTwo).unwrap(),
            "569"
        );
    }
}
