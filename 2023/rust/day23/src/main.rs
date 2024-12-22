use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day23.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_in_bounds(pos: &Pos, grid: &Grid) -> bool {
    0 <= pos.y && pos.y < grid.len() as i32 && 0 <= pos.x && pos.x < grid[0].len() as i32
}

fn get_successors(pos: &Pos, grid: &Grid) -> Vec<Pos> {
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
            y: pos.y - 1,
        },
        Pos {
            x: pos.x,
            y: pos.y + 1,
        },
    ]
    .into_iter()
    .filter(|p| is_in_bounds(p, grid) && grid[p.y as usize][p.x as usize] != '#')
    .filter(|p| {
        match grid[p.y as usize][p.x as usize] {
            '>' => {
                // will be be going downhill?
                // this must be to the "right" of the current pos
                pos.x < p.x
            }
            '<' => {
                // will be be going downhill?
                // this must be to the "left" of the current pos
                pos.x > p.x
            }
            '^' => {
                // will be be going downhill?
                // this must be "above" the current pos
                pos.y > p.y
            }
            'v' => {
                // will be be going downhill?
                // this must be "below" the current pos
                pos.y < p.y
            }
            _ => true,
        }
    })
    .collect()
}

fn print_grid(grid: &Grid, visited: &HashMap<Pos, u32>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited.contains_key(&Pos {
                x: x as i32,
                y: y as i32,
            }) {
                print!("O");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!()
    }
    println!()
}

// could do DFS and choose the max between the different paths we come across
fn dfs(pos: &Pos, goal: &Pos, grid: &Grid, visited: &mut HashMap<Pos, u32>) -> u32 {
    if let Some(steps) = visited.get(pos) {
        return *steps;
    }

    if pos == goal {
        // reached the end
        return 0;
    }

    visited.insert(pos.clone(), 0);
    let steps = 1 + get_successors(pos, grid)
        .iter()
        .map(|p| dfs(p, goal, grid, visited))
        .max()
        .expect("should have one number in dfs chain");
    visited.insert(pos.clone(), steps);
    steps
}

fn get_start(grid: &Grid) -> Pos {
    let (x, _) = grid[0].iter().enumerate().find(|(_, &c)| c == '.').unwrap();
    Pos { x: x as i32, y: 0 }
}

fn get_goal(grid: &Grid) -> Pos {
    let (x, _) = grid[grid.len() - 1]
        .iter()
        .enumerate()
        .find(|(_, &c)| c == '.')
        .unwrap();
    Pos {
        x: x as i32,
        y: (grid.len() - 1) as i32,
    }
}

fn part1(input: &str) -> u32 {
    let grid = parse(input);
    let start = get_start(&grid);
    let goal = get_goal(&grid);
    let mut visited = HashMap::new();
    dfs(&start, &goal, &grid, &mut visited)
}

fn part2(input: &str) -> u32 {
    let mut grid = parse(input);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                '>' | '<' | '^' | 'v' => {
                    grid[i][j] = '.';
                }
                _ => {}
            }
        }
    }

    let start = get_start(&grid);
    let goal = get_goal(&grid);
    let mut visited = HashMap::new();
    dfs(&start, &goal, &grid, &mut visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1(INPUT), 94);
    }

    #[test]
    fn test_part2_ex() {
        assert_eq!(part2(INPUT), 154);
    }
}
