use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Edge {
    a: Point,
    b: Point,
}

impl Edge {
    fn horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn min_x(&self) -> u64 {
        self.a.x.min(self.b.x)
    }

    fn max_x(&self) -> u64 {
        self.a.x.max(self.b.x)
    }

    fn min_y(&self) -> u64 {
        self.a.y.min(self.b.y)
    }

    fn max_y(&self) -> u64 {
        self.a.y.max(self.b.y)
    }

    fn includes(&self, point: Point) -> bool {
        if self.horizontal() {
            self.a.y == point.y && point.x >= self.min_x() && point.x <= self.max_x()
        } else {
            self.a.x == point.x && point.y >= self.min_y() && point.y <= self.max_y()
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    /// Make a rectangle from two diagonally opposite points.
    ///
    /// For example:
    ///
    /// ```text
    /// a----------+
    /// |          |
    /// |          |
    /// |          |
    /// +----------b
    /// ```
    fn from_points(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    fn corners(&self) -> [Point; 4] {
        [
            self.a,
            self.b,
            Point {
                x: self.a.x,
                y: self.b.y,
            },
            Point {
                x: self.b.x,
                y: self.a.y,
            },
        ]
    }

    fn min_x(&self) -> u64 {
        self.a.x.min(self.b.x)
    }

    fn min_y(&self) -> u64 {
        self.a.y.min(self.b.y)
    }

    fn max_x(&self) -> u64 {
        self.a.x.max(self.b.x)
    }

    fn max_y(&self) -> u64 {
        self.a.y.max(self.b.y)
    }

    fn intersects(&self, edge: &Edge) -> bool {
        if edge.horizontal() {
            self.min_y() < edge.a.y
                && self.max_y() > edge.a.y
                && self.min_x() < edge.a.x.max(edge.b.x)
                && self.max_x() > edge.a.x.min(edge.b.x)
        } else {
            self.min_x() < edge.a.x
                && self.max_x() > edge.a.x
                && self.min_y() < edge.a.y.max(edge.b.y)
                && self.max_y() > edge.a.y.min(edge.b.y)
        }
    }

    fn size(&self) -> u64 {
        (self.a.x.abs_diff(self.b.x) + 1) * (self.a.y.abs_diff(self.b.y) + 1)
    }
}

pub struct One;

impl Puzzle for One {
    type Input = Vec<Point>;
    type Output = u64;

    fn example_input() -> Self::Input {
        vec![
            Point { x: 7, y: 1 },
            Point { x: 11, y: 1 },
            Point { x: 11, y: 7 },
            Point { x: 9, y: 7 },
            Point { x: 9, y: 5 },
            Point { x: 2, y: 5 },
            Point { x: 2, y: 3 },
            Point { x: 7, y: 3 },
        ]
    }

    fn example_output() -> Self::Output {
        50
    }

    fn input_file() -> &'static str {
        "inputs/day09/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut points = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut iter = line.trim().split(',');

            let x = iter.next().ok_or("invalid input")?.parse()?;
            let y = iter.next().ok_or("invalid input")?.parse()?;

            points.push(Point { x, y });
        }

        Ok(points)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let mut max = 0;

        for a in input.iter() {
            for b in input.iter() {
                let size = Rectangle::from_points(*a, *b).size();

                if size > max {
                    max = size;
                }
            }
        }

        Ok(max)
    }
}

pub struct Two;

impl Two {
    fn intersects_any(rectangle: &Rectangle, perimeter: &[Edge]) -> bool {
        perimeter.iter().any(|line| rectangle.intersects(line))
    }

    fn all_corners_in_perimeter(rectangle: &Rectangle, perimeter: &[Edge]) -> bool {
        'outer: for corner in rectangle.corners() {
            // use ray-casting to see if point is in the polygon drawn by `perimeter`
            let mut collisions = 0;

            for line in perimeter.iter() {
                if line.includes(corner) {
                    // corners on a perimeter line count as inside the polygon
                    continue 'outer;
                }

                if line.horizontal() {
                    continue;
                }

                // cast ray straight to the right, only considering vertical perimeter lines
                if line.min_x() > corner.x && line.min_y() <= corner.y && line.max_y() > corner.y {
                    collisions += 1;
                }
            }

            // an even number of collisions in any direction means the point is outside the polygon
            if collisions % 2 == 0 {
                return false;
            }
        }

        true
    }

    fn check_perimeter(rectangle: &Rectangle, perimeter: &[Edge]) -> bool {
        !Self::intersects_any(rectangle, perimeter)
            && Self::all_corners_in_perimeter(rectangle, perimeter)
    }
}

impl Puzzle for Two {
    type Input = Vec<Point>;
    type Output = u64;

    fn example_input() -> Self::Input {
        One::example_input()
    }

    fn example_output() -> Self::Output {
        24
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
        let mut perimeter = Vec::new();

        for i in 0..input.len() - 1 {
            let a = input[i];
            let b = input[i + 1];

            debug_assert!(a.x == b.x || a.y == b.y);

            perimeter.push(Edge { a, b });
        }

        perimeter.push(Edge {
            a: *input.last().unwrap(),
            b: input[0],
        });

        let mut max = 0;

        for a in input.iter() {
            for b in input.iter() {
                let rectangle = Rectangle::from_points(*a, *b);

                if rectangle.size() > max && Self::check_perimeter(&rectangle, &perimeter) {
                    max = rectangle.size();
                }
            }
        }

        Ok(max)
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
