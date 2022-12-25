use std::collections::{HashMap, HashSet};

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

fn is_nice_part_1(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut contains_double = false;
    let bad_pairs: HashSet<&str> = HashSet::from(["ab", "cd", "pq", "xy"]);
    let chars: Vec<char> = s.chars().collect();
    for window in chars.windows(2) {
        if is_vowel(window[0]) {
            vowel_count += 1
        }
        if window[0] == window[1] {
            contains_double = true
        }
        let pair: &str = &window.iter().collect::<String>();
        if bad_pairs.contains(pair) {
            return false;
        }
    }
    if is_vowel(chars[chars.len() - 1]) {
        vowel_count += 1
    }
    vowel_count >= 3 && contains_double
}

pub fn process_part_1(input: &str) -> String {
    let count = input.lines()
        .filter(|line| is_nice_part_1(line))
        .collect::<Vec<&str>>()
        .len();
    count.to_string()
}

fn is_nice_part_2(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<char>>();

    // - It contains a pair of any two letters that appears at
    //   least twice in the string without overlapping,
    //    like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    let pairs = chars.windows(2)
        .enumerate()
        .map(|(idx, window)| {
            let pair = window.iter().collect::<String>();
            (pair, idx)
        })
        .fold(HashMap::new(), |mut map: HashMap<String, Vec<usize>>, val| {
            map.entry(val.0)
                .and_modify(|indices| indices.push(val.1))
                .or_insert(Vec::from([val.1]));
            map
        });

    let nonoverlapping_pairs = pairs.iter()
        .filter(|(_, idxs)| {
            idxs.len() >= 2 && idxs[0] + 1 != *idxs.last().unwrap()
        })
        .collect::<Vec<_>>()
        .len();

    // - It contains at least one letter which repeats with exactly one letter between them,
    //    like xyx, abcdefeghi (efe), or even aaa.
    let split_repeats = chars.windows(3)
        .filter(|window| window[0] == window[2])
        .collect::<Vec<_>>()
        .len();

    nonoverlapping_pairs >= 1 && split_repeats >= 1
}

pub fn process_part_2(input: &str) -> String {
    let count = input.lines()
        .filter(|line| is_nice_part_2(line))
        .collect::<Vec<&str>>()
        .len();
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART_1: &str = "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";

    #[test]
    fn part1() {
        assert_eq!(process_part_1(INPUT_PART_1), "2");
    }

    const INPUT_PART_2: &str = "qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy
aaa";

    #[test]
    fn part2() {
        assert_eq!(process_part_2(INPUT_PART_2), "2");
    }

    #[test]
    fn part2_1() {
        assert_eq!(process_part_2("aaa\nbbbb"), "1");
    }
}
