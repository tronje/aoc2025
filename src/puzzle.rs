use crate::Result;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait Puzzle {
    type Input;
    type Output;

    fn example_input() -> Self::Input;
    fn example_output() -> Self::Output;

    fn test_example(&mut self) -> Result<()>
    where
        Self::Output: Debug + PartialEq,
    {
        let input = Self::example_input();
        let expected_output = Self::example_output();

        let output = self.solve(input)?;
        assert_eq!(output, expected_output);
        Ok(())
    }

    fn input_file() -> &'static str;

    fn load_input() -> Result<BufReader<File>> {
        let f = File::open(Self::input_file())?;
        Ok(BufReader::new(f))
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead;

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output>;

    fn solution(&mut self) -> Result<Self::Output> {
        let reader = Self::load_input()?;
        let input = Self::parse_input(reader)?;
        self.solve(input)
    }
}
