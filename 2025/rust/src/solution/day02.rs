use crate::solution::Solution;

pub struct Day02;

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|range| {
            let (start, end) = range
                .split_once("-")
                .expect("range will be deliminated by '-'");
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<_>()
}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let ranges = parse(input);

        let total_ids = ranges
            .into_iter()
            .map(|(start, end)| {
                let mut ids: Vec<u64> = Vec::new();
                for n in start..=end {
                    let s = format!("{n}");
                    let l = s.len();
                    let (left, right) = s.split_at(l / 2);
                    if left == right {
                        ids.push(n);
                    }
                }
                ids
            })
            .flatten()
            .sum::<u64>();

        format!("{total_ids}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        let expected = "1227775554";
        let actual = Day02.part1(INPUT);
        assert_eq!(expected, actual);
    }
}
