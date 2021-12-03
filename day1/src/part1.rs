use crate::shared::IncreaseCounter;

/// Given a list of numbers, return the count where the previous number is lower
pub fn part1(input: &[usize]) -> usize {
    input.into_iter().count_increases()
}

#[cfg(test)]
mod test {
    use crate::shared::test_data;

    use super::part1;

    #[test]
    fn test_part1() {
        pretty_env_logger::try_init().ok();
        assert_eq!(part1(&test_data()), 7);
    }
}
