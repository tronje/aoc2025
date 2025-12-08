use crate::prelude::*;
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub struct Database {
    fresh_ingredients: HashSet<RangeInclusive<u64>>,
    available_ingredients: HashSet<u64>,
}

pub struct One;

impl Puzzle for One {
    type Input = Database;
    type Output = u64;

    fn example_input() -> Self::Input {
        let s = "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        .to_owned();

        let reader = BufReader::new(s.as_bytes());
        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        3
    }

    fn input_file() -> &'static str {
        "inputs/day05/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut fresh_ingredients = HashSet::new();
        let mut available_ingredients = HashSet::new();

        let mut flag = false;

        for line in reader.lines() {
            let tmp = line?;
            let trimmed = tmp.trim();

            if trimmed.is_empty() {
                flag = true;
                continue;
            }

            if flag {
                available_ingredients.insert(trimmed.parse()?);
            } else {
                let mut it = trimmed.split('-');
                let start = it.next().ok_or("invalid input")?.parse()?;
                let end = it.next().ok_or("invalid input")?.parse()?;

                fresh_ingredients.insert(RangeInclusive::new(start, end));
            }
        }

        Ok(Database {
            fresh_ingredients,
            available_ingredients,
        })
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let mut count = 0;

        for ingredient in input.available_ingredients {
            for range in input.fresh_ingredients.iter() {
                if range.contains(&ingredient) {
                    count += 1;
                    break;
                }
            }
        }

        Ok(count)
    }
}

trait RangeHelper<T> {
    fn overlaps(&self, other: &Self) -> bool;
    fn merge(self, other: Self) -> Self;
    fn size(&self) -> T;
}

impl RangeHelper<u64> for RangeInclusive<u64> {
    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start())
            || self.contains(other.end())
            || other.contains(self.start())
            || other.contains(self.end())
    }

    fn merge(self, other: Self) -> Self {
        let start = std::cmp::min(self.start(), other.start());
        let end = std::cmp::max(self.end(), other.end());

        Self::new(*start, *end)
    }

    fn size(&self) -> u64 {
        (self.end() - self.start()) + 1
    }
}

fn merge_overlapping(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut out = Vec::with_capacity(ranges.len());

    'outer: for range in ranges.drain(..) {
        for other in out.iter_mut() {
            if range.overlaps(other) {
                *other = range.merge(other.clone());
                continue 'outer;
            }
        }

        out.push(range);
    }

    out
}

pub struct Two;

impl Puzzle for Two {
    type Input = Vec<RangeInclusive<u64>>;
    type Output = u64;

    fn example_input() -> Self::Input {
        let Database {
            mut fresh_ingredients,
            available_ingredients: _,
        } = One::example_input();

        fresh_ingredients.drain().collect()
    }

    fn example_output() -> Self::Output {
        14
    }

    fn input_file() -> &'static str {
        One::input_file()
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let Database {
            mut fresh_ingredients,
            available_ingredients: _,
        } = One::parse_input(reader)?;

        Ok(fresh_ingredients.drain().collect())
    }

    fn solve(&mut self, mut input: Self::Input) -> Result<Self::Output> {
        loop {
            let len_before = input.len();
            input = merge_overlapping(input);

            if len_before == input.len() {
                break;
            }
        }

        Ok(input.iter().map(RangeHelper::size).sum())
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
