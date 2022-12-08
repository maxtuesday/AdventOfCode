use std::fs;
use day_03::process_part_2;

fn main() {
    let file = fs::read_to_string("./../../input/03/input.txt").unwrap();
    println!("{}", process_part_2(&file))
}