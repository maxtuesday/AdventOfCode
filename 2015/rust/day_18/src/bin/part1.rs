use std::fs;
use day_18::process_part_1;

fn main() {
    let file = fs::read_to_string("./../../input/18/input.txt").unwrap();
    println!("{}", process_part_1(&file))
}