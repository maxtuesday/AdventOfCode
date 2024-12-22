use std::collections::HashSet;

fn intersection(a: &str, b: &str) -> Vec<char> {
    let set: HashSet<char> = a.chars().collect();
    b.chars().filter(|c| set.contains(&c)).collect()
}

fn get_value(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => 0
    }
}

pub fn process_part_1(input: &str) -> String {
    let score = input.lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            get_value(intersection(a, b)[0])
        })
        .sum::<u32>();
    score.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let score = input.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|parts| {
            let a: String = intersection(parts[0], parts[1]).into_iter().collect();
            get_value(intersection(a.as_str(), parts[2])[0])
        })
        .sum::<u32>();
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1() {
        assert_eq!(process_part_1(INPUT), "157");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part_2(INPUT), "70");
    }
}
