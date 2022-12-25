use std::collections::{HashMap, HashSet};

fn is_nice_part_1(s: &str) -> bool {
    let chars =s.chars().collect::<Vec<char>>();
    let vowels = chars.iter()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count();
    if vowels < 3 {
        return false
    }

    let pairs = chars.windows(2)
        .filter(|window| window[0] == window[1])
        .count();
    if pairs == 0 {
        return false
    }

    let bad_pairs: HashSet<&str> = HashSet::from(["ab", "cd", "pq", "xy"]);
    let bad_pair_counts = chars.windows(2)
        .filter(|window| {
            let pair: &str = &window.iter().collect::<String>();
            bad_pairs.contains(pair)
        })
        .count();
    bad_pair_counts == 0
}

pub fn process_part_1(input: &str) -> String {
    let count = input.lines()
        .filter(|line| is_nice_part_1(line))
        .count();
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
        .count();

    // - It contains at least one letter which repeats with exactly one letter between them,
    //    like xyx, abcdefeghi (efe), or even aaa.
    let split_repeats = chars.windows(3)
        .filter(|window| window[0] == window[2])
        .count();

    nonoverlapping_pairs >= 1 && split_repeats >= 1
}

pub fn process_part_2(input: &str) -> String {
    let count = input.lines()
        .filter(|line| is_nice_part_2(line))
        .count();
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
