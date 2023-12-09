fn main() {
    let input = include_str!("../../../input/day_06.txt");
    println!("Part 1: {}", part1(input));
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let nums = input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|word| word.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let times = nums[0].clone();
    let distances = nums[1].clone();
    times.into_iter().zip(distances).collect()
}

fn part1(input: &str) -> usize {
    let pairs = parse(input);
    pairs
        .iter()
        .map(|&(time, distance)| {
            (1..time)
                .filter(|&t| {
                    let rem = time - t;
                    t * rem > distance
                })
                .count()
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 288);
    }
}
