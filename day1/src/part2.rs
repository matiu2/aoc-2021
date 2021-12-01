use itertools::Itertools;

use crate::shared::{input_to_numbers, IncreaseCounter};

/// Given a list of numbers, add up a 3 length sliding window, then return how many sliding windows are larger than the previous sliding window
pub fn part2(input: &str) -> usize {
    input_to_numbers(input)
        .tuple_windows()
        .inspect(|window| log::debug!("Window: {:?}", window))
        .map(|(a, b, c)| a + b + c)
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
