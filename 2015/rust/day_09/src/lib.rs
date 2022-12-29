use std::collections::{HashMap, HashSet};

enum PathCost {
    MIN,
    MAX,
}

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

fn find_paths(adj: HashMap<String, Vec<(String, i32)>>, path_cost_type: PathCost) -> Vec<i32> {
    adj.iter()
        .map(|(start, neighbors)| {
            let mut visited = HashSet::from([start.clone()]);
            let mut path_cost = 0;
            let mut next_neighbors = neighbors.clone();
            loop {
                let mut next_available = next_neighbors.into_iter()
                    .filter(|node| !visited.contains(node.0.as_str()))
                    .collect::<Vec<_>>();
                next_available.sort_by(|a, b| a.1.cmp(&b.1));
                let next = match path_cost_type {
                    PathCost::MIN => next_available.first(),
                    PathCost::MAX => next_available.last(),
                };
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
        .collect()
}

pub fn process_part_1(input: &str) -> String {
    let adj = parse_adjacency_list(input);
    find_paths(adj, PathCost::MIN).iter()
        .min()
        .unwrap()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let adj = parse_adjacency_list(input);
    find_paths(adj, PathCost::MAX).iter()
        .max()
        .unwrap()
        .to_string()
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
