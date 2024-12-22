use std::iter::zip;

use super::Solution;

pub struct Day01;

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
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
    (left, right)
}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let (mut left, mut right) = parse(input);

        // sort left and right
        left.sort();
        right.sort();
        let nums = zip(left.into_iter(), right.into_iter()).collect::<Vec<_>>();
        let sum = nums.iter().map(|pair| pair.0.abs_diff(pair.1)).sum::<u32>();

        format!("{sum}")
    }

    fn part2(&self, input: &str) -> String {
        let (left, right) = parse(input);

        let score = left.iter().map(|num| {
            num * (right.iter().filter(|rn| *rn == num).count() as u32)
        }).sum::<u32>();

        format!("{score}")
    }
}
