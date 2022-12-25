use std::fs;

pub struct Solution;

impl Solution {
    fn solve_part_1() {
        let text = fs::read_to_string("./../../input/01/input.txt").unwrap();
        let calories = text.split("\n\n")
            .map(|group| {
                group.lines()
                    .map(|num| num.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .max()
            .unwrap();
        println!("Part 1: {:?}", calories);
    }

    fn solve_part_2() {
        let text = fs::read_to_string("./../../input/01/input.txt").unwrap();
        let mut calories: Vec<_> = text.split("\n\n")
            .map(|group| {
                group.lines()
                    .map(|num| num.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .collect();
        calories.sort_by(|a, b| b.cmp(a));
        let ans = calories.iter().take(3).sum::<u32>();
        println!("Part 2: {:?}", ans);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        Solution::solve_part_1();
    }

    #[test]
    fn part2() {
        Solution::solve_part_2();
    }
}