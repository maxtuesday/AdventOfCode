use std::str::Lines;

fn main() {
    let input = include_str!("../../../input/day_05.txt");
    println!("Part 1: {}", part1(input));
}

#[derive(Debug)]
struct Range {
    src: usize,
    dst: usize,
    len: usize,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Vec<Range>>,
}

fn parse(input: &str) -> Almanac {
    let mut lines = input.lines();
    let first = lines.next();
    let seeds = parse_seeds(first.unwrap());
    lines.next(); // skip next empty line

    // let mut maps:Vec<Vec<Range>> = Vec::from(Vec::new());
    let mut maps: Vec<Vec<Range>> = Vec::new();
    while let Some(line) = lines.next() {
        match line {
            "" => {}
            _ => {
                maps.push(parse_ranges(&mut lines));
            }
        }
    }

    Almanac { seeds, maps }
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
    let almanac = parse(input);
    almanac
        .seeds
        .iter()
        .map(|seed| explore_path(seed.clone(), 0, &almanac))
        .min()
        .expect("at least one number")
}

fn explore_path(n: usize, map_idx: usize, almanac: &Almanac) -> usize {
    if map_idx >= almanac.maps.len() {
        return n;
    }
    // Find if we exist in any of the ranges given
    for range in &almanac.maps[map_idx] {
        if range.src <= n && n < range.src + range.len {
            // n is in the range source range.
            // Map it to the destination.
            let diff = n - range.src;
            let dst = range.dst + diff;
            return explore_path(dst, map_idx + 1, almanac);
        }
    }
    // else we did not find it in any ranges we will, so it is mapped to the same value
    explore_path(n, map_idx + 1, almanac)
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
}
