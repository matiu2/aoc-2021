use std::fs::read;

use crate::model::Data;

mod model;

fn main() {
    let input = read("input.txt").unwrap();
    let data = Data::from_bytes(input.as_slice());
    let part1 = data.part1_answer();
    println!("Part 1 answer: {part1}");
    let part2 = data.part2_answer();
    println!("Part 2 answer: {part2}");
}

#[cfg(test)]
mod unit_tests {
    use crate::model::Data;

    const INPUT: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    #[test]
    fn part1() {
        let data = Data::from_bytes(INPUT.as_bytes());
        let answer = data.part1_answer();
        assert_eq!(answer, 198);
    }

    #[test]
    fn part2() {
        let data = Data::from_bytes(INPUT.as_bytes());
        let answer = data.part2_answer();
        assert_eq!(answer, 230);
    }
}
