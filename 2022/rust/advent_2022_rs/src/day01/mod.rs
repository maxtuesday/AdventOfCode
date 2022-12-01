use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Solution;

impl Solution {
    fn solve_part_1() {
        if let Ok(lines) = read_lines("./../../inputs/01/input.txt") {
            let valid_lines: Vec<_> = lines
                .into_iter()
                .filter_map(|line| line.ok())
                .collect();
            let mut sum = 0;
            let mut max = 0;
            for line in valid_lines {
                if line == "" {
                    if sum > max {
                        max = sum;
                    }
                    sum = 0;
                } else {
                    sum += line.parse::<i32>().unwrap()
                }
            }
            println!("Part 1: {:?}", max);
        }
    }

    fn solve_part_2() {
        if let Ok(lines) = read_lines("./../../inputs/01/input.txt") {
            let valid_lines: Vec<_> = lines
                .into_iter()
                .filter_map(|line| line.ok())
                .collect();
            let mut sums: Vec<i32> = vec![];
            let mut sum = 0;
            for line in valid_lines {
                if line == "" {
                    sums.push(sum);
                    sum = 0;
                } else {
                    sum += line.parse::<i32>().unwrap()
                }
            }
            sums.sort();
            let ans: i32 = sums[sums.len()-3..].into_iter().sum();
            println!("Part 2: {:?}", ans);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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