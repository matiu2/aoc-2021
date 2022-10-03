use crate::parse::Instruction;

#[derive(Default)]
struct State {
    aim: isize,
    /// Horizontal position
    horiz: isize,
    depth: isize,
}

impl State {
    fn update(&mut self, instruction: &Instruction) {
        #[allow(clippy::match_ref_pats)]
        match instruction {
            &Instruction::Forward(amount) => {
                self.horiz += amount as isize;
                self.depth += self.aim * amount as isize;
            }
            &Instruction::Up(amount) => self.aim -= amount as isize,
            &Instruction::Down(amount) => self.aim += amount as isize,
        }
    }

    /// Answer to the actual puzzle
    fn result(&self) -> isize {
        self.horiz * self.depth
    }
}

pub fn part2(instructions: &[Instruction]) -> isize {
    let mut state = State::default();
    instructions.iter().for_each(|i| state.update(i));
    state.result()
}

#[cfg(test)]
mod test {
    use crate::test_utils::test_data;

    #[test]
    fn test_part2() {
        let data = test_data();
        let got = super::part2(&data);
        assert_eq!(got, 900)
    }
}
