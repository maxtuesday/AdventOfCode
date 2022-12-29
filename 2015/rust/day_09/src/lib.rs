use std::collections::{HashMap, HashSet};

fn parse_adjacency_list(input: &str) -> HashMap<String, Vec<(String, i32)>> {
    let mut adj: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for line in input.lines() {
        let tokens = line.split(" ").collect::<Vec<_>>();
        let from = tokens[0];
        let to = tokens[2];
        let cost = tokens[4].parse::<i32>().unwrap();
        adj.entry(from.to_string())
            .and_modify(|dest| {
                dest.push((to.to_string(), cost))
            })
            .or_insert(vec![(to.to_string(), cost)]);
        adj.entry(to.to_string())
            .and_modify(|dest| {
                dest.push((from.to_string(), cost))
            })
            .or_insert(vec![(from.to_string(), cost)]);
    }
    adj
}

pub fn process_part_1(input: &str) -> String {
    let adj = parse_adjacency_list(input);

    let min_path_cost = adj.iter()
        .map(|(start, neighbors)| {
            let mut visited = HashSet::from([start.clone()]);
            let mut path_cost = 0;
            let mut next_neighbors = neighbors.clone();
            loop {
                let next = next_neighbors.into_iter()
                    .filter(|node| !visited.contains(node.0.as_str()))
                    .min_by(|a, b| a.1.cmp(&b.1));
                if let Some(n) = next {
                    path_cost += n.1;
                    visited.insert(n.0.clone());
                    next_neighbors = adj.get(n.0.as_str()).unwrap().clone();
                } else {
                    break;
                }
            }
            path_cost
        })
        .min()
        .unwrap();

    min_path_cost.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let adj = parse_adjacency_list(input);

    let max_path_cost = adj.iter()
        .map(|(start, neighbors)| {
            let mut visited = HashSet::from([start.clone()]);
            let mut path_cost = 0;
            let mut next_neighbors = neighbors.clone();
            loop {
                let next = next_neighbors.into_iter()
                    .filter(|node| !visited.contains(node.0.as_str()))
                    .max_by(|a, b| a.1.cmp(&b.1));
                if let Some(n) = next {
                    path_cost += n.1;
                    visited.insert(n.0.clone());
                    next_neighbors = adj.get(n.0.as_str()).unwrap().clone();
                } else {
                    break;
                }
            }
            path_cost
        })
        .max()
        .unwrap();

    max_path_cost.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn part1() {
        assert_eq!(process_part_1(INPUT), "605");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part_2(INPUT), "982");
    }
}
