use std::fs;
use day_17::process_part_1;

fn main() {
    let file = fs::read_to_string("./../../input/17/input.txt").unwrap();
    println!("{}", process_part_1(&file))
}