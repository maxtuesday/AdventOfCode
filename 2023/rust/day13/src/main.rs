fn main() {
    let input = include_str!("../../../input/day13.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
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

fn count_differences(left: &Vec<char>, right: &Vec<char>) -> usize {
    assert!(left.len() == right.len());
    let mut count = 0;
    for i in 0..left.len() {
        if left[i] != right[i] {
            count += 1;
        }
    }
    return count;
}

fn check_perfect_reflection(grid: &Grid, mut l: usize, mut r: usize) -> bool {
    loop {
        let left = grid.get(l);
        let right = grid.get(r);
        match (left, right) {
            (None, None) | (None, Some(_)) | (Some(_), None) => return true,
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

fn check_smudge_reflection(grid: &Grid, mut l: usize, mut r: usize) -> bool {
    let mut smudges = 0;
    loop {
        let left = grid.get(l);
        let right = grid.get(r);
        match (left, right) {
            (None, None) | (None, Some(_)) | (Some(_), None) => break,
            (Some(left), Some(right)) => {
                let difference_count = count_differences(left, right);
                if difference_count > 1 {
                    return false;
                }
                smudges += difference_count;
            }
        }
        // will the next iteration make l negative?
        if l == 0 {
            break;
        }
        l -= 1;
        r += 1;
    }
    smudges == 1
}

fn first_perfect_reflection(grid: &Grid) -> Option<usize> {
    let reflection_indexes = (0..grid.len() - 1)
        .filter(|&i| check_perfect_reflection(grid, i, i + 1))
        .collect::<Vec<_>>();
    // There should be only 0 or 1 indexes found
    assert!(reflection_indexes.len() <= 1);
    reflection_indexes.first().copied()
}

fn first_smudge_reflection(grid: &Grid) -> Option<usize> {
    let reflection_indexes = (0..grid.len() - 1)
        .filter(|&i| check_smudge_reflection(grid, i, i + 1))
        .collect::<Vec<_>>();
    assert!(reflection_indexes.len() <= 1);
    reflection_indexes.first().copied()
}

fn find_perfect_reflection(grid: &Grid) -> Reflection {
    if let Some(row) = first_perfect_reflection(grid) {
        return Reflection::Row(row + 1);
    }
    if let Some(col) = first_perfect_reflection(&transpose(grid)) {
        return Reflection::Col(col + 1);
    }
    unreachable!("did not find a reflection");
}

fn find_smudge_reflection(grid: &Grid) -> Reflection {
    if let Some(row) = first_smudge_reflection(grid) {
        return Reflection::Row(row + 1);
    }
    if let Some(col) = first_smudge_reflection(&transpose(grid)) {
        return Reflection::Col(col + 1);
    }
    unreachable!("did not find a reflection");
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|grid| match find_perfect_reflection(&grid) {
            Reflection::Row(val) => val * 100,
            Reflection::Col(val) => val,
        })
        .sum()
}

fn part2(input: &str) ->usize {
    parse(input)
        .into_iter()
        .map(|grid| match find_smudge_reflection(&grid) {
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

    #[test]
    fn test_part2_ex() {
        assert_eq!(part2(INPUT), 400);
    }
}
