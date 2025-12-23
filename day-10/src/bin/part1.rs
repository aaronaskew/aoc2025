use std::fmt::Display;

use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while1},
    character::complete::{self, char, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, terminated},
};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_, machines) = all_consuming(parse).parse(input).unwrap();

    machines
        .iter()
        .map(|machine| machine.presses())
        .sum::<usize>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    let mut max_len = 0;

    separated_list1(
        line_ending,
        (
            preceded(
                char('['),
                terminated(take_while1(|ch| matches!(ch, '.' | '#')), char(']')),
            )
            .map(|s: &str| {
                if s.len() > max_len {
                    max_len = s.len();
                }

                s.chars()
                    .rev()
                    .fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
            }),
            tag(" "),
            separated_list1(
                char(' '),
                preceded(
                    char('('),
                    terminated(separated_list1(char(','), complete::u16), char(')')),
                ),
            ),
            char(' '),
            preceded(
                char('{'),
                terminated(separated_list1(char(','), complete::u16), char('}')),
            ),
        )
            .map(
                |(light_goal, _, wiring_schematics, _, joltage_requirements)| {
                    let mut buttons = vec![];

                    for wiring_schematic in wiring_schematics {
                        let button = wiring_schematic.iter().fold(0, |acc, idx| acc | 1 << *idx);

                        buttons.push(button);
                    }

                    Machine::new(light_goal, buttons, joltage_requirements)
                },
            ),
    )
    .parse(input)
}

struct Machine {
    light_goal: u16,
    buttons: Vec<u16>,
    _joltage_requirements: Vec<u16>,
}

impl Machine {
    fn new(light_goal: u16, buttons: Vec<u16>, joltage_requirements: Vec<u16>) -> Self {
        Self {
            light_goal,
            buttons,
            _joltage_requirements: joltage_requirements,
        }
    }

    fn presses(&self) -> usize {
        for num_presses in 1..=self.buttons.len() {
            for sequence in self.buttons.iter().combinations(num_presses) {
                let mut lights = 0;

                for button in sequence {
                    lights ^= *button;

                    if lights == self.light_goal {
                        return num_presses;
                    }
                }
            }
        }

        panic!("no button sequence found");
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "light_goal: {:b}", self.light_goal)?;
        writeln!(f, "buttons: [")?;
        for button in &self.buttons {
            writeln!(f, "  {:b},", button)?;
        }
        writeln!(f, "]")?;
        writeln!(f, "joltage_requirements: {:?}", self._joltage_requirements)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let result = "7";

        assert_eq!(process(input), result);
    }
}
