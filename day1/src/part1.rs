/// Given a list of numbers, return the count where the previous number is lower

pub fn part1(input: &str) -> usize {
    // The number of times the depth increases
    let mut count = 0;
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
        .reduce(|a, b| {
            if b > a {
                log::info!("{} > {}: {}", b, a, count);
                count += 1;
            } else {
                log::info!("{} !> {}: {}", b, a, count);
            }
            b
        });
    count
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn test_part1() {
        pretty_env_logger::try_init().ok();
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
        assert_eq!(part1(input), 7);
    }
}
