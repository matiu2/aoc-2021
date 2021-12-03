use itertools::Itertools;

/// Given input where each line is a number, returns the actual numbers
/// Logs any that couldn't be parsed
pub fn input_to_numbers<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input
        .lines()
        .map(|line| line.parse::<usize>())
        .enumerate()
        .inspect(|(line_num, result)| {
            if let Err(result) = result {
                log::error!("Error parsing line {}: {}", line_num, result)
            }
        })
        .flat_map(|(_line_num, result)| result.ok())
}

/// A trait used for augmenting the behaviour of iterators of numbers
pub trait IncreaseCounter {
    fn count_increases(&mut self) -> usize;
}

impl<Iter, Item> IncreaseCounter for Iter
where
    Iter: Iterator<Item = Item>,
    Item: PartialOrd + Clone,
{
    /// Counts the times a number is greater than its predecesor
    fn count_increases(&mut self) -> usize {
        self.tuple_windows().filter(|(a, b)| b > a).count()
    }
}

#[cfg(test)]
pub fn test_data() -> Vec<usize> {
    let input = "199
200
208
210
200
207
240
269
260
263";
    input_to_numbers(input).collect()
}
