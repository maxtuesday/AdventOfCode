use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day14.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut grid = parse(input);
    tilt(&mut grid, Direction::North);
    calculate_load(&grid)
}

fn part2(input: &str) -> usize {
    let mut grid = parse(input);
    // find cycle size and where cycle starts.
    // Cycle the remaining steps after we take out all the cycles
    // (1000000000 - (cycle_end_idx + 1)) % cycle_size
    let (cycle_end_idx, cycle_size) = find_cycle_size(&mut grid);
    let required_cycles = (1000000000 - (cycle_end_idx + 1)) % cycle_size;
    for _ in 0..required_cycles {
        cycle(&mut grid);
    }
    calculate_load(&grid)
}

#[derive(Clone)]
enum Space {
    RoundRock,
    CubeRock,
    Empty,
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

type Grid = Vec<Vec<Space>>;

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Space::RoundRock,
                    '#' => Space::CubeRock,
                    '.' => Space::Empty,
                    c => unimplemented!("unknown character: {c}"),
                })
                .collect()
        })
        .collect()
}

fn next_location(r: usize, c: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::North => (r - 1, c),
        Direction::East => (r, c + 1),
        Direction::South => (r + 1, c),
        Direction::West => (r, c - 1),
    }
}

fn handle_movement(r: usize, c: usize, grid: &mut Grid, direction: &Direction) {
    match grid[r][c] {
        Space::RoundRock => {
            let mut rr = r;
            let mut cc = c;
            loop {
                let is_next_move_in_bounds = match direction {
                    Direction::North => rr > 0,
                    Direction::East => cc < grid[0].len() - 1,
                    Direction::South => rr < grid.len() - 1,
                    Direction::West => cc > 0,
                };
                if !is_next_move_in_bounds {
                    break;
                }

                (rr, cc) = next_location(rr, cc, &direction);
                match grid[rr][cc] {
                    Space::RoundRock | Space::CubeRock => {
                        // stop
                        match direction {
                            Direction::North => rr += 1,
                            Direction::East => cc -= 1,
                            Direction::South => rr -= 1,
                            Direction::West => cc += 1,
                        };
                        break;
                    }
                    Space::Empty => {
                        // continue
                    }
                }
            }
            grid[r][c] = Space::Empty;
            grid[rr][cc] = Space::RoundRock;
        }
        Space::CubeRock | Space::Empty => {
            // do nothing
        }
    }
}

fn tilt(grid: &mut Grid, direction: Direction) {
    match direction {
        Direction::North => {
            for r in 0..grid.len() {
                for c in 0..grid[0].len() {
                    handle_movement(r, c, grid, &direction)
                }
            }
        }
        Direction::South => {
            for r in (0..grid.len()).rev() {
                for c in 0..grid[0].len() {
                    handle_movement(r, c, grid, &direction)
                }
            }
        }
        Direction::West => {
            for r in 0..grid.len() {
                for c in 0..grid[0].len() {
                    handle_movement(r, c, grid, &direction)
                }
            }
        }
        Direction::East => {
            for r in 0..grid.len() {
                for c in (0..grid[0].len()).rev() {
                    handle_movement(r, c, grid, &direction)
                }
            }
        }
    };
}

fn calculate_load(grid: &Grid) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().map(move |space| match space {
                Space::CubeRock | Space::Empty => 0,
                Space::RoundRock => grid.len() - r,
            })
        })
        .sum()
}

fn cycle(grid: &mut Grid) {
    tilt(grid, Direction::North);
    tilt(grid, Direction::West);
    tilt(grid, Direction::South);
    tilt(grid, Direction::East);
}

fn grid_signature(grid: &Grid) -> String {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|space| match space {
                    Space::RoundRock => "O",
                    Space::CubeRock => "#",
                    Space::Empty => ".",
                })
                .collect::<String>()
        })
        .collect::<String>()
}

fn find_cycle_size(grid: &mut Grid) -> (usize, usize) {
    let mut history: HashMap<String, usize> = HashMap::new();
    let mut i = 0;
    loop {
        cycle(grid);
        let sig = grid_signature(&grid);
        match history.insert(sig, i) {
            Some(prev) => {
                let cycle_size = i - prev;
                return (i, cycle_size);
            }
            None => {
                // do nothing
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1(INPUT), 136);
    }

    #[test]
    fn test_part2_ex() {
        assert_eq!(part2(INPUT), 64);
    }
}
