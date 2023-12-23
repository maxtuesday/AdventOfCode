use std::{cmp, collections::HashSet, vec};

fn main() {
    let input = include_str!("../../../input/day18.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let digs = parse(input);
    let dimensions = get_dimensions(&digs);
    dbg!(&dimensions);
    // let mut grid = create_grid(&dimensions);
    // print_grid(&grid);

    // dig
    let mut visited = dig(&dimensions, &digs);
    // let dig_wall = visited.clone();
    let w = (dimensions.w_right + dimensions.w_left.abs()) as usize + 1;
    let h = (dimensions.h_up + dimensions.h_down.abs()) as usize + 1;
    // vec![vec!['.'; w + 1]; h + 1]
    flood_fill(Pos { r: h / 2, c: w / 2 }, w, h, &mut visited);

    // if inside {
    //     fill_visited('x', &mut grid, &visited);
    // }
    // fill_visited('#', &mut grid, &dig_wall);

    // print_grid(&grid);

    visited.len()
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Dig {
    direction: Direction,
    distance: i32,
}

type Grid = Vec<Vec<char>>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    r: usize,
    c: usize,
}

#[derive(Debug)]
struct Dimensions {
    h_up: i32,
    h_down: i32,
    w_right: i32,
    w_left: i32,
}

// How do we find the dimensions of the Grid?
// Follow the dig instructions, start at (+w 0, +h 0, -w 0, -h 0)
// Add to the min and max based on directions.

fn parse(input: &str) -> Vec<Dig> {
    input
        .lines()
        .map(|line| {
            let tokens = line.split(" ").collect::<Vec<_>>();
            let direction = match tokens[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                s => unimplemented!("unrecognized direction: {s}"),
            };
            let distance = tokens[1]
                .parse::<i32>()
                .expect("direction should be followed by a number");
            Dig {
                direction,
                distance,
            }
        })
        .collect()
}

fn get_dimensions(digs: &Vec<Dig>) -> Dimensions {
    let mut dimensions = Dimensions {
        h_up: 0,
        h_down: 0,
        w_right: 0,
        w_left: 0,
    };
    let mut h = 0;
    let mut w = 0;

    for dig in digs {
        match dig.direction {
            Direction::Up => {
                h += dig.distance;
                dimensions.h_up = cmp::max(dimensions.h_up, h);
            }
            Direction::Down => {
                h -= dig.distance;
                dimensions.h_down = cmp::min(dimensions.h_down, h);
            }
            Direction::Left => {
                w -= dig.distance;
                dimensions.w_left = cmp::min(dimensions.w_left, w);
            }
            Direction::Right => {
                w += dig.distance;
                dimensions.w_right = cmp::max(dimensions.w_right, w);
            }
        }
    }
    dimensions
}

// fn create_grid(dimensions: &Dimensions) -> Grid {
//     let w = (dimensions.w_right + dimensions.w_left.abs()) as usize;
//     let h = (dimensions.h_up + dimensions.h_down.abs()) as usize;
//     vec![vec!['.'; w + 1]; h + 1]
// }

fn dig(dimensions: &Dimensions, digs: &Vec<Dig>) -> HashSet<Pos> {
    let mut r = dimensions.h_up;
    let mut c = -dimensions.w_left;
    assert!(c >= 0);

    let mut visited = HashSet::new();

    for dig in digs {
        for _ in 0..dig.distance {
            (r, c) = match dig.direction {
                Direction::Up => (r - 1, c),
                Direction::Down => (r + 1, c),
                Direction::Left => (r, c - 1),
                Direction::Right => (r, c + 1),
            };
            let r = r as usize;
            let c = c as usize;
            // grid[r][c] = '#';
            visited.insert(Pos { r, c });
        }
    }
    visited
}

fn get_next_position(direction: Direction, pos: &Pos, w: usize, h: usize) -> Option<Pos> {
    match direction {
        Direction::Up => {
            let Some(r) = pos.r.checked_sub(1) else {
                println!("R OOB NEG: {:?}", pos);
                return None;
            };
            Some(Pos { r, c: pos.c })
        }
        Direction::Down => {
            let r = pos.r + 1;
            if r >= h {
                println!("R OOB POS: {:?}", pos);
                return None;
            }
            Some(Pos { r, c: pos.c })
        }
        Direction::Left => {
            let Some(c) = pos.c.checked_sub(1) else {
                println!("C OOB NEG: {:?}", pos);
                return None;
            };
            Some(Pos { r: pos.r, c })
        }
        Direction::Right => {
            let c = pos.c + 1;
            if c >= w {
                println!("C OOB POS: {:?}", pos);
                return None;
            }
            Some(Pos { r: pos.r, c })
        }
    }
}

fn get_neighbors(pos: &Pos, w: usize, h: usize, visited: &HashSet<Pos>) -> Vec<Pos> {
    let neighbors = vec![
        get_next_position(Direction::Up, pos, w, h),
        get_next_position(Direction::Down, pos, w, h),
        get_next_position(Direction::Left, pos, w, h),
        get_next_position(Direction::Right, pos, w, h),
    ];

    neighbors
        .into_iter()
        .filter_map(|pos| pos)
        .filter(|pos| !visited.contains(pos))
        .collect()
}

fn flood_fill(start: Pos, w: usize, h: usize, visited: &mut HashSet<Pos>) {
    let mut queue = vec![start];

    while let Some(pos) = queue.pop() {
        // get neighbors
        let neighbors = get_neighbors(&pos, w, h, visited);
        // if !is_inside {
        //     inside = false;
        // }
        for nei in neighbors {
            visited.insert(nei.clone());
            queue.push(nei);
        }
    }
}

// fn fill_visited(fill: char, grid: &mut Grid, visited: &HashSet<Pos>) {
//     for pos in visited {
//         grid[pos.r][pos.c] = fill;
//     }
// }

// fn print_grid(grid: &Grid) {
//     for r in 0..grid.len() {
//         for c in 0..grid[0].len() {
//             print!("{}", grid[r][c]);
//         }
//         println!();
//     }
//     println!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1(INPUT), 62);
    }

    #[test]
    fn test_part1_ex1() {
        let input = "U 3
L 4
D 6
R 4
U 3";
        /*
        LLLLU
        D...U
        D...U <- Start (How do we get the starting location in the grid?)
        D...U
        D...U
        D...U
        DRRRR

        Dimensions {
            h_up: 3,
            h_down: -3,
            w_right: 0,
            w_left: -4,
        }

        Start from [0, 0]
        Go down by h_up
        Go right by w_left

        */
        assert_eq!(part1(input), 35);
    }
}
