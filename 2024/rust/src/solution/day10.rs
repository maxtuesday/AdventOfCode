use std::collections::HashSet;

use super::Solution;

pub struct Day10;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    r: usize,
    c: usize,
}

type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_oob(pos: &Pos, grid: &Grid) -> bool {
    pos.r >= grid.len() || pos.c >= grid[0].len()
}

const DR: [isize; 4] = [1, 0, -1, 0];
const DC: [isize; 4] = [0, 1, 0, -1];

fn next_positions(pos: Pos, grid: &Grid) -> Vec<Pos> {
    (0..4)
        .filter_map(|i| {
            Some(Pos {
                r: pos.r.checked_add_signed(DR[i])?,
                c: pos.c.checked_add_signed(DC[i])?,
            })
        })
        // filter for only in-bounds positions
        .filter(|p| !is_oob(p, grid))
        // filter for only increasing by 1 positions
        .filter(|p| grid[p.r][p.c] as i8 - grid[pos.r][pos.c] as i8 == 1)
        .collect()
}

fn dfs(pos: Pos, grid: &Grid, visited: &mut HashSet<Pos>) -> usize {
    if grid[pos.r][pos.c] == '9' && !visited.contains(&pos) {
        visited.insert(pos);
        return 1;
    }

    next_positions(pos, grid)
        .into_iter()
        .map(|next| dfs(next, grid, visited))
        .sum()
}

impl Solution for Day10 {
    fn part1(&self, input: &str) -> String {
        let grid = parse(input);

        // Gather all starting positions
        let starting_positions = grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(c, val)| {
                        if *val == '0' {
                            Some(Pos { r, c })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let score = starting_positions
            .into_iter()
            .map(|pos| {
                let mut visited_peaks = HashSet::new();
                dfs(pos.clone(), &grid, &mut visited_peaks)
            })
            .sum::<usize>();

        format!("{score}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT: &str = "0123
1234
8765
9876";

    #[test]
    fn part1_small_input() {
        let expected = String::from("1");
        let actual = Day10.part1(SMALL_INPUT);

        assert_eq!(actual, expected);
    }

    const LARGE_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_large_input() {
        let expected = String::from("36");
        let actual = Day10.part1(LARGE_INPUT);

        assert_eq!(actual, expected);
    }
}
