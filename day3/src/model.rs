#[derive(Clone)]
pub struct Data(Vec<Vec<bool>>);

/// The count of ones and zeros in a column
#[derive(Default)]
struct Count {
    ones: u32,
    zeros: u32,
}

impl Count {
    /// Returns the most common bit for this column
    fn gamma(&self) -> bool {
        self.ones > self.zeros
    }
    /// Returns the least common bit for this column
    fn epsilon(&self) -> bool {
        self.ones < self.zeros
    }
    /// Returns the most common bit value - if both are equal in count, returns ones (true)
    fn most_common(&self) -> bool {
        self.ones >= self.zeros
    }
}

impl Data {
    /// Given a column number returns the number of ones and zeros in it
    fn count_bits<'a>(data: impl Iterator<Item = &'a Vec<bool>>, col: usize) -> Count {
        data.filter(|row| !row.is_empty()).map(|row| row[col]).fold(
            Count::default(),
            |mut count, value| {
                match value {
                    true => count.ones += 1,
                    false => count.zeros += 1,
                };
                count
            },
        )
    }
    /// Turns a row of bits into a value
    fn row_to_val(row: impl IntoIterator<Item = bool>) -> u32 {
        row.into_iter()
            .fold(0, |output, bit| output << 1 | bit as u32)
    }
    pub fn part1_answer(&self) -> u32 {
        let len = self.0[0].len();
        let counts: Vec<Count> = (0..len)
            .map(|index| Self::count_bits(self.0.iter(), index))
            .collect();
        let gamma = Self::row_to_val(counts.iter().map(|count| count.gamma()));
        let epsilon = Self::row_to_val(counts.iter().map(|count| count.epsilon()));
        gamma * epsilon
    }
    pub fn from_bytes(data: &[u8]) -> Data {
        let data = data
            .split(|&c| c == b'\n')
            .map(|row| {
                row.iter()
                    .map(|&c| match c as char {
                        '1' => true,
                        '0' => false,
                        _ => panic!("Bad char: {c}"),
                    })
                    .collect()
            })
            .collect();
        Data(data)
    }
    /// Filters the data to get a  gas reading
    ///
    /// It'll go through each column (bit) and find the most common bit in that column
    /// Then it'll filter the data keeping either rows where that bit matches, or doesn't match
    /// Depending on the value of `keep_most_common`
    /// Once it reaches 1 row, it'll return that row
    fn filter_gas(&self, keep_most_common: bool) -> u32 {
        let mut data = self.0.clone();
        let num_bits = data[0].len();
        while data.len() > 1 {
            // Find the most common bit in this position
            for col in 0..num_bits {
                let filter_value = Self::count_bits(data.iter(), col).most_common();
                // Only keep rows where the current bit matches the existing bit
                data = data
                    .into_iter()
                    .filter(|row| !row.is_empty())
                    .filter(|row| {
                        if keep_most_common {
                            row[col] == filter_value
                        } else {
                            row[col] != filter_value
                        }
                    })
                    .collect();
                if data.len() == 1 {
                    let out = Self::row_to_val(data[0].iter().copied());
                    return out;
                }
            }
        }
        unreachable!();
    }
    fn get_oxygen(&self) -> u32 {
        self.filter_gas(true)
    }
    fn get_co2(&self) -> u32 {
        self.filter_gas(false)
    }
    pub fn part2_answer(&self) -> u32 {
        self.get_oxygen() * self.get_co2()
    }
}
