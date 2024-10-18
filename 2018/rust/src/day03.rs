use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

pub struct Day03;

#[derive(Hash, Eq, PartialEq, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
}

struct Claim {
    id: usize,
    origin: Pos,
    width: usize,
    height: usize,
}

impl Claim {
    fn from(line: &str) -> Self {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        let id = parts[0][1..]
            .parse::<usize>()
            .expect("claim id should be a positive integer");

        let origin_parts = parts[2].split(",").collect::<Vec<_>>();
        let origin_left = origin_parts[0]
            .parse::<usize>()
            .expect("origin left edge distance");
        let origin_top = origin_parts[1]
            .replace(":", "")
            .parse::<usize>()
            .expect("origin top edge distance");
        let origin = Pos {
            x: origin_left,
            y: origin_top,
        };

        let dim_parts = parts[3].split("x").collect::<Vec<_>>();
        let width = dim_parts[0].parse::<usize>().expect("width");
        let height = dim_parts[1].parse::<usize>().expect("height");

        Claim {
            id,
            origin,
            width,
            height,
        }
    }
}

fn parse(input: &str) -> Vec<Claim> {
    input.lines().map(Claim::from).collect::<Vec<_>>()
}

fn dump_grid(grid: &Vec<Vec<usize>>) {
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let count = grid[r][c];
            if count == 0 {
                print!(".")
            } else {
                print!("{count}");
            }
        }
        println!();
    }
    println!();
}

fn process_claims(claims: &Vec<Claim>, grid_size: (usize, usize)) -> usize {
    let mut grid = vec![vec![0; grid_size.0]; grid_size.1];

    for claim in claims {
        let y = claim.origin.y;
        let x = claim.origin.x;
        for r in y..y + claim.height {
            for c in x..x + claim.width {
                grid[r][c] += 1;
            }
        }
    }

    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] >= 2 {
                count += 1;
            }
        }
    }

    count
}

fn process_claims_2(claims: &Vec<Claim>, grid_size: (usize, usize)) -> usize {
    let mut grid: HashMap<Pos, Vec<usize>> = HashMap::new();

    for claim in claims {
        let id = claim.id;
        let y = claim.origin.y;
        let x = claim.origin.x;
        for r in y..y + claim.height {
            for c in x..x + claim.width {
                grid.entry(Pos { x: c, y: r })
                    .and_modify(|v| v.push(id))
                    .or_insert(vec![id]);
            }
        }
    }

    // let mut overlapping_claims = HashSet::new();
    let overlapping_claims = grid
        .iter()
        .flat_map(|(_, claims)| {
            if claims.len() > 1 {
                claims.clone()
            } else {
                vec![]
            }
        })
        .collect::<HashSet<usize>>();

    for claim in claims {
        if !overlapping_claims.contains(&claim.id) {
            return claim.id;
        }
    }

    panic!("did not find non overlapping");
}

impl Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        let claims = parse(input);
        let count = process_claims(&claims, (1000, 1000));
        format!("{count}")
    }

    fn part2(&self, input: &str) -> String {
        let claims = parse(input);
        let id = process_claims_2(&claims, (1000, 1000));
        format!("{id}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_claims() {
        let claims = r"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

        let expected = 4;
        let actual = process_claims(&parse(claims), (11, 9));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_process_claims_2() {
        let claims = r"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

        let expected = 3;
        let actual = process_claims_2(&parse(claims), (11, 9));
        assert_eq!(actual, expected);
    }
}
