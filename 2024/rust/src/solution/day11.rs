use super::Solution;

pub struct Day11;

fn count_digits(mut num: u64) -> u32 {
    let mut count = 0;
    while num > 0 {
        count += 1;
        num /= 10;
    }
    count
}

fn blink(stone: u64) -> (u64, Option<u64>) {
    // Rules:
    // - If the stone is engraved with the number 0,
    //  it is replaced by a stone engraved with the number 1.
    if stone == 0 {
        return (1, None);
    }

    // - If the stone is engraved with a number that has an even number of digits,
    //  it is replaced by two stones.
    //  The left half of the digits are engraved on the new left stone,
    //  and the right half of the digits are engraved on the new right stone.
    //  (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
    let num_digits = count_digits(stone);
    if num_digits % 2 == 0 {
        let mid = num_digits / 2;
        let factor = 10u64.pow(mid);

        let left = stone / factor;
        let right = stone % factor;
        return (left, Some(right));
    }

    // - If none of the other rules apply, the stone is replaced by a new stone;
    //  the old stone's number multiplied by 2024 is engraved on the new stone.
    (stone * 2024, None)
}

impl Solution for Day11 {
    fn part1(&self, input: &str) -> String {
        let mut stones = input
            .split_whitespace()
            .filter_map(|num| num.parse::<u64>().ok())
            .collect::<Vec<_>>();

        let res = (0..25).fold(stones, |acc, _|  {
            acc.into_iter().flat_map(|stone| {
                let (l, r) = blink(stone);
                match r {
                    Some(r) => vec![l, r],
                    None => vec![l],
                }
            }).collect::<Vec<_>>()
        });

        format!("{}", res.len())
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_blink_6_times() {
        let input = "125 17";
        let expected = "55312";
        let actual = Day11.part1(input);

        assert_eq!(actual, expected);
    }
}
