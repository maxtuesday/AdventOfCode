use crate::solution::Solution;

pub struct Day02;

struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let diffs = self
            .levels
            .windows(2)
            .map(|window| window[0] - window[1])
            .collect::<Vec<i32>>();

        // - all increasing or all decreasing
        let all_increasing = diffs.iter().all(|diff| *diff > 0);
        let all_decreasing = diffs.iter().all(|diff| *diff < 0);

        // - two adj levels differ by at least 1 and at most 3
        let invalid_differ = diffs.iter().any(|diff | diff.abs() < 1 || diff.abs() > 3);

        (all_increasing || all_decreasing) && !invalid_differ
    }
}

fn parse(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>();
            Report { levels }
        })
        .collect()
}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let reports = parse(input);
        let safe_reports = reports.iter().filter(|report| report.is_safe()).count();
        format!("{safe_reports}")
    }

    fn part2(&self, input: &str) -> String {
        todo!("Part 2")
    }
}
