use std::{str::Lines, time::Instant};

fn main() {
    let input = include_str!("../../../input/day05.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug)]
struct Range {
    src: usize,
    dst: usize,
    len: usize,
}

fn parse(input: &str) -> (Vec<usize>, Vec<Vec<Range>>) {
    let mut lines = input.lines();
    let first = lines.next();
    let seeds = parse_seeds(first.unwrap());
    lines.next(); // skip next empty line

    let mut maps: Vec<Vec<Range>> = Vec::new();
    while let Some(line) = lines.next() {
        match line {
            "" => {}
            _ => {
                maps.push(parse_ranges(&mut lines));
            }
        }
    }
    (seeds, maps)
}

fn parse_seeds(line: &str) -> Vec<usize> {
    let (_, seeds) = line.split_once(":").expect("seeds: should begin the line");
    seeds
        .trim()
        .split(" ")
        .filter_map(|seed| seed.parse::<usize>().ok())
        .collect::<Vec<_>>()
}

fn parse_ranges(lines: &mut Lines<'_>) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::new();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let nums = line
            .split(" ")
            .filter_map(|n| n.parse::<usize>().ok())
            .collect::<Vec<_>>();
        assert!(nums.len() == 3);
        ranges.push(Range {
            src: nums[1],
            dst: nums[0],
            len: nums[2],
        });
    }
    ranges
}

fn part1(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |seed, map| {
                let range = map
                    .iter()
                    .find(|range| range.src <= seed && seed < range.src + range.len);
                match range {
                    Some(r) => r.dst + (seed - r.src),
                    None => seed,
                }
            })
        })
        .min()
        .expect("at least one number")
}

fn part2(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    // prepare seeds
    let seed_ranges = seeds
        .as_slice()
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1] - 1)
        .collect::<Vec<_>>();

    // Try backwards.
    // Start from location 0..location_max
    // If we find a Seed in one of the ranges, that is our min location
    let start = Instant::now();
    let mut location: usize = 0;
    loop {
        let seed = maps.iter().rev().fold(location, |loc, map| {
            let range = map
                .iter()
                .find(|&range| range.dst <= loc && loc < range.dst + range.len);
            match range {
                Some(r) => r.src + (loc - r.dst),
                None => loc,
            }
        });
        let seed = seed_ranges.iter().find(|range| range.contains(&seed));
        if seed.is_some() {
            break;
        }
        location += 1;
    }
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
    location
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(INPUT), 46);
    }
}
