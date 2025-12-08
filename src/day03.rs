use crate::prelude::*;

trait Helper {
    fn idx_of_max(&self) -> usize;
    fn max_joltage(&self, digits: usize) -> u64;
}

impl<T> Helper for T
where
    T: AsRef<[u8]>,
{
    fn idx_of_max(&self) -> usize {
        let mut max = 0;
        let mut max_idx = 0;

        for (i, item) in self.as_ref().iter().enumerate() {
            if *item > max {
                max_idx = i;
                max = *item;
            }
        }

        max_idx
    }

    fn max_joltage(&self, digits: usize) -> u64 {
        let mut out = 0;

        let mut prev_max_idx = 0;

        for i in 0..digits {
            let from = if i == 0 { 0 } else { prev_max_idx + 1 };
            let to = self.as_ref().len() - (digits - i);

            let slice = &self.as_ref()[from..=to];
            let next_idx = from + slice.idx_of_max();

            out *= 10;
            out += self.as_ref()[next_idx] as u64;

            prev_max_idx = next_idx;
        }

        out
    }
}

pub struct One;

impl Puzzle for One {
    type Input = Vec<Vec<u8>>;
    type Output = u64;

    fn example_input() -> Self::Input {
        let s = "987654321111111
811111111111119
234234234234278
818181911112111"
            .to_owned();

        let reader = BufReader::new(s.as_bytes());

        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        357
    }

    fn input_file() -> &'static str {
        "inputs/day03/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut banks = Vec::new();

        for line in reader.lines() {
            let mut batteries = Vec::new();

            for char in line?.trim().chars() {
                let battery = char.to_digit(10).ok_or("invalid input")? as u8;
                batteries.push(battery)
            }

            banks.push(batteries);
        }

        Ok(banks)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let mut total = 0;

        for bank in input {
            total += bank.max_joltage(2);
        }

        Ok(total)
    }
}

pub struct Two;

impl Puzzle for Two {
    type Input = <One as Puzzle>::Input;
    type Output = <One as Puzzle>::Output;

    fn example_input() -> Self::Input {
        One::example_input()
    }

    fn example_output() -> Self::Output {
        3121910778619
    }

    fn input_file() -> &'static str {
        One::input_file()
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        One::parse_input(reader)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let mut total = 0;

        for bank in input {
            total += bank.max_joltage(12);
        }

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() -> Result<()> {
        let mut one = One;
        one.test_example()
    }

    #[test]
    fn two() -> Result<()> {
        let mut two = Two;
        two.test_example()
    }
}
