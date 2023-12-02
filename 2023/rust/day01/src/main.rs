fn main() {
    let input = include_str!("../../../input/day_01.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|char| char.is_digit(10))
                .collect::<Vec<_>>();
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            format!("{}{}", *first, *last).parse::<i64>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(INPUT), 142);
    }
}
