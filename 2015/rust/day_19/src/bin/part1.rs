use std::fs;
use day_19::process_part_1;

fn main() {
    let file = fs::read_to_string("./../../input/19/input.txt").unwrap();
    println!("{}", process_part_1(&file))
}