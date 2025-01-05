use std::collections::HashMap;

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

fn transform_stone(stone: u64) -> (u64, Option<u64>) {
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

fn blink(stone: u64, blinks: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }

    let key = (stone, blinks);
    if let Some(val) = cache.get(&key) {
        return *val;
    }

    let (left, right) = transform_stone(stone);
    let lc = blink(left, blinks - 1, cache);
    let rc = if let Some(r) = right {
        blink(r, blinks - 1, cache)
    } else {
        0
    };

    let res = lc + rc;
    cache.entry(key).and_modify(|val| *val = res).or_insert(res);
    res
}

fn process_blinks(input: &str, blinks: u32) -> u64 {
    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();
    input
        .split_whitespace()
        .filter_map(|num| num.parse::<u64>().ok())
        .map(|stone| blink(stone, blinks, &mut cache))
        .sum::<u64>()
}

impl Solution for Day11 {
    fn part1(&self, input: &str) -> String {
        let count = process_blinks(input, 25);
        format!("{count}")
    }

    fn part2(&self, input: &str) -> String {
        let count = process_blinks(input, 75);
        format!("{count}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "125 17";
        let expected = "55312";
        let actual = Day11.part1(input);

        assert_eq!(actual, expected);
    }
}
