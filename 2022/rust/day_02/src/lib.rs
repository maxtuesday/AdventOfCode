use std::collections::HashMap;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissor),
            _ => Err("Not a valid move".to_string())
        }
    }
}

pub fn process_part_1(input: &str) -> String {
    let rock = HashMap::from([
        (Move::Rock, 3),
        (Move::Paper, 0),
        (Move::Scissor, 6),
    ]);

    let paper = HashMap::from([
        (Move::Rock, 6),
        (Move::Paper, 3),
        (Move::Scissor, 0),
    ]);

    let scissor = HashMap::from([
        (Move::Rock, 0),
        (Move::Paper, 6),
        (Move::Scissor, 3),
    ]);

    let score = input.lines()
        .map(|line| {
            let moves = line.split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect::<Vec<Move>>();
            let op = &moves[0];
            let me = &moves[1];
            match me {
                Move::Rock => rock.get(op).unwrap() + 1,
                Move::Paper => paper.get(op).unwrap() + 2,
                Move::Scissor => scissor.get(op).unwrap() + 3,
            }
        })
        .sum::<i32>();
    score.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let lose = HashMap::from([
        (Move::Rock, 3),
        (Move::Paper, 1),
        (Move::Scissor, 2),
    ]);

    let draw = HashMap::from([
        (Move::Rock, 1),
        (Move::Paper, 2),
        (Move::Scissor, 3),
    ]);

    let win = HashMap::from([
        (Move::Rock, 2),
        (Move::Paper, 3),
        (Move::Scissor, 1),
    ]);

    let score = input.lines()
        .map(|line| {
            let moves = line.split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect::<Vec<Move>>();
            let op = &moves[0];
            let me = &moves[1];
            match me {
                Move::Rock => lose.get(op).unwrap() + 0,
                Move::Paper => draw.get(op).unwrap() + 3,
                Move::Scissor => win.get(op).unwrap() + 6,
            }
        })
        .sum::<i32>();
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1() {
        assert_eq!(process_part_1(INPUT), "15")
    }

    #[test]
    fn part2() {
        assert_eq!(process_part_2(INPUT), "12")
    }
}
