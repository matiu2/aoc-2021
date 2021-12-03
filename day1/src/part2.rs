use crate::shared::IncreaseCounter;

/// Given a list of numbers, add up a 3 length sliding window, then return how
/// many sliding windows are larger than the previous sliding window
pub fn part2(input: &[usize]) -> usize {
    input
        .windows(3)
        .map(|window| window.into_iter().sum::<usize>())
        .inspect(|sum| log::debug!("Sum: {}", sum))
        .count_increases()
}

#[cfg(test)]
mod test {
    use crate::shared::test_data;

    #[test]
    fn part2() {
        pretty_env_logger::try_init().ok();
        assert_eq!(super::part2(&test_data()), 5);
    }
}
