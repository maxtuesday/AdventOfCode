use std::time::{Duration, Instant};

fn main() {
    let input = include_str!("../../../input/day06.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {:?}", part2(input));
    println!("Part 2: {:?} | (Binary Search)", part2_binary_search(input));
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let nums = input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|word| word.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let times = nums[0].clone();
    let distances = nums[1].clone();
    times.into_iter().zip(distances).collect()
}

fn travel_distance(hold_time: usize, time: usize) -> usize {
    (time - hold_time) * hold_time
}

fn part1(input: &str) -> usize {
    let pairs = parse(input);
    pairs
        .iter()
        .map(|&(time, distance)| {
            (1..time)
                .filter(|&t| travel_distance(t, time) > distance)
                .count()
        })
        .product()
}

fn part2(input: &str) -> (usize, Duration) {
    let start = Instant::now();
    let (time, distance) = parse(input)
        .into_iter()
        .fold((String::new(), String::new()), |acc, x| {
            (format!("{}{}", acc.0, x.0), format!("{}{}", acc.1, x.1))
        });
    let time = time.parse::<usize>().expect("must be number");
    let distance = distance.parse::<usize>().expect("must be number");

    // Time range is not large enough to slow this linear approach enough to warrant a binary search solution.
    // This is actually faster than the implementation below...
    let ans = (1..time)
        .into_iter()
        .filter(|&hold_time| travel_distance(hold_time, time) > distance)
        .count();
    let duration = start.elapsed();
    (ans, duration)
}

fn part2_binary_search(input: &str) -> (usize, Duration) {
    let start = Instant::now();
    let (time, distance) = parse(input)
        .into_iter()
        .fold((String::new(), String::new()), |acc, x| {
            (format!("{}{}", acc.0, x.0), format!("{}{}", acc.1, x.1))
        });
    let time = time.parse::<usize>().expect("must be number");
    let distance = distance.parse::<usize>().expect("must be number");

    // We need to find the first time we beat the distance record
    // and the last time we beat the distance record:
    // Binary Search!

    // Since we are using a difference value for the search_by,
    // it will return Err since that value is not actually in the slice
    let time_range = Vec::from_iter(0..=time);
    let left = time_range
        .as_slice()
        .binary_search_by(|&x| travel_distance(x, time).cmp(&distance))
        .unwrap_err();
    let right = time_range
        .as_slice()
        .binary_search_by(|&x| distance.cmp(&travel_distance(x, time)))
        .unwrap_err();

    let duration = start.elapsed();
    (right - left, duration)
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).0, 71503);
    }

    #[test]
    fn test_part2_binary_search() {
        assert_eq!(part2_binary_search(INPUT).0, 71503);
    }
}
