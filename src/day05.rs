use std::{
    collections::{HashMap, HashSet},
    error::Error,
    hash::RandomState,
};

use itertools::Itertools;

use crate::common::Part;

fn gen_digraph(input: &str) -> Result<HashMap<u32, HashSet<u32>>, Box<dyn Error>> {
    // page -> set of following pages
    let mut digraph: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in input.lines() {
        if let Some((former, latter)) = line.split_once('|') {
            let former = former.parse()?;
            let latter = latter.parse()?;

            digraph.entry(former).or_default().insert(latter);
        }
    }

    Ok(digraph)
}

fn toposort(digraph: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut order = Vec::new();
    let mut unvisited: HashSet<u32, RandomState> = HashSet::from_iter(digraph.keys().copied());

    fn toposort_visit(
        digraph: &HashMap<u32, HashSet<u32>>,
        order: &mut Vec<u32>,
        unvisited: &mut HashSet<u32>,
        node: u32,
    ) {
        if !unvisited.contains(&node) {
            return;
        }

        for outgoing_edge in digraph.get(&node).unwrap() {
            toposort_visit(digraph, order, unvisited, *outgoing_edge);
        }

        unvisited.remove(&node);
        order.push(node);
    }

    while let Some(node) = unvisited.iter().copied().next() {
        toposort_visit(digraph, &mut order, &mut unvisited, node);
    }

    order.reverse();

    order
}

fn is_valid(update_pages: &[u32], digraph: &HashMap<u32, HashSet<u32>>) -> bool {
    for (former, latter) in update_pages.iter().copied().tuple_windows() {
        if !digraph
            .get(&former)
            .map(|h| h.contains(&latter))
            .unwrap_or(false)
        {
            return false;
        }
    }

    true
}

pub(crate) fn day_05(input: &str, part: Part) -> Result<String, Box<dyn Error>> {
    let digraph = gen_digraph(input)?;

    let mut sum = 0;
    'line: for line in input.lines() {
        if line.contains(',') {
            let mut update_pages = line
                .split(',')
                .map(|i| i.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()?;

            match part {
                Part::PartOne => {
                    if !is_valid(&update_pages, &digraph) {
                        continue 'line;
                    }

                    sum += update_pages[update_pages.len() / 2];
                }
                Part::PartTwo => {
                    if is_valid(&update_pages, &digraph) {
                        continue 'line;
                    }

                    // need to toposort anew on each iteration since the entire graph
                    // is not acyclic (but each relevant component is)
                    let mut trim = digraph.clone();
                    trim.retain(|k, _| update_pages.contains(k));
                    let toposort = toposort(&trim);

                    let order = toposort
                        .iter()
                        .copied()
                        .enumerate()
                        .map(|(a, b)| (b, a))
                        .collect::<HashMap<_, _>>();

                    update_pages.sort_unstable_by(|a, b| {
                        order
                            .get(a)
                            .unwrap_or(&usize::MAX)
                            .cmp(order.get(b).unwrap_or(&usize::MAX))
                    });
                    sum += update_pages[update_pages.len() / 2];
                }
            }
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"47|53
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

    #[test]
    fn example_input() {
        assert_eq!(&day_05(EXAMPLE_INPUT, Part::PartOne).unwrap(), "143");
        assert_eq!(&day_05(EXAMPLE_INPUT, Part::PartTwo).unwrap(), "123");
    }

    #[cfg(feature = "regression")]
    #[test]
    fn regression() {
        assert_eq!(
            &day_05(include_str!("../inputs/05.txt"), Part::PartOne).unwrap(),
            "6034"
        );
        assert_eq!(
            &day_05(include_str!("../inputs/05.txt"), Part::PartTwo).unwrap(),
            "6305"
        );
    }
}
