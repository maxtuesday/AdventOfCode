use std::collections::HashSet;

use crate::solution::Solution;

pub struct Day01;

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect()
}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let freq = parse(input).iter().sum::<i32>();
        format!("{freq}")
    }

    fn part2(&self, input: &str) -> String {
        let deltas = parse(input);

        let mut freq = 0;
        let mut idx = 0;
        let mut seen = HashSet::from([0]);

        loop {
            freq += deltas[idx % deltas.len()];
            if let Some(ans) = seen.get(&freq) {
                return format!("{ans}");
            }
            seen.insert(freq);
            idx += 1;
        }
    }
}
