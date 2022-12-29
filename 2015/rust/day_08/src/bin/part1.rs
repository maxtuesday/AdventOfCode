use std::fs;
use day_08::process_part_1;

fn main() {
    let file = fs::read_to_string("./../../input/08/input.txt").unwrap();
    println!("{}", process_part_1(&file))
}