use crate::solution::Solution;

pub struct Day06;

enum Direction {
    North,
    South,
    East,
    West,
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get(pos: (usize, usize), grid: &Vec<Vec<char>>) -> Option<&char> {
    grid.get(pos.0)?.get(pos.1)
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

fn walk(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // find start position
    let mut pos = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find(|(_, char)| **char == '^')
                .map(|(c, _)| (r, c))
        })
        .expect("should contain a starting position");

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
                    dir = match dir {
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                    };
                } else {
                    pos = next;
                }
            },
            None => break,
        }
    }
    grid
}

fn visited(grid: &Vec<Vec<char>>) -> usize {
    grid.iter().map(|row| {
        row.iter().filter(|char| **char == 'X').count()
    }).sum()
}

fn display(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|char| {
            print!("{char}");
        });
        println!();
    });
}

impl Solution for Day06 {
    fn part1(&self, input: &str) -> String {
        let grid = parse(input);
        let walked = walk(grid);
        display(&walked);

        let visited_count = visited(&walked);
        format!("{visited_count}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}
