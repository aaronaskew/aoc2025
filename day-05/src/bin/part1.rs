use std::ops::RangeInclusive;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_, (ranges, ids)) = all_consuming(parse).parse(input).expect("should parse");

    ids.par_iter()
        .filter(|id| {
            for range in &ranges {
                if range.contains(*id) {
                    return true;
                }
            }
            false
        })
        .count()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let (input, ranges) = terminated(
        separated_list1(
            line_ending,
            separated_pair(complete::u64, tag("-"), complete::u64)
                .map(|(first, last)| first..=last),
        ),
        multispace1,
    )
    .parse(input)?;

    let (input, ids) = separated_list1(line_ending, complete::u64).parse(input)?;

    Ok((input, (ranges, ids)))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        let result = "3";

        assert_eq!(process(input), result);
    }
}
