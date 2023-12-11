use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day07.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
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

fn get_hand_type(cards: &str, jokers_as_wildcards: bool) -> HandType {
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
    let hand_type = match max_count.1 {
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
    };
    // check if we need to promote any hands based on the joker card count
    let joker_count = card_counts.get(&'J').unwrap_or(&0).clone();
    if jokers_as_wildcards && joker_count > 0 {
        return promote_with_jokers(hand_type, joker_count);
    }
    // else return the existing hand type
    hand_type
}

fn promote_with_jokers(hand_type: HandType, joker_count: u32) -> HandType {
    match hand_type {
        // ABCDJ -> OnePair
        HandType::HighCard => HandType::OnePair,
        // AABCJ -> Three
        // ABCJJ -> Three
        HandType::OnePair => HandType::ThreeOfAKind,
        // AABBJ -> Full
        // AABJJ -> Four
        HandType::TwoPair => {
            if joker_count == 1 {
                HandType::FullHouse
            } else {
                HandType::FourOfAKind
            }
        }
        // ABJJJ -> Four
        // AAABJ -> Four
        HandType::ThreeOfAKind => HandType::FourOfAKind,
        // AAJJJ -> Five
        // AAAJJ -> Five
        HandType::FullHouse => HandType::FiveOfAKind,
        // AJJJJ -> Five
        // AAAAJ -> Five
        HandType::FourOfAKind => HandType::FiveOfAKind,
        // AAAAA -> Five
        // JJJJJ -> Five
        HandType::FiveOfAKind => HandType::FiveOfAKind,
    }
}

fn letter_card_to_num(card: char, jokers_as_wildcards: bool) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if jokers_as_wildcards {
                1
            } else {
                11
            }
        }
        'T' => 10,
        _ => card.to_digit(10).expect("should be a digit"),
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: String,
    bid: u32,
    hand_type: HandType,
    jokers_as_wildcards: bool,
}

impl Hand {
    fn new(line: String, jokers_as_wildcards: bool) -> Self {
        let tokens = line.split(" ").collect::<Vec<_>>();
        let cards = tokens[0];
        let bid = tokens[1]
            .parse::<u32>()
            .expect("cards should be followed by a number");
        Self {
            cards: String::from(cards),
            bid,
            hand_type: get_hand_type(cards, jokers_as_wildcards),
            jokers_as_wildcards,
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
            let a_value = letter_card_to_num(card_a, a.jokers_as_wildcards);
            let b_value = letter_card_to_num(card_b, b.jokers_as_wildcards);
            return a_value.cmp(&b_value);
        }
        std::cmp::Ordering::Equal
    })
}

fn parse(input: &str, jokers_as_wildcards: bool) -> Vec<Hand> {
    input
        .lines()
        .map(|line| Hand::new(String::from(line), jokers_as_wildcards))
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> u32 {
    let mut hands = parse(input, false);
    sort_hands(&mut hands);
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank as u32 + 1))
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut hands = parse(input, true);
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
    fn test_part2() {
        assert_eq!(part2(INPUT), 5905);
    }

    #[test]
    fn test_get_hand_type() {
        assert_eq!(get_hand_type("23456", false), HandType::HighCard);
        assert_eq!(get_hand_type("A23A4", false), HandType::OnePair);
        assert_eq!(get_hand_type("23432", false), HandType::TwoPair);
        assert_eq!(get_hand_type("TTT98", false), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("23332", false), HandType::FullHouse);
        assert_eq!(get_hand_type("AA8AA", false), HandType::FourOfAKind);
        assert_eq!(get_hand_type("AAAAA", false), HandType::FiveOfAKind);
    }

    #[test]
    fn test_get_hand_type_jokers_as_wildcards() {
        assert_eq!(get_hand_type("ABCDJ", true), HandType::OnePair);
        assert_eq!(get_hand_type("AABCJ", true), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("ABCJJ", true), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("AABBJ", true), HandType::FullHouse);
        assert_eq!(get_hand_type("AABJJ", true), HandType::FourOfAKind);
        assert_eq!(get_hand_type("ABJJJ", true), HandType::FourOfAKind);
        assert_eq!(get_hand_type("AAABJ", true), HandType::FourOfAKind);
        assert_eq!(get_hand_type("AAJJJ", true), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("AAAJJ", true), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("AJJJJ", true), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("AAAAJ", true), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("AAAAA", true), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("JJJJJ", true), HandType::FiveOfAKind);
    }

    #[test]
    fn test_sort_by() {
        let mut hands = vec![
            Hand::new(String::from("32T3K 765"), false),
            Hand::new(String::from("T55J5 684"), false),
            Hand::new(String::from("KK677 28"), false),
            Hand::new(String::from("KTJJT 220"), false),
            Hand::new(String::from("QQQJA 483"), false),
        ];
        let expected = vec![
            Hand::new(String::from("32T3K 765"), false),
            Hand::new(String::from("KTJJT 220"), false),
            Hand::new(String::from("KK677 28"), false),
            Hand::new(String::from("T55J5 684"), false),
            Hand::new(String::from("QQQJA 483"), false),
        ];
        sort_hands(&mut hands);
        assert_eq!(hands, expected);
    }
}
