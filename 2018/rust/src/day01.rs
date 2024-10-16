use crate::solution::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let freq = input.lines().filter_map(|line| line.parse::<i32>().ok()).sum::<i32>();
        format!("{freq}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}