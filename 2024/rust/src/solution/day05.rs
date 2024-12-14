use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use super::Solution;

pub struct Day05;

type OrderingRules = HashMap<u32, HashSet<u32>>;

fn ordering_rules_from_lines(lines: Vec<&str>) -> OrderingRules {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    lines.iter().for_each(|line| {
        let (x, y) = line.split_once("|").expect("rule should be in format X|Y");
        let x = x.parse::<u32>().expect("should be valid integer");
        let y = y.parse::<u32>().expect("should be valid integer");
        rules
            .entry(x)
            .and_modify(|val| {
                val.insert(y);
            })
            .or_insert(HashSet::from([y]));
    });
    rules
}

fn parse(input: &str) -> (OrderingRules, Vec<Vec<u32>>) {
    let lines = input.lines().collect::<Vec<_>>();
    let (split_point, _) = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line.is_empty())
        .expect("should find empty line in middle of input");

    let (rules, updates) = lines.split_at(split_point);

    let rules = ordering_rules_from_lines(rules.to_vec());
    let mut updates = updates.iter();
    updates.next(); // pop empty line
    let updates = updates
        .map(|line| {
            line.split(",")
                .filter_map(|num| num.parse::<u32>().ok())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    (rules, updates)
}

fn is_valid(update: &Vec<u32>, ordering_rules: &OrderingRules) -> bool {
    let mut seen: HashSet<u32> = HashSet::new();
    for page in update {
        // get all the pages that should be printed after this page from the ordering_rules
        if let Some(after) = ordering_rules.get(page) {
            if seen.intersection(after).count() > 0 {
                return false;
            }
        }
        seen.insert(page.clone());
    }
    true
}

fn fix_update(mut update: Vec<u32>, ordering_rules: &OrderingRules) -> Vec<u32> {
    // We can sort the list based on ordering rules
    update.sort_by(|a, b| {
        // check if b is in a_next
        if let Some(a_next) = ordering_rules.get(a) {
            if a_next.contains(b) {
                // b should come after a
                return Ordering::Less;
            }
        }

        // check if a is in b_next
        if let Some(b_next) = ordering_rules.get(b) {
            if b_next.contains(a) {
                // a should come after b
                return Ordering::Greater;
            }
        }

        unimplemented!("If we reach this, then we have ordering rules that are transitive")
    });
    update
}

impl Solution for Day05 {
    fn part1(&self, input: &str) -> String {
        let (ordering_rules, updates) = parse(input);

        let sum = updates
            .iter()
            .filter(|update| is_valid(update, &ordering_rules))
            .map(|update| update[update.len() / 2])
            .sum::<u32>();

        format!("{sum}")
    }

    fn part2(&self, input: &str) -> String {
        let (ordering_rules, updates) = parse(input);

        let sum = updates
            .into_iter()
            .filter(|update| !is_valid(update, &ordering_rules))
            .map(|update| {
                let new_order = fix_update(update, &ordering_rules);
                new_order[new_order.len() / 2]
            })
            .sum::<u32>();

        format!("{sum}")
    }
}
