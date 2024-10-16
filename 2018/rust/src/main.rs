use std::{collections::HashMap, env, fs, time::Instant};

use solution::Solution;

mod solution;
mod day01;
mod day02;

fn day_entry(day: usize, sol: impl Solution + 'static) -> (usize, Box<dyn Solution>) {
    (day, Box::new(sol))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args
        .get(1)
        .unwrap_or(&String::from("1"))
        .parse::<usize>()
        .expect("expected an integer");

    let solutions: HashMap<usize, Box<dyn Solution>> = HashMap::from([
        day_entry(1, day01::Day01{}),
        day_entry(2, day02::Day02{}),
    ]);

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
