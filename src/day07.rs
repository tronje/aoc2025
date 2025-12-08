use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
pub struct Manifold {
    start: Point,
    splitters: HashSet<Point>,
    size: Point,
}

impl Manifold {
    fn is_leaf(&self, pt: Point) -> bool {
        pt.y == self.size.y - 1
    }
}

#[derive(Default)]
struct ManifoldBuilder {
    start: Option<Point>,
    splitters: HashSet<Point>,
    size: Option<Point>,
}

impl ManifoldBuilder {
    fn build(self) -> Result<Manifold> {
        Ok(Manifold {
            start: self.start.ok_or("invalid input")?,
            splitters: self.splitters,
            size: self.size.ok_or("invalid input")?,
        })
    }
}

pub struct One;

impl Puzzle for One {
    type Input = Manifold;
    type Output = usize;

    fn example_input() -> Self::Input {
        let s = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

        let reader = BufReader::new(s.as_bytes());
        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        21
    }

    fn input_file() -> &'static str {
        "inputs/day07/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut manifold = ManifoldBuilder::default();

        let mut x = 0;
        let mut y = 0;

        for line in reader.lines() {
            for c in line?.chars() {
                match c {
                    'S' => manifold.start = Some(Point { x, y }),
                    '^' => {
                        manifold.splitters.insert(Point { x, y });
                    }
                    _ => (),
                }

                x += 1;

                if x > manifold.size.map(|pt| pt.x).unwrap_or(0) {
                    manifold.size = Some(Point { x, y });
                }
            }

            x = 0;
            y += 1;
        }

        manifold.size.as_mut().unwrap().y = y;

        manifold.build()
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let mut beam_points = HashSet::new();
        beam_points.insert(input.start);

        let mut used_splitters = HashSet::new();

        let mut x = 0;
        let mut y = 1;

        while x < input.size.x && y < input.size.y {
            let point = Point { x, y };
            let point_above = Point { x, y: y - 1 };

            if beam_points.contains(&point_above) {
                if input.splitters.contains(&point) {
                    if x > 0 {
                        beam_points.insert(Point { x: x - 1, y });
                    }

                    if x < input.size.x - 1 {
                        beam_points.insert(Point { x: x + 1, y });
                    }

                    used_splitters.insert(point);
                } else {
                    beam_points.insert(point);
                }
            }

            x += 1;

            if x == input.size.x {
                x = 0;
                y += 1;
            }
        }

        Ok(used_splitters.len())
    }
}

pub struct Two;

impl Puzzle for Two {
    type Input = Manifold;
    type Output = usize;

    fn example_input() -> Self::Input {
        One::example_input()
    }

    fn example_output() -> Self::Output {
        40
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
        let mut beam_points = HashMap::new();
        beam_points.insert(input.start, 1);

        let mut x = 0;
        let mut y = 1;

        while x < input.size.x && y < input.size.y {
            let point = Point { x, y };
            let point_above = Point { x, y: y - 1 };

            if let Some(count) = beam_points.get(&point_above).copied() {
                if input.splitters.contains(&point) {
                    if x > 0 {
                        let down_left = Point { x: x - 1, y };
                        *beam_points.entry(down_left).or_insert(0) += count;
                    }

                    if x < input.size.x - 1 {
                        let down_right = Point { x: x + 1, y };
                        *beam_points.entry(down_right).or_insert(0) += count;
                    }
                } else {
                    *beam_points.entry(point).or_insert(0) += count;
                }
            }

            x += 1;

            if x == input.size.x {
                x = 0;
                y += 1;
            }
        }

        Ok(beam_points
            .iter()
            .filter(|(point, _)| input.is_leaf(**point))
            .map(|(_, count)| count)
            .sum())
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
