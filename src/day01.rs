use crate::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Turn(i32);

impl Turn {
    fn left(value: i32) -> Self {
        Self(-value.abs())
    }

    fn right(value: i32) -> Self {
        Self(value.abs())
    }
}

impl FromStr for Turn {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.trim().chars();
        let sign = match iter.next() {
            Some('L') => -1i32,
            Some('R') => 1i32,
            Some(_) | None => return Err("invalid input".into()),
        };

        let mut value = 0i32;

        for c in iter {
            let digit: i32 = c
                .to_digit(10)
                .ok_or_else(|| "invalid input".to_owned())?
                .try_into()?;

            value *= 10;
            value += digit;
        }

        Ok(Self(value * sign))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dial(i32);

impl Dial {
    const MAX: i32 = 100; // exclusive

    fn add(&mut self, rhs: Turn) {
        self.0 = (self.0 + rhs.0) % Self::MAX;

        if self.0 < 0 {
            self.0 += Self::MAX;
        }
    }

    fn is_zero(self) -> bool {
        self.0 == 0
    }

    fn zero_crossings(self, turn: Turn) -> u64 {
        let full_turns = (turn.0.abs() / Self::MAX) as u64;

        let remainder = turn.0.abs() % Self::MAX;

        if (turn.0 < 0 && remainder >= self.0 && self.0 != 0)
            || (turn.0 > 0 && (remainder + self.0) >= Self::MAX)
        {
            full_turns + 1
        } else {
            full_turns
        }
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self(50)
    }
}

pub struct One;

impl Puzzle for One {
    type Input = Vec<Turn>;
    type Output = u32;

    fn example_input() -> Self::Input {
        vec![
            Turn::left(68),
            Turn::left(30),
            Turn::right(48),
            Turn::left(5),
            Turn::right(60),
            Turn::left(55),
            Turn::left(1),
            Turn::left(99),
            Turn::right(14),
            Turn::left(82),
        ]
    }

    fn example_output() -> Self::Output {
        3
    }

    fn input_file() -> &'static str {
        "inputs/day01/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut turns = vec![];

        for line in reader.lines() {
            let line = line?;
            let turn = Turn::from_str(&line)?;
            turns.push(turn);
        }

        Ok(turns)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let mut dial = Dial::default();
        let mut count = 0;

        for turn in input {
            dial.add(turn);

            if dial.is_zero() {
                count += 1;
            }
        }

        Ok(count)
    }
}

pub struct Two;

impl Puzzle for Two {
    type Input = Vec<Turn>;
    type Output = u64;

    fn example_input() -> Self::Input {
        One::example_input()
    }

    fn example_output() -> Self::Output {
        6
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
        let mut dial = Dial::default();
        let mut count: u64 = 0;

        for turn in input {
            count += dial.zero_crossings(turn);
            dial.add(turn);
        }

        Ok(count)
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
