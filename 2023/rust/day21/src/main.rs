use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../input/day21.txt");
    println!("Part 1: {}", part1(input));
    // println!("Part 2: {}", part2(input));
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
    .filter(|p| get_location(grid, p) != '#' && !visited.contains(p))
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

/*
How do we translate to the "parallel" plots?
When we go out of bounds, we need to probably modulo the index...

  a  b  c
  d  e  f
  g  h  i

We start at 'e'
'a' if x and y are neg
'b' if x is pos and y is neg
'c' if x is pos and over grid[0].len() and y is neg

'd' if x is neg and y is pos
'f' if x is pos and over grid[0].len() and y is pos

'g' if x is neg and y is over grid.len()
'h' if x is pos and y is over grid.len()
'i' if x is over grid[0].len() and y is over grid.len()

*/

fn get_location(grid: &Grid, pos: &Pos) -> char {
    let x_max = grid[0].len() as i32;
    let y_max = grid.len() as i32;
    let x = pos.x % x_max;
    let x = if x < 0 { x_max + x } else { x };
    let y = pos.y % y_max;
    let y = if y < 0 { y_max + y } else { y };
    grid[x as usize][y as usize]
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    count_plots(&grid, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...........
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

    #[test]
    fn test_count_plots_ex() {
        let grid = parse(INPUT);
        assert_eq!(count_plots(&grid, 6), 16);
    }

    #[test]
    fn test_count_plots_part2_ex1() {
        let grid = parse(INPUT);
        assert_eq!(count_plots(&grid, 6), 16);
    }

    #[test]
    fn test_count_plots_ex2() {
        let grid = parse(INPUT);
        assert_eq!(count_plots(&grid, 10), 50);
    }

    #[test]
    fn test_count_plots_ex3() {
        let grid = parse(INPUT);
        assert_eq!(count_plots(&grid, 50), 1594);
    }

    // #[test]
    // fn test_count_plots_ex4() {
    //     let grid = parse(INPUT);
    //     assert_eq!(count_plots(&grid, 100), 6536);
    // }

    // #[test]
    // fn test_count_plots_ex5() {
    //     let grid = parse(INPUT);
    //     assert_eq!(count_plots(&grid, 500), 167004);
    // }

    // #[test]
    // fn test_count_plots_ex6() {
    //     let grid = parse(INPUT);
    //     assert_eq!(count_plots(&grid, 1000), 668697);
    // }

    // #[test]
    // fn test_count_plots_ex7() {
    //     let grid = parse(INPUT);
    //     assert_eq!(count_plots(&grid, 5000), 16733044);
    // }
}
