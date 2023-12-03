fn main() {
    let input = include_str!("../../../input/day_01.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

struct Digit {
    idx: usize,
    digit: u32,
}

impl From<&(usize, char)> for Digit {
    fn from(val: &(usize, char)) -> Self {
        Digit {
            idx: val.0,
            digit: val.1.to_digit(10).unwrap(),
        }
    }
}

fn find_first_and_last_digits(line: &str) -> Option<(Digit, Digit)> {
    let digits = line
        .chars()
        .enumerate()
        .filter(|(_, char)| char.is_digit(10))
        .collect::<Vec<_>>();
    Some((Digit::from(digits.first()?), Digit::from(digits.last()?)))
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            // First part of problem guarentees there is at least 1 digit in each line
            let (first, last) = find_first_and_last_digits(line).unwrap();
            format!("{}{}", first.digit, last.digit)
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    // Could do this with some regex index matching
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .lines()
        .map(|line| {
            let mut first_num = Digit {
                idx: line.len() + 1,
                digit: 0,
            };
            let mut last_num = Digit { idx: 0, digit: 0 };
            if let Some((first, last)) = find_first_and_last_digits(line) {
                first_num = first;
                last_num = last;
            }

            for (idx, n) in nums.iter().enumerate() {
                if let Some(first) = line.find(n) {
                    if first < first_num.idx {
                        first_num.digit = (idx + 1) as u32;
                        first_num.idx = first;
                    }
                };

                if let Some(last) = line.rfind(n) {
                    if last > last_num.idx {
                        last_num.digit = (idx + 1) as u32;
                        last_num.idx = last;
                    }
                }
            }
            format!("{}{}", first_num.digit, last_num.digit)
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
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
