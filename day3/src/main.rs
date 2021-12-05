use std::fs::read_to_string;

use part1::part1;

pub mod part1;
fn main() {
    let input = read_to_string("input.txt").unwrap();
    let part1 = part1(&input);
    println!("Day 3 part 1: {}", part1);
}
