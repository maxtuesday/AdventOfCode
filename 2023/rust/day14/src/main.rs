fn main() {
    let input = include_str!("../../../input/day14.txt");
    println!("Part 1: {}", part1(input));
}

enum Space {
    RoundRock,
    CubeRock,
    Empty,
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

fn tilt(grid: &mut Grid) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            // move only RoundRocks North
            match grid[r][c] {
                Space::RoundRock => {
                    // "subtrack" r until grid[r][c] is either 0 or hit a Rock
                    let mut rr = r;
                    while rr > 0 {
                        rr -= 1;
                        match grid[rr][c] {
                            Space::RoundRock | Space::CubeRock => {
                                // stop
                                rr += 1; // this is the location we should move the RoundRock to
                                break;
                            }
                            Space::Empty => {
                                // continue
                            }
                        }
                    }
                    grid[r][c] = Space::Empty;
                    grid[rr][c] = Space::RoundRock;
                }
                Space::CubeRock | Space::Empty => {
                    // do nothing
                }
            }
        }
    }
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

fn print_grid(grid: &Grid) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let c = match grid[r][c] {
                Space::RoundRock => "O",
                Space::CubeRock => "#",
                Space::Empty => ".",
            };
            print!("{c}");
        }
        println!()
    }
    println!()
}

fn part1(input: &str) -> usize {
    let mut grid = parse(input);
    tilt(&mut grid);
    calculate_load(&grid)
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
}
