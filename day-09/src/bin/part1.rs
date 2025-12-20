use glam::IVec2;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_, red_tiles) = all_consuming(parse).parse(input).unwrap();

    red_tiles
        .iter()
        .permutations(2)
        .map(|v| {
            let mut v = v;
            v.sort_by(|a, b| match a.x.cmp(&b.x) {
                std::cmp::Ordering::Equal => a.y.cmp(&b.y),
                ord => ord,
            });
            v
        })
        .unique()
        .map(|v| area(v[0], v[1]))
        .sorted()
        .last()
        .unwrap()
        .to_string()
}

fn area(a: &IVec2, b: &IVec2) -> u64 {
    (b.x.abs_diff(a.x) + 1) as u64 * (b.y.abs_diff(a.y) + 1) as u64
}

fn parse(input: &str) -> IResult<&str, Vec<IVec2>> {
    separated_list1(
        line_ending,
        separated_pair(complete::i32, tag(","), complete::i32).map(|(x, y)| IVec2::new(x, y)),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let result = "50";

        assert_eq!(process(input), result);
    }
}
