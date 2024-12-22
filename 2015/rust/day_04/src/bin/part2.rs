use std::fs;
use day_04::process_part_2;

fn main() {
    let file = fs::read_to_string("./../../input/04/input.txt").unwrap();
    println!("{}", process_part_2(&file))
}