use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
pub enum Instruction {
    #[display("forward {0}")]
    Forward(usize),
    #[display("up {0}")]
    Up(usize),
    #[display("down {0}")]
    Down(usize),
}

#[cfg(test)]
mod unit_test {
    use super::Instruction;

    #[test]
    fn test_parse() {
        let input = "forward 5
down 5
up 3";
        let expected = vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Up(3),
        ];
        let got: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();
        assert_eq!(got, expected);
    }
}
