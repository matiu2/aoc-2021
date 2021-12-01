use std::fs::read_to_string;

pub mod part1;
pub mod part2;
pub mod shared;

fn main() {
    pretty_env_logger::init();
    let input = read_to_string("input.txt").expect("input.txt should exist");
    let part1 = part1::part1(&input);
    println!("Part 1 answer: {}", part1);

    let part2 = part2::part2(&input);
    println!("Part 2 answer: {}", part2);
}
