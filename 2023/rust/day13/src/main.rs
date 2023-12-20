fn main() {
    let input = include_str!("../../../input/day13.txt");
    println!("Part 1: {}", part1(input));
}

type Grid = Vec<Vec<char>>;

enum Reflection {
    Row(usize),
    Col(usize),
}

fn parse(input: &str) -> Vec<Grid> {
    input
        .split("\n\n")
        .map(|pattern| pattern.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn transpose(grid: &Grid) -> Grid {
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

fn check_reflection(grid: &Grid, mut l: usize, mut r: usize) -> bool {
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
        // will the next iteration make l negative?
        if l == 0 {
            return true;
        }
        l -= 1;
        r += 1;
    }
}

fn first_reflection(grid: &Grid) -> Option<usize> {
    let reflection_indexes = (0..grid.len() - 1)
        .filter(|&i| check_reflection(grid, i, i + 1))
        .collect::<Vec<_>>();
    // There should be only 0 or 1 indexes found
    assert!(reflection_indexes.len() <= 1);
    reflection_indexes.first().copied()
}

fn find_reflection(grid: &Grid) -> Reflection {
    if let Some(row) = first_reflection(grid) {
        return Reflection::Row(row + 1);
    }
    if let Some(col) = first_reflection(&transpose(grid)) {
        return Reflection::Col(col + 1);
    }

    unreachable!("did not find a reflection");
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|grid| match find_reflection(&grid) {
            Reflection::Row(val) => val * 100,
            Reflection::Col(val) => val,
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
