use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1, space0, space1},
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
    let (_, problems) = all_consuming(parse).parse(input).expect("should parse");

    problems.iter().map(|p| p.solve()).sum::<u64>().to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Problem>> {
    let (input, operands) = terminated(
        separated_list1(
            line_ending,
            preceded(
                space0,
                terminated(separated_list1(space1, complete::u64), space0),
            ),
        ),
        line_ending,
    )
    .parse(input)?;

    let (input, operations) = terminated(
        separated_list1(
            multispace1,
            alt((tag("+"), tag("*"))).map(|op| match op {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                _ => {
                    panic!("should be + or *")
                }
            }),
        ),
        space0,
    )
    .parse(input)?;

    assert!(operands[0].len() == operations.len());

    let mut problems = vec![];

    for x in 0..operands[0].len() {
        let mut problem_operands = vec![];

        for operand_line in &operands {
            problem_operands.push(operand_line[x]);
        }

        problems.push(Problem {
            operands: problem_operands,
            operation: operations[x].clone(),
        })
    }

    Ok((input, problems))
}

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

#[derive(Clone, Debug)]
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

        let result = "4277556";

        assert_eq!(process(input), result);
    }
}
