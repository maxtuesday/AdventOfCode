use std::{collections::HashSet, fmt::Display};

fn main() {
    let input = include_str!("../../../input/day16.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let start_beam = Beam {
        pos: Pos { r: 0, c: 0 },
        direction: Direction::Right,
    };
    let energized_grid = energize(&grid, start_beam);
    total_energy(&energized_grid)
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    // check all the starting locations on the edge
    // Right
    let mut max_count = 0;
    for i in 0..grid.len() {
        let start_beam = Beam {
            pos: Pos { r: i, c: 0 },
            direction: Direction::Right,
        };
        let energized_grid = energize(&grid, start_beam);
        max_count = max_count.max(total_energy(&energized_grid));
    }

    // Left
    for i in 0..grid.len() {
        let start_beam = Beam {
            pos: Pos {
                r: i,
                c: grid[0].len() - 1,
            },
            direction: Direction::Left,
        };
        let energized_grid = energize(&grid, start_beam);
        max_count = max_count.max(total_energy(&energized_grid));
    }

    // Down
    for i in 0..grid[0].len() {
        let start_beam = Beam {
            pos: Pos { r: 0, c: i },
            direction: Direction::Down,
        };
        let energized_grid = energize(&grid, start_beam);
        max_count = max_count.max(total_energy(&energized_grid));
    }

    // Up
    for i in 0..grid[0].len() {
        let start_beam = Beam {
            pos: Pos {
                r: grid.len() - 1,
                c: i,
            },
            direction: Direction::Up,
        };
        let energized_grid = energize(&grid, start_beam);
        max_count = max_count.max(total_energy(&energized_grid));
    }

    max_count
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
enum Space {
    Empty,
    Mirror(char),
    Splitter(char),
    Energized,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Space::Empty => &'.',
            Space::Mirror(m) => m,
            Space::Splitter(s) => s,
            Space::Energized => &'#',
        };
        write!(f, "{}", x)
    }
}

type Grid = Vec<Vec<Space>>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pos {
    r: usize,
    c: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Beam {
    pos: Pos,
    direction: Direction,
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '/' | '\\' => Space::Mirror(c),
                    '-' | '|' => Space::Splitter(c),
                    c => unimplemented!("unrecognized character: {c}"),
                })
                .collect()
        })
        .collect()
}

fn energize(grid: &Grid, start_beam: Beam) -> Grid {
    let mut energized_grid = vec![vec![Space::Empty; grid[0].len()]; grid.len()];
    let mut visited = HashSet::new();
    let mut queue: Vec<Beam> = Vec::from([start_beam]);

    while let Some(beam) = queue.pop() {
        energized_grid[beam.pos.r][beam.pos.c] = Space::Energized;
        visited.insert(beam.clone());
        for next_beam in next_beams(&beam, grid) {
            if !visited.contains(&next_beam) {
                queue.push(next_beam);
            }
        }
    }
    energized_grid
}

fn total_energy(grid: &Grid) -> usize {
    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|space| **space == Space::Energized)
                .count()
        })
        .sum()
}

fn get_dimensions(grid: &Grid) -> (usize, usize) {
    (grid.len(), grid[0].len())
}

fn next_beam(pos: &Pos, direction: Direction, dimensions: &(usize, usize)) -> Option<Beam> {
    let Some(r) = (match direction {
        Direction::Up => pos.r.checked_sub(1),
        Direction::Down => Some(pos.r + 1),
        Direction::Left => Some(pos.r),
        Direction::Right => Some(pos.r),
    }) else {
        return None;
    };

    let Some(c) = (match direction {
        Direction::Up => Some(pos.c),
        Direction::Down => Some(pos.c),
        Direction::Left => pos.c.checked_sub(1),
        Direction::Right => Some(pos.c + 1),
    }) else {
        return None;
    };

    if r >= dimensions.0 || c >= dimensions.1 {
        return None;
    }

    Some(Beam {
        pos: Pos { r, c },
        direction,
    })
}

fn next_beams(beam: &Beam, grid: &Grid) -> Vec<Beam> {
    let pos = &beam.pos;
    let dimensions = get_dimensions(grid);
    let next = match grid[pos.r][pos.c] {
        Space::Empty => vec![next_beam(pos, beam.direction.clone(), &dimensions)],
        Space::Mirror(mirror) => match mirror {
            '\\' => match beam.direction {
                Direction::Up => vec![next_beam(pos, Direction::Left, &dimensions)],
                Direction::Down => vec![next_beam(pos, Direction::Right, &dimensions)],
                Direction::Left => vec![next_beam(pos, Direction::Up, &dimensions)],
                Direction::Right => vec![next_beam(pos, Direction::Down, &dimensions)],
            },
            '/' => match beam.direction {
                Direction::Up => vec![next_beam(pos, Direction::Right, &dimensions)],
                Direction::Down => vec![next_beam(pos, Direction::Left, &dimensions)],
                Direction::Left => vec![next_beam(pos, Direction::Down, &dimensions)],
                Direction::Right => vec![next_beam(pos, Direction::Up, &dimensions)],
            },
            _ => todo!(),
        },
        Space::Splitter(splitter) => match splitter {
            '-' => {
                // if the Direction is Left or Right, then we should treat this like an Empty space
                match beam.direction {
                    Direction::Up | Direction::Down => {
                        let left = next_beam(pos, Direction::Left, &dimensions);
                        let right = next_beam(pos, Direction::Right, &dimensions);
                        vec![left, right]
                    }
                    Direction::Left | Direction::Right => {
                        vec![next_beam(pos, beam.direction.clone(), &dimensions)]
                    }
                }
            }
            '|' => {
                // if the Direction is Up or Down, then we should treat this like an Empty space
                match beam.direction {
                    Direction::Up | Direction::Down => {
                        vec![next_beam(pos, beam.direction.clone(), &dimensions)]
                    }
                    Direction::Left | Direction::Right => {
                        let up = next_beam(pos, Direction::Up, &dimensions);
                        let down = next_beam(pos, Direction::Down, &dimensions);
                        vec![up, down]
                    }
                }
            }
            _ => todo!(),
        },
        Space::Energized => vec![],
    };
    next.into_iter().filter_map(|b| b).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1(INPUT), 46);
    }

    #[test]
    fn test_part2_ex() {
        assert_eq!(part2(INPUT), 51);
    }
}
