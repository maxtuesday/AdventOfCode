fn main() {
    let input = include_str!("../../../input/day01.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
    .lines()
    .map(|line| {
        let digits = line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();
        // First part of problem guarentees there is at least 1 digit in each line
        10*digits.first().unwrap() + digits.last().unwrap()
    })
    .sum()
}

fn part2(input: &str) -> u32 {
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input.lines().map(|line| {
        let digits = line.chars().enumerate()
            .filter_map(|(i,c)| {
                c.to_digit(10).or_else(|| {
                    for (idx, n) in nums.iter().enumerate() {
                        if line[i..].starts_with(n) {
                            // We will only start with one of the number words
                            return Some((idx + 1) as u32);
                        }
                    }
                    None
                })
            })
            .collect::<Vec<_>>();
        10*digits.first().unwrap() + digits.last().unwrap()
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2_sample() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        assert_eq!(part2(input), 281);
    }
}
