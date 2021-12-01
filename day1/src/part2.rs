use crate::shared::{input_to_numbers, IncreaseCounter};

/// Given a list of numbers, add up a 3 length sliding window, then return how many sliding windows are larger than the previous sliding window
pub fn part2(input: &str) -> usize {
    // Because we're doing sliding windows, we need to do some actual allocation this time
    // We could use itertools though, which would just allocate 3 values
    let numbers: Vec<usize> = input_to_numbers(input).collect();
    numbers
        // Get 3 values in a sliding window
        .windows(3)
        .inspect(|window| log::debug!("Window: {:?}", window))
        // Add them up
        .map(|window| window.into_iter().sum())
        .inspect(|sum| log::debug!("Sum: {}", sum))
        // Count how many are an increase
        .count_increases()
}

#[cfg(test)]
mod test {
    use crate::shared::test_data;

    #[test]
    fn part2() {
        pretty_env_logger::try_init().ok();
        assert_eq!(super::part2(test_data()), 5);
    }
}
