use crate::prelude::*;
use std::io::BufReader;
use std::ops::{Add, Mul};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn apply<T>(self, a: T, b: T) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }
}

impl FromStr for Operator {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(format!("not an operator: {s}").into()),
        }
    }
}

#[derive(Debug)]
pub struct Problem {
    numbers: Vec<u64>,
    operator: Operator,
}

#[derive(Default)]
struct ProblemBuilder {
    numbers: Vec<u64>,
    operator: Option<Operator>,
}

impl ProblemBuilder {
    fn build(self) -> Result<Problem> {
        match self.operator {
            Some(operator) => Ok(Problem {
                numbers: self.numbers,
                operator,
            }),
            None => Err("incomplete problem".into()),
        }
    }
}

pub struct One;

impl Puzzle for One {
    type Input = Vec<Problem>;
    type Output = u64;

    fn example_input() -> Self::Input {
        let s = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"
        .to_owned();

        let reader = BufReader::new(s.as_bytes());

        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        4277556
    }

    fn input_file() -> &'static str {
        "inputs/day06/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut problems = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            for (column, component) in trimmed.split_whitespace().enumerate() {
                match component.parse::<u64>() {
                    Ok(num) => {
                        if column >= problems.len() {
                            problems.push(ProblemBuilder {
                                numbers: vec![num],
                                operator: None,
                            });
                        } else {
                            problems[column].numbers.push(num);
                        }
                    }

                    Err(_) => match component.parse::<Operator>() {
                        Ok(op) => problems[column].operator = Some(op),

                        Err(_) => return Err(format!("invalid input: {component}").into()),
                    },
                }
            }
        }

        problems
            .into_iter()
            .map(|builder| builder.build())
            .collect::<Result<Vec<Problem>>>()
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let mut total = 0;

        for problem in input {
            total += problem
                .numbers
                .iter()
                .copied()
                .reduce(|acc, e| problem.operator.apply(acc, e))
                .unwrap();
        }

        Ok(total)
    }
}

pub struct Two;

impl Puzzle for Two {
    type Input = Vec<Problem>;
    type Output = u64;

    fn example_input() -> Self::Input {
        let s = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"
        .to_owned();

        let reader = BufReader::new(s.as_bytes());

        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        3263827
    }

    fn input_file() -> &'static str {
        One::input_file()
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut columns = Vec::new();

        for line in reader.lines() {
            let line = line?;

            for (column, c) in line.chars().enumerate() {
                if column >= columns.len() {
                    columns.push(String::from(c));
                } else {
                    if c == '+' || c == '*' {
                        columns[column].push(' ');
                    }

                    columns[column].push(c);
                }
            }
        }

        let mut problems = Vec::new();

        let mut builder = ProblemBuilder::default();
        for column in columns {
            if column.chars().all(char::is_whitespace) {
                let tmp = std::mem::take(&mut builder);
                problems.push(tmp.build()?);
                continue;
            }

            for component in column.split_whitespace() {
                match component.parse::<u64>() {
                    Ok(num) => builder.numbers.push(num),
                    Err(_) => match component.parse::<Operator>() {
                        Ok(op) => builder.operator = Some(op),
                        Err(_) => return Err(format!("invalid input: {component}").into()),
                    },
                }
            }
        }

        problems.push(builder.build()?);

        Ok(problems)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        Ok(input.iter().fold(0, |acc, problem| {
            acc + problem
                .numbers
                .iter()
                .copied()
                .reduce(|acc, e| problem.operator.apply(acc, e))
                .unwrap()
        }))
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
