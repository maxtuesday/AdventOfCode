use std::char;

use super::Solution;

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
) -> Option<()> {
    for i in 1..=3 {
        r = r.checked_add_signed(r_delta)?;
        c = c.checked_add_signed(c_delta)?;
        let char = grid.get(r)?.get(c)?;
        if *char != TARGET[i] as char {
            return None;
        }
    }
    return Some(());
}

fn search_xmas(r: usize, c: usize, grid: &Vec<Vec<char>>) -> usize {
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
        .into_iter()
        .filter_map(|(r_delta, c_delta)| search_dir(r, c, grid, r_delta, c_delta))
        .count()
}

fn search_x(r: usize, c: usize, grid: &Vec<Vec<char>>) -> Option<()> {
    // r and c should be an A, so we want to check the surrounding corners
    let top_left = grid
        .get(r.checked_add_signed(-1)?)?
        .get(c.checked_add_signed(-1)?)?
        .to_string();
    let top_right = grid
        .get(r.checked_add_signed(-1)?)?
        .get(c.checked_add_signed(1)?)?
        .to_string();
    let bottom_left = grid
        .get(r.checked_add_signed(1)?)?
        .get(c.checked_add_signed(-1)?)?
        .to_string();
    let bottom_right = grid
        .get(r.checked_add_signed(1)?)?
        .get(c.checked_add_signed(1)?)?
        .to_string();
    let cross = vec![top_left, top_right, bottom_left, bottom_right].join("");
    let cross = cross.as_str();

    // Check layouts:
    // M . M   M . S  S . S  S . M 
    // . A .   . A .  . A .  . A .
    // S . S   M . S  M . M  S . M
    // MMSS    MSMS   SSMM   SMSM
    if matches!(cross, "MMSS" | "MSMS" | "SSMM" | "SMSM") {
        Some(())
    } else {
        None
    }
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
                    found += search_xmas(r, c, &grid);
                }
            }
        }
        format!("{found}")
    }

    fn part2(&self, input: &str) -> String {
        let grid = parse(input);

        let mut found = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 'A' && search_x(r, c, &grid).is_some() {
                    found += 1;
                }
            }
        }
        format!("{found}")
    }
}
