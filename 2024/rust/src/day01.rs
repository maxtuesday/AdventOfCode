use std::iter::zip;

use crate::solution::Solution;

pub struct Day01;

struct Lists {
    left: Vec<u32>,
    right: Vec<u32>,
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        assert!(nums.len() == 2);

        left.push(nums[0]);
        right.push(nums[1]);
    }

    // sort left and right
    left.sort();
    right.sort();

    zip(left.into_iter(), right.into_iter()).collect()
}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let nums = parse(input);
        let sum = nums.iter().map(|pair| pair.0.abs_diff(pair.1)).sum::<u32>();
        format!("{sum}")
    }

    fn part2(&self, input: &str) -> String {
        todo!("Part 2")
    }
}
