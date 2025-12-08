mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day08;
mod puzzle;

macro_rules! day_impl {
    ($mod:ident, $one:ident, $two:ident) => {
        let mut one = $mod::$one;
        let start = std::time::Instant::now();
        let solution = one.solution()?;
        println!("part one: {solution} (took {:?})", start.elapsed());

        let mut two = $mod::$two;
        let start = std::time::Instant::now();
        let solution = two.solution()?;
        println!("part two: {solution} (took {:?})", start.elapsed());
    };
}

macro_rules! day_fun {
    ($name:ident, $one:ident, $two:ident) => {
        fn $name() -> Result<()> {
            day_impl!($name, $one, $two);
            Ok(())
        }
    };
}

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
pub use puzzle::Puzzle;

pub mod prelude {
    pub use super::Result;
    pub use crate::puzzle::Puzzle;
    pub use std::fs::File;
    pub use std::io::BufRead;
}

fn main() -> Result<()> {
    let day = std::env::args()
        .nth(1)
        .ok_or("specify a day!")?
        .parse::<u32>()?;

    match day {
        1 => day01(),
        2 => day02(),
        3 => day03(),
        4 => day04(),
        5 => day05(),
        6 => day06(),
        7 => todo!(),
        8 => day08(),
        9 => todo!(),
        10 => todo!(),
        11 => todo!(),
        12 => todo!(),
        _ => Err("invalid day".into()),
    }
}

day_fun!(day01, One, Two);
day_fun!(day02, One, Two);
day_fun!(day03, One, Two);
day_fun!(day04, One, Two);
day_fun!(day05, One, Two);
day_fun!(day06, One, Two);
day_fun!(day08, One, Two);
