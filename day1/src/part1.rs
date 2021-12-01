/// Given a list of numbers, return the count where the previous number is lower
use crate::shared::{input_to_numbers, IncreaseCounter};

pub fn part1(input: &str) -> usize {
    // The number of times the depth increases
    input_to_numbers(input).count_increases()
}

#[cfg(test)]
mod test {
    use crate::shared::test_data;

    use super::part1;

    #[test]
    fn test_part1() {
        pretty_env_logger::try_init().ok();
        assert_eq!(part1(test_data()), 7);
    }
}
