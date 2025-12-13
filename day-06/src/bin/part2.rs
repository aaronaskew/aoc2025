use itertools::{Itertools, Position};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let problems = parse(input);

    problems.iter().map(|p| p.solve()).sum::<u64>().to_string()
}

fn parse(input: &str) -> Vec<Problem> {
    let mut width: usize = 0;
    let mut op: Operation = Operation::Add;

    let ops_widths = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .with_position()
        .filter_map(|(pos, ch)| {
            width += 1;

            let mut op_width: Option<(Operation, usize)> = None;

            if pos == Position::Last {
                op_width = Some((op, width + 1));
            } else if matches!(ch, '*' | '+') {
                if pos == Position::Middle {
                    op_width = Some((op, width - 1));
                }

                op = match ch {
                    '*' => Operation::Multiply,
                    '+' => Operation::Add,
                    _ => {
                        panic!("* or +");
                    }
                };

                width = 0;
            }

            op_width
        })
        .collect::<Vec<(Operation, usize)>>();

    let number_lines: Vec<&str> = input
        .lines()
        .with_position()
        .take_while(|(pos, _)| *pos != Position::Last)
        .map(|(_, line)| line)
        .collect();

    let num_problems = ops_widths.len();

    let mut problems = vec![vec![]; num_problems];

    let mut char_start_idx = 0;

    for i in 0..problems.len() {
        let problem_strings = &mut problems[i];

        let num_operands = ops_widths[i].1;

        let char_end_idx = char_start_idx + ops_widths[i].1;

        for char_idx in char_start_idx..char_end_idx {
            for line in &number_lines {
                let operand_idx = char_idx - char_start_idx;

                let ch = line.chars().nth(char_idx).unwrap();

                if problem_strings.get(operand_idx).is_none() {
                    problem_strings.push(String::new());
                }

                if ch.is_ascii_digit() {
                    problem_strings[operand_idx].push(ch);
                }
            }
        }

        char_start_idx += num_operands + 1;
    }

    (0..problems.len())
        .map(|i| Problem {
            operands: problems[i]
                .iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect(),
            operation: ops_widths[i].0,
        })
        .collect()
}

#[derive(Debug)]
struct Problem {
    operands: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.operands.iter().sum(),
            Operation::Multiply => self.operands.iter().product(),
        }
    }
}

#[derive(Clone, Debug, Copy)]
enum Operation {
    Add,
    Multiply,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        let result = "3263827";

        assert_eq!(process(input), result);
    }
}
