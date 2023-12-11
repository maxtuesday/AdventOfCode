use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day08.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn parse(input: &str) -> (Vec<char>, HashMap<String, Node>) {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .expect("first line should be instructions")
        .chars()
        .collect::<Vec<_>>();
    lines.next(); // skip next line
    let nodes = lines
        .map(|line| {
            let (id, rest) = line.split_once("=").unwrap();
            let id = String::from(id.trim());
            let (left, right) = rest.split_once(",").unwrap();
            let (_, left) = left.split_once("(").unwrap();
            let (right, _) = right.split_once(")").unwrap();
            let left = left.trim();
            let right = right.trim();
            let node = Node {
                id: id.clone(),
                left: String::from(left),
                right: String::from(right),
            };
            (id, node)
        })
        .collect::<HashMap<String, Node>>();
    (instructions, nodes)
}

fn part1(input: &str) -> usize {
    let (instructions, graph) = parse(input);
    // Start at AAA and follow `instructions` until we reach ZZZ

    let target = String::from("ZZZ");
    let mut i: usize = 0;
    let len = instructions.len();
    let mut cur = String::from("AAA");
    loop {
        if cur == target {
            return i;
        }

        let next = graph
            .get(&cur)
            .expect("there should be a Node in the graph");
        match instructions[i % len] {
            'L' => cur = next.left.clone(),
            'R' => cur = next.right.clone(),
            _ => unimplemented!("we do not have any other instructions other than L or R"),
        }
        i += 1;
    }
}

fn steps_to_z(instructions: &Vec<char>, start: String, graph: &HashMap<String, Node>) -> usize {
    let mut i = 0;
    let len = instructions.len();
    let mut cur = start;
    loop {
        if cur.ends_with("Z") {
            return i;
        }

        let next = graph
            .get(&cur)
            .expect("there should be a Node in the graph");
        match instructions[i % len] {
            'L' => cur = next.left.clone(),
            'R' => cur = next.right.clone(),
            _ => unimplemented!("we do not have any other instructions other than L or R"),
        }
        i += 1;
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    // set 'a' as larger value
    if a < b {
        let tmp = a;
        a = b;
        b = tmp;
    }
    while b > 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(nums: &Vec<usize>) -> usize {
    nums.iter().fold(1, |lcm, n| lcm * n / gcd(lcm, *n))
}

fn part2(input: &str) -> usize {
    let (instructions, graph) = parse(input);

    let steps = graph
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|node| steps_to_z(&instructions, node.clone(), &graph))
        .collect::<Vec<_>>();

    // find Least Common Multiple (LCM) of all the steps
    lcm(&steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_rl() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part1_llr() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part2(input), 6);
    }
}
