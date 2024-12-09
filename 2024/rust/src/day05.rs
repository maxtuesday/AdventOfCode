use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

pub struct Day05;

#[derive(Debug)]
struct OrderingRules {
    rules: HashMap<u32, HashSet<u32>>,
}

impl OrderingRules {
    fn from(lines: Vec<&str>) -> Self {
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
        Self { rules }
    }
}

fn parse(input: &str) -> (OrderingRules, Vec<Vec<u32>>) {
    let lines = input.lines().collect::<Vec<_>>();
    let (split_point, _) = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line.is_empty())
        .expect("should find empty line in middle of input");

    let (rules, updates) = lines.split_at(split_point);

    let rules = OrderingRules::from(rules.to_vec());
    let mut updates= updates.iter();
    updates.next(); // pop empty line
    let updates = updates.map(|line| {
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
        if let Some(after) = ordering_rules.rules.get(page) {
            let intersection_count = seen.intersection(after).count();
            if intersection_count > 0 {
                return false;
            }
        }
        seen.insert(page.clone());
    }
    true
}

impl Solution for Day05 {
    fn part1(&self, input: &str) -> String {
        let (ordering_rules, updates) = parse(input);

        let sum = updates.iter().filter(|update| {
            is_valid(update, &ordering_rules)
        }).map(|update| {
            let l = update.len();
            update[l/2]
        }).sum::<u32>();

        format!("{sum}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}
