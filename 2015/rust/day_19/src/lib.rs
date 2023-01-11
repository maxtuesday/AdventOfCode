use std::collections::{BTreeMap, BTreeSet};

pub fn process_part_1(input: &str) -> String {
    let medicine = parse_input(input);

    let mut distinct: BTreeSet<String> = BTreeSet::new();
    for (from, to_list) in medicine.replacements {
        let indexes = medicine.start_molecule
            .match_indices(from.as_str())
            .collect::<Vec<_>>();
        for (index, s) in indexes {
            for to in &to_list {
                let mut next_molecule = medicine.start_molecule.clone();
                next_molecule.replace_range(index..index + s.len(), to.as_str());
                distinct.insert(next_molecule);
            }
        }
    }

    distinct.len().to_string()
}

pub fn process_part_2(input: &str) -> String {
    // Used this solution as reference:
    // https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4h7ji/?utm_source=share&utm_medium=web2x&context=3

    let medicine = parse_input(input);
    let symbol_count = medicine.start_molecule.chars()
        .filter(|c| c.is_uppercase())
        .count();
    let rn_count = count_pattern(&medicine.start_molecule, "Rn");
    let ar_count = count_pattern(&medicine.start_molecule, "Ar");
    let y_count =medicine.start_molecule.chars()
        .filter(|c| *c == 'Y')
        .count();

    let res = symbol_count - rn_count - ar_count - (2 * y_count) - 1;
    res.to_string()
}

struct Medicine {
    replacements: BTreeMap<String, Vec<String>>,
    start_molecule: String,
}

fn parse_input(input: &str) -> Medicine {
    let mut replacements: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let lines = input.lines().collect::<Vec<&str>>();
    lines.iter().take_while(|line| **line != "")
        .for_each(|line| {
            let tokens = line.split(" => ").collect::<Vec<&str>>();
            let from = tokens[0].to_string();
            let to = tokens[1].to_string();
            replacements.entry(from)
                .and_modify(|res| res.push(to.clone()))
                .or_insert(vec![to]);
        });
    let start_molecule = lines.last().unwrap().to_string();

    Medicine { replacements, start_molecule }
}

fn count_pattern(s: &String, p: &str) -> usize {
    s.chars()
        .collect::<Vec<char>>()
        .windows(p.len())
        .map(|window| window.iter().collect::<String>())
        .filter(|window| window == p)
        .count()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    const INPUT_1: &str = r#"H => HO
H => OH
O => HH

HOH"#;

    const INPUT_2: &str = r#"H => HO
H => OH
O => HH

HOHOHO"#;

    #[test]
    fn part1_test1() {
        assert_eq!(process_part_1(INPUT_1), "4");
    }


    #[test]
    fn part1_test2() {
        assert_eq!(process_part_1(INPUT_2), "7");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./../../input/19/input.txt").unwrap();
        assert_eq!(process_part_2(&input), "195");
    }
}
