use crate::solution::Solution;

pub struct Day01;

fn parse(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|line| {
            let (dir, turns) = line.split_at(1);
            let turns = turns.parse::<i32>().expect("should be valid integer");
            (dir, turns)
        })
        .collect::<_>()
}

fn modulo(a: i32, b: i32) -> i32 {
    (a % b + b) % b
}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let mut pos: i32 = 50;
        let mut count = 0;

        let ops = parse(input);
        for (dir, turns) in ops {
            match dir {
                "L" => pos -= turns,
                "R" => pos += turns,
                c => unreachable!("got prefix: {c}"),
            }
            pos = modulo(pos, 100);
            if pos == 0 {
                count += 1;
            }
        }

        format!("{count}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        let expected = "3".to_string();
        let actual = Day01.part1(INPUT);
        assert_eq!(expected, actual);
    }
}
