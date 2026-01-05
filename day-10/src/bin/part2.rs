use std::fmt::Display;

use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while1},
    character::complete::{self, char, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, terminated},
};
use z3::{Optimize, ast::Int};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_, machines) = all_consuming(parse).parse(input).unwrap();

    for machine in &machines {
        machine.presses();
    }

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
                    terminated(separated_list1(char(','), complete::usize), char(')')),
                ),
            ),
            char(' '),
            preceded(
                char('{'),
                terminated(separated_list1(char(','), complete::usize), char('}')),
            ),
        )
            .map(|(light_goal, _, buttons, _, joltage_requirements)| {
                Machine::new(light_goal, buttons, joltage_requirements)
            }),
    )
    .parse(input)
}

struct Machine {
    _light_goal: u16,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

impl Machine {
    fn new(light_goal: u16, buttons: Vec<Vec<usize>>, joltage_requirements: Vec<usize>) -> Self {
        Self {
            _light_goal: light_goal,
            buttons,
            joltage_requirements,
        }
    }

    fn presses(&self) -> usize {
        // let solver = Optimize::new();
        // let zero = Int::from_u64(0);

        // let mut eq = vec![zero.clone(); m.joltage.len()];
        // let mut presses = zero.clone();
        // for (i, conn) in m.buttons.iter().enumerate() {
        //     let count = Int::new_const(format!("count{i}"));
        //     solver.assert(&count.ge(&zero));
        //     for &j in conn {
        //         eq[j] += &count;
        //     }
        //     presses += count;
        // }
        // for (i, &jolt) in m.joltage.iter().enumerate() {
        //     solver.assert(&eq[i].eq(Int::from_u64(jolt)));
        // }

        // solver.minimize(&presses);

        // assert_eq!(solver.check(&[]), SatResult::Sat);
        // let model = solver.get_model().unwrap();
        // model.eval(&presses, true).unwrap().as_u64().unwrap()

        let button_presses = self
            .buttons
            .iter()
            .enumerate()
            .map(|(i, _)| Int::fresh_const(&format!("button_presses{}", i)))
            .collect::<Vec<_>>();

        let solver = Optimize::new();

        for button_press in &button_presses {
            solver.assert(&button_press.ge(0));
        }

        for (joltage_i, joltage) in self.joltage_requirements.iter().enumerate() {
            let mut equation = Int::from(0);

            for (button_i, button) in self.buttons.iter().enumerate() {
                if button.contains(&joltage_i) {
                    equation += button_presses[button_i].clone();
                }
            }

            solver.assert(&equation.eq(Int::from(*joltage as u64)));
        }

        let mut total_presses = Int::from(0);

        for button_press in button_presses {
            total_presses += button_press.clone();
        }

        solver.minimize(&total_presses);

        assert!(solver.check(&[]) == z3::SatResult::Sat);

        let model = solver.get_model().unwrap();

        model.eval(&total_presses, true).unwrap().as_u64().unwrap() as usize
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "light_goal: {:b}", self._light_goal)?;
        writeln!(f, "buttons: [")?;
        for button in &self.buttons {
            writeln!(f, "  {:?},", button)?;
        }
        writeln!(f, "]")?;
        writeln!(f, "joltage_requirements: {:?}", self.joltage_requirements)?;
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

        let result = "33";

        assert_eq!(process(input), result);
    }
}
