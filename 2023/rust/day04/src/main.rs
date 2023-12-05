use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../input/day_04.txt");
    println!("Part 1: {}", part1(input));
}

fn parse_nums(nums: &str) -> HashSet<u32> {
    nums.trim()
        .split(" ")
        .filter_map(|num| num.parse::<u32>().ok())
        .collect()
}

struct Card {
    id: u32,
    left: HashSet<u32>,
    right: HashSet<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (card, nums) = value
            .split_once(":")
            .expect("Card #: should prefix list of numbers");
        let id = card
            .split(" ")
            .last()
            .expect("there should be a card id")
            .parse::<u32>()
            .expect("should be a number");
        let (left, right) = nums
            .split_once("|")
            .expect("number lists should be separated with |");
        let left = parse_nums(left);
        let right = parse_nums(right);
        Card { id, left, right }
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let card = Card::from(line);
            let matches = card
                .left
                .intersection(&card.right)
                .collect::<Vec<_>>()
                .len();
            if matches > 0 {
                1 << matches - 1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 13);
    }
}
