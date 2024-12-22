use std::collections::HashSet;

use super::Solution;

pub struct Day06;

#[derive(Eq, PartialEq, Hash, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_start_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find(|(_, char)| **char == '^')
                .map(|(c, _)| (r, c))
        })
        .expect("should contain a starting position")
}

fn next_pos(pos: (usize, usize), dir: &Direction, grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let next = match dir {
        Direction::North => (pos.0.checked_add_signed(-1)?, pos.1),
        Direction::South => (pos.0.checked_add_signed(1)?, pos.1),
        Direction::East => (pos.0, pos.1.checked_add_signed(1)?),
        Direction::West => (pos.0, pos.1.checked_add_signed(-1)?),
    };
    grid.get(next.0)?.get(next.1).map(|_| next)
}

fn next_dir(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
    }
}

fn walk(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // find start position
    let mut pos = get_start_position(&grid);

    // Start moving up
    let mut dir = Direction::North;
    loop {
        // mark current position as visited
        grid[pos.0][pos.1] = 'X';

        let next = next_pos(pos, &dir, &grid);
        match next {
            Some(next) => {
                let val = grid[next.0][next.1];
                if val == '#' {
                    // obstruction
                    // change direction but keep current position
                    dir = next_dir(dir);
                } else {
                    pos = next;
                }
            }
            None => break,
        }
    }
    grid
}

fn visited(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, char)| **char == 'X')
                .map(|(c, _)| (r, c))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

#[derive(Hash, Eq, PartialEq)]
struct ObstructionEncountered {
    pos: (usize, usize),
    travel_dir: Direction,
}

fn contains_loop(
    start_pos: (usize, usize),
    grid: &Vec<Vec<char>>,
) -> bool {
    let mut visited: HashSet<ObstructionEncountered> = HashSet::new();

    // Start moving North
    let mut dir = Direction::North;
    let mut pos = start_pos;
    loop {
        let next = next_pos(pos, &dir, &grid);
        match next {
            Some(next) => {
                let val = grid[next.0][next.1];
                if val == '#' {
                    // obstruction
                    // change direction but keep current position
                    dir = next_dir(dir);

                    let obs_hit = ObstructionEncountered {
                        pos: next,
                        travel_dir: dir.clone(),
                    };
                    if visited.contains(&obs_hit) {
                        return true;
                    }
                    visited.insert(obs_hit);
                } else {
                    pos = next;
                }
            }
            None => return false,
        }
    }
}

impl Solution for Day06 {
    fn part1(&self, input: &str) -> String {
        let grid = parse(input);
        let walked = walk(grid);
        let visited_count = visited(&walked).len();
        format!("{visited_count}")
    }

    fn part2(&self, input: &str) -> String {
        let mut grid = parse(input);

        let starting_pos = get_start_position(&grid);
        let walked = walk(grid.clone());
        let visited_positions = visited(&walked);

        let loop_count = visited_positions
            .into_iter()
            .filter(|pos| *pos != starting_pos)
            .filter(|pos| {
                grid[pos.0][pos.1] = '#'; // add new obstacle
                let result = contains_loop(starting_pos.clone(), &grid);
                grid[pos.0][pos.1] = '.'; // revert
                result
            })
            .count();

        format!("{loop_count}")
    }
}
