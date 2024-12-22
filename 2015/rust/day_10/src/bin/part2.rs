use std::fs;
use day_10::process_part_2;

fn main() {
    let file = fs::read_to_string("./../../input/10/input.txt").unwrap();
    println!("{}", process_part_2(&file))
}