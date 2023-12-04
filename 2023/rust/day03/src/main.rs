use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day_03.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn prepare(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> u32 {
    let mut num = String::new();
    let mut is_adjacent = false;
    let mut nums: Vec<u32> = Vec::new();

    let grid = prepare(input);

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if !ch.is_digit(10) {
                // reset state and push number if meets requirement
                if is_adjacent {
                    nums.push(num.parse::<u32>().expect("should be valid number"))
                }
                is_adjacent = false;
                num = String::new();
                continue;
            }

            // found a digit, check if we are currently building a number
            num.push(ch.clone());
            if is_adjacent {
                continue;
            }
            // check if digit is near symbol
            for i in -1..=1 {
                for j in -1..=1 {
                    let rr = (r as i32) + i;
                    let cc = (c as i32) + j;
                    if rr < 0 || rr >= rows || cc < 0 || cc >= cols {
                        continue;
                    }
                    match grid[rr as usize][cc as usize] {
                        '0'..='9' | '.' => {}
                        _ => is_adjacent = true,
                    }
                }
            }
        }
    }

    nums.iter().sum()
}

fn part2(input: &str) -> u32 {
    let grid = prepare(input);

    let mut num = String::new();
    let mut is_adjacent = false;
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut cur_gears: Vec<(usize, usize)> = Vec::new();

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if !ch.is_digit(10) {
                // reset state and push number if meets requirement
                if is_adjacent {
                    let n = num.parse::<u32>().expect("should be valid number");
                    for gear in cur_gears.iter() {
                        gears
                            .entry(*gear)
                            .and_modify(|val| val.push(n))
                            .or_insert(Vec::from([n]));
                    }
                }
                is_adjacent = false;
                num = String::new();
                cur_gears.clear();
                continue;
            }

            // found a digit, check if we are currently building a number
            num.push(ch.clone());
            if is_adjacent {
                continue;
            }
            // check if digit is near symbol
            for i in -1..=1 {
                for j in -1..=1 {
                    let rr = (r as i32) + i;
                    let cc = (c as i32) + j;
                    if rr < 0 || rr >= rows || cc < 0 || cc >= cols {
                        continue;
                    }
                    match grid[rr as usize][cc as usize] {
                        '0'..='9' | '.' => {}
                        _ => {
                            cur_gears.push((rr as usize, cc as usize));
                            is_adjacent = true
                        }
                    }
                }
            }
        }
    }

    gears
        .values()
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums.iter().product::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(INPUT), 4361);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(INPUT), 467835);
    }
}
