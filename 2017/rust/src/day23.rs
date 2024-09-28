use std::collections::HashMap;

use crate::solution::Solution;

pub struct Day23;

#[derive(Debug)]
enum Input {
    Value(i32),
    Register(String),
}

impl Input {
    fn from(s: &str) -> Self {
        match s.parse::<i32>() {
            Ok(n) => Input::Value(n),
            Err(_) => Input::Register(s.to_string()),
        }
    }
}

#[derive(Debug)]
enum Op {
    Set { x: String, y: Input },
    Sub { x: String, y: Input },
    Mul { x: String, y: Input },
    Jnz { x: Input, y: Input },
}

fn parse(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            let x = tokens[1].to_string();
            let y = Input::from(tokens[2]);
            match tokens[0] {
                "set" => Op::Set { x, y },
                "sub" => Op::Sub { x, y },
                "mul" => Op::Mul { x, y },
                "jnz" => Op::Jnz {
                    x: Input::from(tokens[1]),
                    y,
                },
                c => unimplemented!("unknown instruction: {c}"),
            }
        })
        .collect()
}

fn get_value(input: &Input, regs: &HashMap<String, i32>) -> i32 {
    match input {
        Input::Value(val) => val,
        Input::Register(r) => regs.get(r).unwrap_or(&0),
    }
    .clone()
}

impl Solution for Day23 {
    fn part1(&self, input: &str) -> String {
        let instructions = parse(input);
        let mut regs: HashMap<String, i32> = HashMap::new();
        let mut pc: i32 = 0;

        let mut mul_count = 0;

        while 0 <= pc && pc < instructions.len() as i32 {
            match instructions.get(pc as usize).unwrap() {
                Op::Set { x, y } => {
                    let value = get_value(y, &regs);
                    regs.entry(x.clone())
                        .and_modify(|v| *v = value)
                        .or_insert(value);
                }
                Op::Sub { x, y } => {
                    let value = get_value(y, &regs);
                    regs.entry(x.clone())
                        .and_modify(|v: &mut i32| *v -= value)
                        .or_insert(value);
                }
                Op::Mul { x, y } => {
                    let value = get_value(y, &regs);
                    regs.entry(x.clone())
                        .and_modify(|v| *v *= value)
                        .or_insert(value);
                    mul_count += 1;
                }
                Op::Jnz { x, y } => {
                    let x_val = get_value(x, &regs);
                    let y_val = get_value(y, &regs);
                    if x_val != 0 {
                        pc += y_val;
                        pc -= 1; // cancel out pc += 1 at the end of the loop
                    }
                }
            }
            pc += 1;
        }

        return mul_count.to_string();
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}
