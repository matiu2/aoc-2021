use std::fs::read_to_string;

use crate::{parse::Instruction, part1::part1, part2::part2};

pub mod parse;
pub mod part1;
pub mod part2;
#[cfg(test)]
pub mod test_utils;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("input.txt")?;
    let data: Result<Vec<Instruction>, _> = input.lines().map(|line| line.parse()).collect();
    let data = data?;
    let part1 = part1(&data);
    println!("Day 2 - Part 1: {}", part1);
    let part2 = part2(&data);
    println!("Day 2 - Part 2: {}", part2);
    Ok(())
}
