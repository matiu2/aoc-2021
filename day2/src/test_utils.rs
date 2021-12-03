use crate::parse::Instruction;

pub fn test_data() -> Vec<Instruction> {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    input.lines().map(|line| line.parse().unwrap()).collect()
}
