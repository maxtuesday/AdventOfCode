use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../input/day21.txt");
    println!("Part 1: {}", part1(input));
}

type Grid = Vec<Vec<char>>;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_start(grid: &Grid) -> Pos {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                return Pos {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }
    unreachable!("did not find starting location");
}

fn is_in_bounds(pos: &Pos, grid: &Grid) -> bool {
    0 <= pos.x && pos.x < grid.len() as i32 && 0 <= pos.y && pos.y < grid[0].len() as i32
}

fn get_successors(pos: &Pos, grid: &Grid, visited: &HashSet<Pos>) -> Vec<Pos> {
    vec![
        Pos {
            x: pos.x + 1,
            y: pos.y,
        },
        Pos {
            x: pos.x - 1,
            y: pos.y,
        },
        Pos {
            x: pos.x,
            y: pos.y + 1,
        },
        Pos {
            x: pos.x,
            y: pos.y - 1,
        },
    ]
    .into_iter()
    .filter(|p| {
        is_in_bounds(p, grid) && grid[p.x as usize][p.y as usize] != '#' && !visited.contains(p)
    })
    .collect()
}

fn count_plots(grid: &Grid, steps: usize) -> usize {
    let start = get_start(grid);
    let mut visited: HashSet<Pos> = HashSet::from([start.clone()]);
    let mut i = 0;

    let mut even_steps_plots = HashSet::from([start]);
    let mut odd_steps_plots = HashSet::new();

    while i < steps {
        let mut next = HashSet::new();
        if i % 2 == 0 {
            for pos in even_steps_plots.iter() {
                for nei in get_successors(&pos, grid, &visited) {
                    visited.insert(nei.clone());
                    next.insert(nei);
                }
            }
            odd_steps_plots.extend(next.iter());
        } else {
            for pos in odd_steps_plots.iter() {
                for nei in get_successors(&pos, grid, &visited) {
                    visited.insert(nei.clone());
                    next.insert(nei);
                }
            }
            even_steps_plots.extend(next.iter());
        }
        i += 1;
    }
    even_steps_plots.len()
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    count_plots(&grid, 64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_plots_ex() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        let grid = parse(input);
        assert_eq!(count_plots(&grid, 6), 16);
    }
}
