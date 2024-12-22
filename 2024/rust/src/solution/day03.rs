use regex::Regex;

use super::Solution;

pub struct Day03;

enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

impl Instruction {
    fn from(input: &str) -> Self {
        match input {
            s if s.starts_with("mul") => {
                let (_, nums) = s.split_once("mul(").unwrap();
                let (nums, _) = nums.split_once(")").unwrap();
                let (l, r) = nums.split_once(",").unwrap();
                let l = l.parse::<i32>().expect("left: valid number");
                let r = r.parse::<i32>().expect("right: valid number");
                Instruction::Mul(l, r)
            }
            s if s.starts_with("don't") => Instruction::Dont,
            s if s.starts_with("do") => Instruction::Do,
            _ => unimplemented!(),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    let re =
        Regex::new("mul\\([0-9]+,[0-9]+\\)|do\\(\\)|don't\\(\\)").expect("expected valid RegExp");
    input
        .lines()
        .flat_map(|line| re.find_iter(line).map(|m| m.as_str()))
        .map(Instruction::from)
        .collect()
}

impl Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        let instructions = parse(input);

        let sum = instructions
            .iter()
            .map(|inst| match inst {
                Instruction::Mul(l, r) => l * r,
                Instruction::Do => 0,
                Instruction::Dont => 0,
            })
            .sum::<i32>();

        format!("{sum}")
    }

    fn part2(&self, input: &str) -> String {
        let instructions = parse(input);

        let mut do_it = true;
        let sum = instructions.iter().map(|inst| {
            match inst {
                Instruction::Mul(l, r) => {
                    if do_it {
                        l * r
                    } else {
                        0
                    }
                }
                Instruction::Do => {
                    do_it = true;
                    0
                },
                Instruction::Dont => {
                    do_it = false;
                    0
                },
            }
        }).sum::<i32>();

        format!("{sum}")
    }
}
