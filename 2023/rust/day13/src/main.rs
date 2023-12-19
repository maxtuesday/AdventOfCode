fn main() {
    let input = include_str!("../../../input/day13.txt");
    println!("Part 1: {}", part1(input));
}

#[derive(Clone)]
struct Pattern {
    grid: Vec<Vec<char>>,
    reflection_row: usize,
    reflection_col: usize,
}

fn parse(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pattern| {
            let grid = pattern.lines().map(|line| line.chars().collect()).collect();
            Pattern {
                grid,
                reflection_row: 0,
                reflection_col: 0,
            }
        })
        .collect()
}

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = Vec::new();
    for c in 0..grid[0].len() {
        let mut row = Vec::new();
        for r in (0..grid.len()).rev() {
            row.push(grid[r][c]);
        }
        new_grid.push(row);
    }
    new_grid
}

fn check_row_reflection(grid: &Vec<Vec<char>>, mut l: usize, mut r: usize) -> bool {
    loop {
        let left = grid.get(l);
        let right = grid.get(r);
        match (left, right) {
            (None, None) => return true,
            (None, Some(_)) => return true,
            (Some(_), None) => return true,
            (Some(left), Some(right)) => {
                if left != right {
                    return false;
                }
            }
        }
        if l == 0 {
            return true;
        }
        l -= 1;
        r += 1;
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            print!("{}", grid[r][c]);
        }
        println!()
    }
    println!()
}

fn find_reflection(pattern: Pattern) -> Pattern {
    // print_grid(&pattern.grid);
    // search rows
    // use windows(2) and spread out the check until we run out of rows

    // let range = (0..pattern.grid.len() - 1);

    let matching_row_pairs = pattern
        .grid
        .windows(2)
        .enumerate()
        .filter(|(i, _)| check_row_reflection(&pattern.grid, *i, i + 1))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let reflection_row = if matching_row_pairs.len() > 0 {
        matching_row_pairs[0] + 1
    } else {
        0
    };

    let transposed_grid = transpose(&pattern.grid);
    // print_grid(&transposed_grid);
    let matching_col_pairs = transposed_grid
        .windows(2)
        .enumerate()
        .filter(|(i, _)| check_row_reflection(&transposed_grid, *i, i + 1))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let reflection_col = if matching_col_pairs.len() > 0 {
        matching_col_pairs[0] + 1
    } else {
        0
    };
    // if the len of rows is 1, then that is the only spot the reflection can exist
    // if the len is greater than 1, then we need to

    Pattern {
        grid: pattern.grid.clone(),
        reflection_row,
        reflection_col,
    }
}

fn part1(input: &str) -> usize {
    // Find if pattern is mirrored
    // Horizontally or Vertically

    parse(input)
        .into_iter()
        .map(|pattern| {
            let p = find_reflection(pattern);
            p.reflection_col + (p.reflection_row * 100)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1(INPUT), 405);
    }

    #[test]
    fn test_part1_ex_1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn test_part1_ex_2() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part1(input), 400);
    }
}
