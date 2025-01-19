use std::{collections::HashMap, env, fs, time::Instant};

use solution::Solution;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod solution;

type SolutionSet = HashMap<usize, Box<dyn Solution>>;

fn day_entry(day: usize, sol: impl Solution + 'static) -> (usize, Box<dyn Solution>) {
    (day, Box::new(sol))
}

enum Day {
    Day(usize),
    All,
}

fn exec_solution(day: usize, solutions: &SolutionSet) {
    let solution = solutions.get(&day).expect("solution for {day} not defined");
    let path = format!("../input/day{:02}.txt", day);
    let input = fs::read_to_string(path).unwrap();

    println!("Day {:02}", day);

    let start = Instant::now();
    let part1 = solution.part1(input.as_str());
    println!("[{:>10?}] Part 1: {}", start.elapsed(), part1);

    let start = Instant::now();
    let part2 = solution.part2(input.as_str());
    println!("[{:>10?}] Part 2: {}", start.elapsed(), part2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).map_or(Day::All, |val| {
        Day::Day(val.parse::<usize>().expect("expected a positive integer"))
    });

    let solutions: SolutionSet = HashMap::from([
        day_entry(1, day01::Day01 {}),
        day_entry(2, day02::Day02 {}),
        day_entry(3, day03::Day03 {}),
        day_entry(4, day04::Day04 {}),
        day_entry(5, day05::Day05 {}),
    ]);

    match day {
        Day::Day(i) => exec_solution(i, &solutions),
        Day::All => {
            let mut days = solutions.keys().collect::<Vec<_>>();
            days.sort();
            for day in days {
                exec_solution(*day, &solutions);
            }
        }
    }
}
