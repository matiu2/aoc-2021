use std::fs::read_to_string;

pub mod part1;

use anyhow::Result;
fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let part1 = part1::part1(&input);
    println!("Part 1 answer: {}", part1);
    Ok(())
}
