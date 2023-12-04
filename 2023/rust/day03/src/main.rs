fn main() {
    let input = include_str!("../../../input/day_03.txt");
    println!("Part 1: {}", part1(input))
}

fn part1(input: &str) -> u32 {
    let mut num = String::new();
    let mut is_adjacent = false;
    let mut nums: Vec<u32> = Vec::new();

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

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
                continue
            }
            // check if digit is near symbol
            for i in -1..=1 {
                for j in -1..=1 {
                    let rr = (r as i32) + i;
                    let cc = (c as i32) + j;
                    if rr < 0 || rr >= rows || cc < 0 || cc >= cols {
                        continue
                    }
                    match grid[rr as usize][cc as usize] {
                        '0'..='9' | '.' => {},
                        _ => {
                            is_adjacent = true
                        }
                    }
                }
            }
        }
    }
    
    nums.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(input), 4361);
    }
}
