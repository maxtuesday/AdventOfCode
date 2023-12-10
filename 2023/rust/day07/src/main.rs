use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day_07.txt");
    println!("Part 1: {}", part1(input));
}

// Ordered by rank
#[derive(Debug, Ord, PartialEq, Eq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_hand_type(cards: &str) -> HandType {
    let card_counts: HashMap<char, u32> = cards.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    // check for the different hand types
    // check max card count:
    let max_count = card_counts
        .iter()
        .max_by(|x, y| x.1.cmp(y.1))
        .expect("cards should not be empty");
    match max_count.1 {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            // Need to check if the other two cards are a pair or different.
            if card_counts.len() == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            // Need to check if there are two pairs or just one.
            if card_counts.len() == 3 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        1 => HandType::HighCard,
        _ => unimplemented!("we have an empty hand!"),
    }
}

fn letter_card_to_num(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).expect("should be a digit"),
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: String,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: &str, bid: u32) -> Self {
        Self {
            cards: String::from(cards),
            bid,
            hand_type: get_hand_type(cards),
        }
    }
}

fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| {
        // Compare HandType rank
        if a.hand_type != b.hand_type {
            return a.hand_type.cmp(&b.hand_type);
        }
        // Otherwise, compare by card value rank.
        for (card_a, card_b) in a.cards.chars().zip(b.cards.chars()) {
            if card_a == card_b {
                continue;
            }
            let a_value = letter_card_to_num(card_a);
            let b_value = letter_card_to_num(card_b);
            return a_value.cmp(&b_value);
        }
        std::cmp::Ordering::Equal
    })
}

fn part1(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| {
            let tokens = line.split(" ").collect::<Vec<_>>();
            let cards = tokens[0];
            let bid = tokens[1]
                .parse::<u32>()
                .expect("cards should be followed by a number");
            Hand::new(cards, bid)
        })
        .collect::<Vec<_>>();
    sort_hands(&mut hands);
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank as u32 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6440);
    }

    #[test]
    fn test_get_hand_type() {
        assert_eq!(get_hand_type("AAAAA"), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("AA8AA"), HandType::FourOfAKind);
        assert_eq!(get_hand_type("23332"), HandType::FullHouse);
        assert_eq!(get_hand_type("TTT98"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("23432"), HandType::TwoPair);
        assert_eq!(get_hand_type("A23A4"), HandType::OnePair);
        assert_eq!(get_hand_type("23456"), HandType::HighCard);
    }

    #[test]
    fn test_sort_by() {
        let mut hands = vec![
            Hand::new("32T3K", 765),
            Hand::new("T55J5", 684),
            Hand::new("KK677", 28),
            Hand::new("KTJJT", 220),
            Hand::new("QQQJA", 483),
        ];
        let expected = vec![
            Hand::new("32T3K", 765),
            Hand::new("KTJJT", 220),
            Hand::new("KK677", 28),
            Hand::new("T55J5", 684),
            Hand::new("QQQJA", 483),
        ];
        sort_hands(&mut hands);
        assert_eq!(hands, expected);
    }
}
