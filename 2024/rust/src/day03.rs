use regex::Regex;

use crate::solution::Solution;

pub struct Day03;

fn parse(input: &str) -> Vec<&str> {
    let re = Regex::new("mul\\([0-9]+,[0-9]+\\)").expect("expected valid RegExp");
    input
        .lines()
        .flat_map(|line| re.find_iter(line).map(|m| m.as_str()))
        .collect::<Vec<_>>()
}

impl Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        let instructions = parse(input);
        
        let sum = instructions.iter().map(|inst| {
            let (_, nums) = inst.split_once("mul(").unwrap();
            let (nums, _) = nums.split_once(")").unwrap();
            let (l, r) = nums.split_once(",").unwrap();
            let l = l.parse::<i32>().expect("left: valid number");
            let r = r.parse::<i32>().expect("right: valid number");
            l * r
        }).sum::<i32>();

        format!("{sum}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}
