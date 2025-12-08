use crate::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::BufReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn distance(self, other: Self) -> u64 {
        ((self.x.abs_diff(other.x)).pow(2)
            + (self.y.abs_diff(other.y)).pow(2)
            + (self.z.abs_diff(other.z)).pow(2))
        .isqrt()
    }
}

fn build_distances(input: &HashSet<Point>) -> Vec<(Point, Point, u64)> {
    let mut distances = Vec::new();
    let mut connections = HashSet::new();

    for a in input.iter() {
        for b in input.iter() {
            if a == b || connections.contains(&(b, a)) {
                continue;
            }

            distances.push((*a, *b, a.distance(*b)));
            connections.insert((a, b));
        }
    }

    distances.sort_by_key(|(_, _, distance)| *distance);
    distances
}

fn add_cluster(clusters: &mut Vec<HashSet<Point>>, a: Point, b: Point) {
    let mut first_cluster = None;
    let mut second_cluster = None;

    for (idx, cluster) in clusters.iter().enumerate() {
        if cluster.contains(&a) || cluster.contains(&b) {
            if first_cluster.is_none() {
                first_cluster = Some(idx);
            } else if second_cluster.is_none() {
                second_cluster = Some(idx);
                break;
            }
        }
    }

    match (first_cluster, second_cluster) {
        (None, None) => {
            let mut cluster = HashSet::new();
            cluster.insert(a);
            cluster.insert(b);

            clusters.push(cluster);
        }

        (Some(idx), None) => {
            clusters[idx].insert(a);
            clusters[idx].insert(b);
        }

        (Some(idx_a), Some(idx_b)) => {
            let mut to_merge = clusters.remove(idx_b);

            clusters[idx_a].extend(to_merge.drain());

            clusters[idx_a].insert(a);
            clusters[idx_a].insert(b);
        }

        _ => unreachable!(),
    }
}

pub struct One;

impl Puzzle for One {
    type Input = HashSet<Point>;
    type Output = usize;

    fn example_input() -> Self::Input {
        let s = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
        .to_owned();

        let reader = BufReader::new(s.as_bytes());

        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        40
    }

    fn input_file() -> &'static str {
        "inputs/day08/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut junction_boxes = HashSet::new();

        for line in reader.lines() {
            let line = line?;
            let mut iter = line.trim().split(',');

            let x = iter.next().ok_or("invalid input")?.parse()?;
            let y = iter.next().ok_or("invalid input")?.parse()?;
            let z = iter.next().ok_or("invalid input")?.parse()?;

            junction_boxes.insert(Point { x, y, z });
        }

        Ok(junction_boxes)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let n = if cfg!(test) { 10 } else { 1000 };

        let distances = build_distances(&input);
        let mut clusters: Vec<HashSet<Point>> = Vec::new();

        for (a, b, _) in distances.iter().take(n) {
            add_cluster(&mut clusters, *a, *b);
        }

        clusters.sort_by_key(|cluster| -(cluster.len() as i32));

        Ok(clusters
            .iter()
            .map(HashSet::len)
            .take(3)
            .reduce(|acc, e| acc * e)
            .ok_or("not enough clusters")?)
    }
}

pub struct Two;

impl Puzzle for Two {
    type Input = HashSet<Point>;
    type Output = u64;

    fn example_input() -> Self::Input {
        One::example_input()
    }

    fn example_output() -> Self::Output {
        25272
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
        let distances = build_distances(&input);
        let mut clusters: Vec<HashSet<Point>> = Vec::new();

        for (i, (a, b, _)) in distances.iter().enumerate() {
            add_cluster(&mut clusters, *a, *b);

            // avoid checking if a solution has been reached before a minimum of points processed
            if i >= input.len() {
                for cluster in clusters.iter() {
                    if cluster.len() == input.len() {
                        return Ok(a.x * b.x);
                    }
                }
            }
        }

        Err("no solution found".into())
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
