use std::collections::HashMap;

const TARGET_COMPOUNDS: &str = r#"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1"#;

pub fn process_part_1(input: &str) -> String {
    let aunts = parse_input(input);
    let target_compounds = parse_expected_compounds(TARGET_COMPOUNDS);
    find_aunt_part_1(aunts, target_compounds).unwrap().id.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let aunts = parse_input(input);
    let target_compounds = parse_expected_compounds(TARGET_COMPOUNDS);
    find_aunt_part_2(aunts, target_compounds).unwrap().id.to_string()
}

#[derive(Clone)]
struct Aunt {
    id: i32,
    compounds: HashMap<String, i32>,
}

fn parse_input(input: &str) -> Vec<Aunt> {
    let aunts = input.lines()
        .map(|line| {
            let line_parts = line.split_once(": ").unwrap();

            let id = line_parts.0.split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let compounds = line_parts.1.split(", ")
                .map(|token| {
                    let parts = token.split(": ").collect::<Vec<_>>();
                    let key = parts[0].to_string();
                    let value = parts[1].parse::<i32>().unwrap();
                    (key, value)
                })
                .collect::<HashMap<String, i32>>();

            Aunt { id, compounds }
        })
        .collect::<Vec<_>>();
    aunts
}

fn parse_expected_compounds(input: &str) -> HashMap<String, i32> {
    input.lines()
        .map(|line| {
            let tokens = line.split(": ").collect::<Vec<_>>();
            (tokens[0].to_string(), tokens[1].parse::<i32>().unwrap())
        })
        .collect::<HashMap<String, i32>>()
}

fn find_aunt_part_1(aunts: Vec<Aunt>, target_compounds: HashMap<String, i32>) -> Option<Aunt> {
    aunts.into_iter()
        .find(|aunt| {
            let matching_compounds = aunt.compounds.iter()
                .filter(|(compound, count)| {
                    let target_count = target_compounds.get(compound.as_str()).unwrap();
                    *count == target_count
                })
                .count();
            matching_compounds == aunt.compounds.len()
        })
}

fn find_aunt_part_2(aunts: Vec<Aunt>,  target_compounds: HashMap<String, i32>) -> Option<Aunt> {
    aunts.into_iter()
        .find(|aunt| {
            let matching_compounds = aunt.compounds.iter()
                .filter(|(compound, count)| {
                    let target_count = target_compounds.get(compound.as_str()).unwrap();
                    match compound.as_str() {
                        "cats" | "trees" => *count > target_count,
                        "pomeranians" | "goldfish" => *count < target_count,
                        _ => *count == target_count
                    }
                })
                .count();
            matching_compounds == aunt.compounds.len()
        })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./../../input/16/input.txt").unwrap();
        assert_eq!(process_part_1(&input), "40")
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./../../input/16/input.txt").unwrap();
        assert_eq!(process_part_2(&input), "241")
    }
}
