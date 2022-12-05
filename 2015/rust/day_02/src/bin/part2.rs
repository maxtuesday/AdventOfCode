use std::fs;
use day_02::process_part_2;

fn main() {
    let file = fs::read_to_string("./../../input/02/input.txt").unwrap();
    println!("{}", process_part_2(&file))
}