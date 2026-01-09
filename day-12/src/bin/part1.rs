use std::collections::HashSet;

use glam::IVec2;
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while1},
    character::complete::{self, char, line_ending, space1},
    combinator::all_consuming,
    multi::separated_list1,
};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_, (shapes, regions)) = all_consuming(parse).parse(input).expect("should parse");

    regions
        .iter()
        .filter(|region| {
            let shape_area = region
                .shape_counts
                .iter()
                .enumerate()
                .map(|(i, count)| shapes[i].area() * count)
                .sum::<usize>();

            shape_area <= region.area()
        })
        .count()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<Shape>, Vec<Region>)> {
    (
        separated_list1((line_ending, line_ending), shape),
        line_ending,
        line_ending,
        separated_list1(line_ending, region),
    )
        .map(|(s, _, _, r)| (s, r))
        .parse(input)
}

fn shape(input: &str) -> IResult<&str, Shape> {
    let (input, _) = (complete::u64, char(':'), line_ending).parse(input)?;

    separated_list1(line_ending, take_while1(|c| matches!(c, '#' | '.')))
        .map(|v| Shape::new(v))
        .parse(input)
}

fn region(input: &str) -> IResult<&str, Region> {
    (
        complete::u64,
        char('x'),
        complete::u64,
        tag(": "),
        separated_list1(space1, complete::u64),
    )
        .map(|(width, _, length, _, counts)| Region {
            dimensions: IVec2::new(width as i32, length as i32),
            shape_counts: counts.iter().map(|c| *c as usize).collect(),
        })
        .parse(input)
}

#[derive(Debug)]
struct Shape {
    squares: HashSet<IVec2>,
}

impl Shape {
    fn new(input: Vec<&str>) -> Self {
        let mut squares = HashSet::new();

        for (j, row) in input.iter().enumerate() {
            for (i, c) in row.char_indices() {
                if c == '#' {
                    squares.insert(IVec2::new(i as i32, j as i32));
                }
            }
        }

        Self { squares }
    }

    fn area(&self) -> usize {
        self.squares.len()
    }
}

#[derive(Debug)]
struct Region {
    dimensions: IVec2,
    shape_counts: Vec<usize>,
}

impl Region {
    fn area(&self) -> usize {
        (self.dimensions.x * self.dimensions.y) as usize
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

        let result = "2";

        assert_eq!(process(input), result);
    }
}
