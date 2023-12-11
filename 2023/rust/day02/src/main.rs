use std::{cmp::max, collections::HashMap};

fn main() {
    let input = include_str!("../../../input/day02.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => unreachable!("there should only be red, green, and blue colors"),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    rocks: HashMap<Color, usize>,
}

fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (id, rest) = line.split_once(":").unwrap();
            let (_, id) = id.split_once(" ").unwrap();

            let mut game = Game {
                id: id.parse().unwrap(),
                rocks: HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]),
            };

            rest.replace(";", ",").split(",").for_each(|rock| {
                let (count, color) = rock.trim().split_once(" ").unwrap();
                let count = count.parse::<usize>().unwrap();
                let color = Color::from(color);
                game.rocks.entry(color).and_modify(|c| *c = max(*c, count));
            });

            game
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|game| {
            game.rocks.get(&Color::Red).unwrap() <= &12
                && game.rocks.get(&Color::Green).unwrap() <= &13
                && game.rocks.get(&Color::Blue).unwrap() <= &14
        })
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|game| game.rocks.values().product::<usize>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(INPUT), 8);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(INPUT), 2286);
    }
}
