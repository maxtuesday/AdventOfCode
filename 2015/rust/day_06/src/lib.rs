use std::ops::RangeInclusive;

enum Operation {
    On,
    Off,
    Toggle,
}

struct Instruction {
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
    op: Operation,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        let tokens = line.split(" ").collect::<Vec<&str>>();
        let op = match tokens[0] {
            "toggle" => Operation::Toggle,
            _ => match tokens[1] {
                "on" => Operation::On,
                "off" => Operation::Off,
                _ => panic!("Not supported operation")
            }
        };
        let ranges = tokens.iter()
            .filter(|token| token.contains(","))
            .map(|range| range.split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
            )
            .collect::<Vec<_>>();
        let x_range = ranges[0][0]..=ranges[1][0];
        let y_range = ranges[0][1]..=ranges[1][1];
        Instruction {
            x_range,
            y_range,
            op,
        }
    }).collect::<Vec<_>>()
}

pub fn process_part_1(input: &str) -> String {
    let mut lights = vec![vec![false; 1000]; 1000];
    for instruction in parse_instructions(input) {
        for y in instruction.y_range.clone() {
            for x in instruction.x_range.clone() {
                match instruction.op {
                    Operation::On => lights[y][x] = true,
                    Operation::Off => lights[y][x] = false,
                    Operation::Toggle => lights[y][x] = !lights[y][x]
                }
            }
        }
    }

    let lights_on = lights.iter()
        .flatten()
        .filter(|light| **light)
        .count();

    lights_on.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let mut lights = vec![vec![0; 1000]; 1000];
    for instruction in parse_instructions(input) {
        for y in instruction.y_range.clone() {
            for x in instruction.x_range.clone() {
                match instruction.op {
                    Operation::On => lights[y][x] += 1,
                    Operation::Off => {
                        if lights[y][x] > 0 {
                            lights[y][x] -= 1;
                        }
                    }
                    Operation::Toggle => lights[y][x] += 2
                }
            }
        }
    }

    let light_brightness = lights.iter()
        .flatten()
        .sum::<usize>();
    light_brightness.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(process_part_1("turn on 0,0 through 999,999"), "1000000");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(process_part_2("turn on 0,0 through 0,0"), "1");
    }

    #[test]
    fn part2_test2() {
        assert_eq!(process_part_2("toggle 0,0 through 999,999"), "2000000");
    }
}
