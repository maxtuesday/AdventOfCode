use std::fs;
use day_15::process_part_1;

fn main() {
    let file = fs::read_to_string("./../../input/15/input.txt").unwrap();
    println!("{}", process_part_1(&file))
}