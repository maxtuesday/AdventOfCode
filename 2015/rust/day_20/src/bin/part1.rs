use std::fs;
use day_20::process_part_1;

fn main() {
    let file = fs::read_to_string("./../../input/20/input.txt").unwrap();
    println!("{}", process_part_1(&file))
}