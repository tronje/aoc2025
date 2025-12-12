use crate::bit;
use crate::prelude::*;
use crate::util::{ExtractIntegers, Graph, hash};
use std::hash::Hash;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct IndicatorLights {
    bitmask: u32,
    size: u8,
}

impl IndicatorLights {
    fn from_ints(integers: &[u32], size: usize) -> Self {
        debug_assert!(size <= u8::MAX as usize);

        let mut bitmask = 0;

        for n in integers.iter() {
            bitmask |= bit!(n);
        }

        Self {
            bitmask,
            size: size as u8,
        }
    }

    fn from_bools(bools: &[bool]) -> Self {
        debug_assert!(bools.len() <= u8::MAX as usize);

        let mut bitmask = 0;
        let size = bools.len() as u8;

        for (i, on) in bools.iter().enumerate() {
            if *on {
                bitmask |= bit!(i);
            }
        }

        Self { bitmask, size }
    }

    fn apply_button(self, button: Self) -> Self {
        debug_assert_eq!(self.size, button.size);

        let bitmask = self.bitmask ^ button.bitmask;
        let size = self.size;

        Self { bitmask, size }
    }

    fn all_states(self) -> Vec<Self> {
        let mut out = Vec::new();

        let max = 2 << self.size as u32;
        for bitmask in 0..max {
            out.push(Self {
                bitmask,
                size: self.size,
            });
        }

        out
    }

    fn apply_to_joltage(self, joltages: &mut [u32]) {
        for (i, joltage) in joltages.iter_mut().enumerate() {
            if (self.bitmask & bit!(i)) != 0 {
                *joltage += 1;
            }
        }
    }
}

impl std::fmt::Debug for IndicatorLights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.bitmask)
    }
}

pub struct MachineSpec {
    indicator_lights: IndicatorLights,
    buttons: Vec<IndicatorLights>,
    joltage_requirements: Vec<u32>,
}

pub struct One;

impl Puzzle for One {
    type Input = Vec<MachineSpec>;
    type Output = u64;

    fn example_input() -> Self::Input {
        let s = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
        .to_owned();

        let reader = BufReader::new(s.as_bytes());

        Self::parse_input(reader).unwrap()
    }

    fn example_output() -> Self::Output {
        7
    }

    fn input_file() -> &'static str {
        "inputs/day10/input"
    }

    fn parse_input<B>(reader: B) -> Result<Self::Input>
    where
        B: BufRead,
    {
        let mut specs = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut components = line.split_whitespace();

            let lights = components.next().ok_or("invalid input")?;
            let mut indicator_lights = Vec::new();

            for c in lights.chars() {
                match c {
                    '.' => indicator_lights.push(false),
                    '#' => indicator_lights.push(true),
                    _ => {}
                }
            }

            let mut buttons = Vec::new();
            let mut joltage_requirements = Vec::new();

            for component in components {
                let uints = component.uints();

                if component.starts_with('(') {
                    buttons.push(IndicatorLights::from_ints(&uints, indicator_lights.len()));
                } else {
                    joltage_requirements.extend(uints);
                    break;
                }
            }

            specs.push(MachineSpec {
                indicator_lights: IndicatorLights::from_bools(&indicator_lights),
                buttons,
                joltage_requirements,
            });
        }

        Ok(specs)
    }

    fn solve(&mut self, input: Self::Input) -> Result<Self::Output> {
        let mut out = 0;

        for spec in input {
            let all_states = spec.indicator_lights.all_states();
            let mut graph = Graph::new();

            let goal = IndicatorLights {
                bitmask: 0,
                size: spec.indicator_lights.size,
            };

            for state in all_states.iter() {
                graph.add_vertex(*state);

                for button in spec.buttons.iter() {
                    let to = state.apply_button(*button);

                    graph.add_edge(*state, to);
                }
            }

            out += graph
                .shortest_path(goal, spec.indicator_lights)
                .ok_or("no solution")? as u64;
        }

        Ok(out)
    }
}

pub struct Two;

impl Two {
    fn check_joltage(target: &[u32], candidate: &[u32]) -> bool {
        debug_assert_eq!(target.len(), candidate.len());

        target.iter().zip(candidate.iter()).all(|(t, c)| t >= c)
    }
}

impl Puzzle for Two {
    type Input = Vec<MachineSpec>;
    type Output = u64;

    fn example_input() -> Self::Input {
        One::example_input()
    }

    fn example_output() -> Self::Output {
        33
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
        let mut out = 0;

        for spec in input {
            let mut initial_joltage = spec.joltage_requirements.clone();
            initial_joltage.iter_mut().for_each(|joltage| *joltage = 0);

            let mut graph = Graph::new();
            let mut states = Vec::new();

            graph.add_vertex(hash(&initial_joltage));
            states.push(initial_joltage.clone());

            while let Some(state) = states.pop() {
                for button in spec.buttons.iter() {
                    let mut next_state = state.clone();
                    button.apply_to_joltage(&mut next_state);

                    if !Self::check_joltage(&spec.joltage_requirements, &next_state) {
                        continue;
                    }

                    let newly_inserted = graph.add_vertex(hash(&next_state));

                    graph.add_edge(hash(&state), hash(&next_state));

                    if newly_inserted {
                        states.push(next_state);
                    }
                }
            }

            out += graph
                .shortest_path(hash(&initial_joltage), hash(&spec.joltage_requirements))
                .ok_or("no solution")? as u64;
        }

        Ok(out)
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
