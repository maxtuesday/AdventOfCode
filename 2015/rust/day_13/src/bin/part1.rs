use std::fs;
use day_13::process_part_1;

fn main() {
    let file = fs::read_to_string("./../../input/13/input.txt").unwrap();
    println!("{}", process_part_1(&file))
}