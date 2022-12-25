use std::collections::HashSet;

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

    let non_overlapping_pairs = chars.windows(2)
        .enumerate()
        .map(|(idx, window)| (idx, window.iter().collect::<String>()))
        .filter(|pair| {
            if let Some(idx) = s.rfind(pair.1.as_str()) {
                pair.0 != idx && pair.0 + 1 != idx
            } else {
                false
            }
        })
        .count();
    if non_overlapping_pairs == 0 {
        return false
    }

    let split_repeats = chars.windows(3)
        .filter(|window| window[0] == window[2])
        .count();
    split_repeats > 0
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
