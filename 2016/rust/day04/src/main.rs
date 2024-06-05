use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day04.txt");
    println!("Part 1: {}", part1(input));
}

struct Room {
    char_freqs: HashMap<char, u32>,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn from_line(line: &str) -> Self {
        // aaaaa-bbb-z-y-x-123[abxyz]
        // Split on last '-'
        // count freq of characters
        // Take last element and split on '['
        // Left is sector_id, right it checksum minus last ']'

        let (chars, rest) = line
            .rsplit_once('-')
            .expect("there must be at least one '-'");

        let mut char_freqs: HashMap<char, u32> = HashMap::new();
        for char in chars.chars() {
            match char {
                'a'..='z' => {
                    char_freqs
                        .entry(char)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
                _ => {
                    // ignore other characters
                }
            };
        }

        let (sector_id, checksum) = rest
            .split_once('[')
            .expect("there must be '[' for start of checksum");

        let sector_id = sector_id
            .parse::<u32>()
            .expect("sector_id must be a valid number");

        let checksum = checksum[..checksum.len() - 1].to_string();

        Room {
            char_freqs,
            sector_id,
            checksum,
        }
    }

    fn create_checksum(&self) -> String {
        let mut char_freqs = self.char_freqs.iter().collect::<Vec<_>>();
        char_freqs.sort_by(|a, b| match a.1.cmp(&b.1).reverse() {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            cmp => cmp,
        });
        char_freqs
            .into_iter()
            .take(5)
            .map(|(c, _)| c)
            .collect::<String>()
    }

    fn is_valid(&self) -> bool {
        self.create_checksum() == self.checksum
    }
}

fn parse(input: &str) -> Vec<Room> {
    input.lines().map(|line| Room::from_line(line)).collect()
}

fn part1(input: &str) -> u32 {
    parse(input)
        .iter()
        .filter(|room| room.is_valid())
        .map(|room| room.sector_id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]
";

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(INPUT), 1514);
    }
}
