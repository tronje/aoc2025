use crate::prelude::*;
use std::io::BufReader;

fn surrounding_indices(x: usize, y: usize) -> [(usize, usize); 8] {
    [
        (x.overflowing_sub(1).0, y),                      // left
        (x + 1, y),                                       // right
        (x, y.overflowing_sub(1).0),                      // above
        (x, y + 1),                                       // below
        (x.overflowing_sub(1).0, y.overflowing_sub(1).0), // top left
        (x + 1, y.overflowing_sub(1).0),                  // top right
        (x.overflowing_sub(1).0, y + 1),                  // bottom left
        (x + 1, y + 1),                                   // bottom right
    ]
}

pub struct One;

impl Puzzle for One {
    type Input = Vec<Vec<bool>>;
    type Output = u32;

    fn example_input() -> Self::Input {
        let s = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_owned();

        let reader = BufReader::new(s.as_bytes());

        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        13
    }

    fn input_file() -> &'static str {
        "inputs/day04/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut grid = Vec::new();

        for line in reader.lines() {
            let mut row = Vec::new();

            for c in line?.trim().chars() {
                match c {
                    '.' => row.push(false),
                    '@' => row.push(true),
                    _ => return Err("invalid input".into()),
                }
            }

            grid.push(row);
        }

        Ok(grid)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let length = input.len();
        let width = input[0].len();

        let mut reachable_rolls = 0;

        for (y, row) in input.iter().enumerate() {
            for (x, place) in row.iter().enumerate() {
                if !place {
                    continue;
                }

                let mut adjacent_rolls = 0;

                for (other_x, other_y) in surrounding_indices(x, y) {
                    if other_x < width && other_y < length && input[other_y][other_x] {
                        adjacent_rolls += 1;
                        if adjacent_rolls >= 4 {
                            break;
                        }
                    }
                }

                if adjacent_rolls < 4 {
                    reachable_rolls += 1;
                }
            }
        }

        Ok(reachable_rolls)
    }
}

pub struct Two;

impl Puzzle for Two {
    type Input = Vec<Vec<bool>>;
    type Output = usize;

    fn example_input() -> Self::Input {
        One::example_input()
    }

    fn example_output() -> Self::Output {
        43
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

    fn solve(&mut self, mut input: Self::Input) -> Result<Self::Output> {
        let length = input.len();
        let width = input[0].len();

        let mut removed_rolls = 0;

        loop {
            let mut removed_this_round = 0;

            for y in 0..length {
                for x in 0..width {
                    if !input[y][x] {
                        continue;
                    }

                    let mut adjacent_rolls = 0;

                    for (other_x, other_y) in surrounding_indices(x, y) {
                        if other_x < width && other_y < length && input[other_y][other_x] {
                            adjacent_rolls += 1;
                            if adjacent_rolls >= 4 {
                                break;
                            }
                        }
                    }

                    if adjacent_rolls < 4 {
                        input[y][x] = false;
                        removed_rolls += 1;
                        removed_this_round += 1;
                    }
                }
            }

            if removed_this_round == 0 {
                break;
            }
        }

        Ok(removed_rolls)
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
