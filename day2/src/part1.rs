use crate::parse::Instruction;

impl Instruction {
    /// Returns the number of meters to dive for this instruction
    pub fn depth_delta(&self) -> isize {
        match self {
            Instruction::Up(amount) => -(*amount as isize),
            Instruction::Down(amount) => *amount as isize,
            _ => 0,
        }
    }

    /// Returns the number of meters forward to move for this instruction
    pub fn horiz_delta(&self) -> isize {
        match self {
            Instruction::Forward(amount) => *amount as isize,
            _ => 0,
        }
    }
}

/// Calculate the horizontal and vertical distance and return their product
pub fn part1(instructions: &[Instruction]) -> isize {
    let horiz: isize = instructions.iter().map(|i| i.horiz_delta()).sum();
    let depth: isize = instructions.iter().map(|i| i.depth_delta()).sum();
    horiz * depth
}

#[cfg(test)]
mod test {
    use crate::parse::Instruction;

    use super::part1;

    #[test]
    fn test_part1() {
        let data = crate::test_utils::test_data();
        let expected = 150;
        let got = part1(&data);
        assert_eq!(expected, got);
    }
}
