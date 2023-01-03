use std::cmp::max;
use std::collections::{HashMap};

fn parse_adj_list(input: &str) -> HashMap<String, HashMap<String, i32>> {
    let mut adj: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in input.lines() {
        let tokens = line.split(" ").collect::<Vec<_>>();
        let a = tokens.first().unwrap().to_string();
        let mut b = tokens.last().unwrap().to_string();
        // remove period at end
        b.remove(b.len() - 1);

        let is_positive = tokens.iter()
            .find(|token| **token == "gain");
        let factor = if is_positive.is_some() { 1 } else { -1 };
        let happiness = *tokens.iter()
            .filter_map(|token| token.parse::<i32>().ok())
            .collect::<Vec<_>>()
            .first()
            .unwrap() * factor;

        adj.entry(a.clone())
            .and_modify(|neighbors| {
                neighbors.entry(b.clone())
                    .and_modify(|val| {
                        *val += happiness;
                    })
                    .or_insert(happiness);
            })
            .or_insert(HashMap::from([(b.clone(), happiness)]));

        adj.entry(b)
            .and_modify(|neighbors| {
                neighbors.entry(a.clone())
                    .and_modify(|val| {
                        *val += happiness;
                    })
                    .or_insert(happiness);
            })
            .or_insert(HashMap::from([(a.clone(), happiness)]));
    }
    adj
}

fn backtrack(cur_name: String,
             sum: i32,
             visited: &mut Vec<String>,
             adj: &HashMap<String, HashMap<String, i32>>,
             max_sum: &mut i32,
) {
    visited.push(cur_name.clone());

    if visited.len() == adj.keys().len() {
        let last_connection = adj.get(visited.first().unwrap()).unwrap()
            .get(visited.last().unwrap()).unwrap();
        let final_sum = sum + last_connection;
        *max_sum = max(*max_sum, final_sum);
    }

    for neighbor in adj.get(cur_name.as_str()).unwrap() {
        if !visited.contains(neighbor.0) {
            backtrack(neighbor.0.clone(), sum + neighbor.1, visited, adj, max_sum);
        }
    }
    visited.pop();
}

fn get_optimal_happiness(adj: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut max_sum = 0;
    let mut visited = vec![];
    let start = adj.keys().last().unwrap();
    backtrack(start.clone(), 0, &mut visited, &adj, &mut max_sum);
    max_sum
}

pub fn process_part_1(input: &str) -> String {
    let adj = parse_adj_list(input);
    get_optimal_happiness(&adj).to_string()
}

pub fn process_part_2(input: &str) -> String {
    let mut adj = parse_adj_list(input);

    let me = "ME";
    let mut my_neighbors:HashMap<String, i32> = HashMap::new();
    for (k, v) in &mut adj {
        v.insert(me.to_string(), 0);
        my_neighbors.insert(k.clone(), 0);
    }
    adj.insert(me.to_string(), my_neighbors);

    get_optimal_happiness(&adj).to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn part1() {
        assert_eq!(process_part_1(INPUT), "330");
    }
}
