use std::{cmp::min, collections::HashSet};

fn main() {
    let input = include_str!("../../../input/day04.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_nums(nums: &str) -> HashSet<u32> {
    nums.trim()
        .split(" ")
        .filter_map(|num| num.parse::<u32>().ok())
        .collect()
}

struct Card {
    matches: usize,
}

impl Card {
    fn score(&self) -> usize {
        if self.matches > 0 {
            1 << self.matches - 1
        } else {
            0
        }
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (_, nums) = value
            .split_once(":")
            .expect("Card #: should prefix list of numbers");
        let (left, right) = nums
            .split_once("|")
            .expect("number lists should be separated with |");
        let left = parse_nums(left);
        let right = parse_nums(right);
        let matches = left.intersection(&right).collect::<Vec<_>>().len();
        Card { matches }
    }
}

fn part1(input: &str) -> usize {
    input.lines().map(|line| Card::from(line).score()).sum()
}

fn part2(input: &str) -> usize {
    let cards = input.lines().map(Card::from).collect::<Vec<_>>();
    let mut card_counts: Vec<usize> = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let range_right = min(cards.len() - i - 1, card.matches);
        for m in 1..=range_right {
            card_counts[i + m] += card_counts[i];
        }
    }

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(INPUT), 30);
    }
}
