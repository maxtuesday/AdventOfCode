use std::char;

use crate::solution::Solution;

pub struct Day04;

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>()
}

const TARGET: &[u8] = "XMAS".as_bytes();

fn search_dir(
    mut r: usize,
    mut c: usize,
    grid: &Vec<Vec<char>>,
    r_delta: isize,
    c_delta: isize,
) -> bool {
    for i in 1..=3 {
        match r.checked_add_signed(r_delta) {
            Some(next_r) => match grid.get(next_r) {
                Some(row) => {
                    r = next_r;
                    match c.checked_add_signed(c_delta) {
                        Some(next_c) => match row.get(next_c) {
                            Some(char) => {
                                c = next_c;
                                if *char != TARGET[i] as char {
                                    return false;
                                }
                            }
                            None => return false,
                        },
                        None => return false,
                    }
                }
                None => return false,
            },
            None => return false,
        }
    }
    return true;
}

fn search(r: usize, c: usize, grid: &Vec<Vec<char>>) -> usize {
    let deltas = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    deltas
        .iter()
        .filter(|(r_delta, c_delta)| search_dir(r, c, grid, *r_delta, *c_delta))
        .count()
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        let grid = parse(input);

        // navigate through the grid
        // when we encounter an X, start searching for M, A, S
        // vertically, horizontally, and diagonally
        let mut found = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 'X' {
                    found += search(r, c, &grid);
                }
            }
        }
        format!("{found}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}
