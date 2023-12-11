use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day08.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug)]
struct Node {
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
                left: String::from(left),
                right: String::from(right),
            };
            (id, node)
        })
        .collect::<HashMap<String, Node>>();
    (instructions, nodes)
}

fn steps_to(
    instructions: &Vec<char>,
    graph: &HashMap<String, Node>,
    start: &str,
    ends_with: &str,
) -> usize {
    let mut i = 0;
    let len = instructions.len();
    let mut cur = String::from(start);
    loop {
        if cur.ends_with(ends_with) {
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

fn part1(input: &str) -> usize {
    let (instructions, graph) = parse(input);
    // Start at AAA and follow `instructions` until we reach ZZZ
    steps_to(&instructions, &graph, "AAA", "ZZZ")
}

fn part2(input: &str) -> usize {
    let (instructions, graph) = parse(input);
    // find Least Common Multiple (LCM) of all the steps
    graph
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|node| steps_to(&instructions, &graph, node, "Z"))
        .fold(1, |lcm, n| lcm * n / gcd(lcm, n))
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
