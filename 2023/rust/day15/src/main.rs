use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day15.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input.split(",").map(hash).sum()
}

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

fn part2(input: &str) -> u32 {
    hashmap(input)
        .iter()
        .map(|(box_number, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(i, lens)| (box_number + 1) * (i as u32 + 1) * lens.focal_length)
                .sum::<u32>()
        })
        .sum()
}

#[derive(Debug)]
struct Lens {
    id: String,
    focal_length: u32,
}

fn hashmap(input: &str) -> HashMap<u32, Vec<Lens>> {
    let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();
    input.split(",").for_each(|word| {
        if let Some((id, focal_length)) = word.split_once("=") {
            let key = hash(id);
            let focal_length = focal_length
                .parse::<u32>()
                .expect("focal length should be a valid digit");
            boxes
                .entry(key)
                .and_modify(|v| {
                    // search for lens
                    match v.iter().position(|lens| lens.id == id) {
                        Some(i) => {
                            v[i].focal_length = focal_length;
                        }
                        None => {
                            v.push(Lens {
                                id: id.to_string(),
                                focal_length,
                            });
                        }
                    }
                })
                .or_insert(vec![Lens {
                    id: id.to_string(),
                    focal_length,
                }]);
        } else if let Some((id, _)) = word.split_once("-") {
            let key = hash(id);
            boxes.entry(key).and_modify(|v| {
                match v.iter().position(|lens| lens.id == id) {
                    Some(i) => {
                        v.remove(i);
                    }
                    None => {}
                };
            });
        } else {
            unimplemented!("invalid syntax: {word}");
        }
    });
    boxes
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1(INPUT), 1320);
    }

    #[test]
    fn test_part2_ex() {
        assert_eq!(part2(INPUT), 145);
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}
