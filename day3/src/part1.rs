pub fn part1(input: &str) -> usize {
    // First find how many columns we're dealing with
    let cols: usize = input
        .lines()
        .next()
        .map(|line| line.chars().count())
        .unwrap_or_default();
    let mut one_counts = vec![0; cols];
    let mut zero_counts = vec![0; cols];
    // Count the bits in each column
    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| match c {
            '1' => one_counts[i] += 1,
            '0' => zero_counts[i] += 1,
            other => panic!("Unexpected char: {}", other),
        })
    });
    let mut gamma = 0usize;
    one_counts
        .iter()
        .zip(zero_counts.iter())
        .for_each(|(one, zero)| {
            gamma = gamma << 1;
            if one > zero {
                gamma |= 1
            };
        });
    let mask = !(!0 << cols);
    let epsilon = !gamma & mask;
    gamma * epsilon
}

#[cfg(test)]
mod test {

    #[test]
    fn test_part1() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(super::part1(input), 198);
    }
}
