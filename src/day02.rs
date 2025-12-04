use crate::prelude::*;
use std::{io::BufReader, ops::RangeInclusive};

pub struct One;

impl One {
    fn check_number(n: u64) -> bool {
        let s = n.to_string();
        let bytes = s.as_bytes();

        if !bytes.len().is_multiple_of(2) {
            return false;
        }

        let half = bytes.len() / 2;

        bytes[..half] == bytes[half..]
    }
}

impl Puzzle for One {
    type Input = Vec<RangeInclusive<u64>>;
    type Output = u64;

    fn example_input() -> Self::Input {
        let s = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"
            .to_owned();

        let reader = BufReader::new(s.as_bytes());

        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        1227775554
    }

    fn input_file() -> &'static str {
        "inputs/day02/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut ranges = Vec::new();

        for line in reader.lines() {
            let line = line?;

            for range in line.split(',') {
                if range.is_empty() || range.chars().all(char::is_whitespace) {
                    continue;
                }

                let mut split = range.split('-');
                let start = split.next().ok_or("invalid input")?.parse::<u64>()?;
                let end = split.next().ok_or("invalid input")?.parse::<u64>()?;

                ranges.push(RangeInclusive::new(start, end));
            }
        }

        Ok(ranges)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let mut total = 0;

        for range in input {
            for number in range {
                if Self::check_number(number) {
                    total += number
                }
            }
        }

        Ok(total)
    }
}

pub struct Two;

impl Two {
    fn check_number(n: u64) -> bool {
        let s = n.to_string();
        let bytes = s.as_bytes();

        let max_chunk_len = bytes.len() / 2;

        'outer: for chunk_len in 1..=max_chunk_len {
            if !bytes.len().is_multiple_of(chunk_len) {
                continue;
            }

            let mut iter = bytes.chunks(chunk_len);
            let initial = iter.next().unwrap();

            for elem in iter {
                if elem != initial {
                    continue 'outer;
                }
            }

            return true;
        }

        false
    }
}

impl Puzzle for Two {
    type Input = Vec<RangeInclusive<u64>>;
    type Output = u64;

    fn example_input() -> Self::Input {
        One::example_input()
    }

    fn example_output() -> Self::Output {
        4174379265
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

        for range in input {
            for number in range {
                if Self::check_number(number) {
                    total += number
                }
            }
        }

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> Result<()> {
        let mut a = One;
        a.test_example()
    }

    #[test]
    fn b() -> Result<()> {
        let mut b = Two;
        b.test_example()
    }
}
