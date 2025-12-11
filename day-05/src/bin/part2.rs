use std::{fmt::Debug, ops::RangeInclusive};

use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while1},
    character::complete::{self, line_ending, multispace1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_, ranges) = all_consuming(parse).parse(input).expect("should parse");

    let mut fresh_ranges = PiecewiseRangeInclusive { ranges };

    fresh_ranges.sort();

    let mut count = 0;
    let mut previous: Option<RangeInclusive<u64>> = None;

    for current_range in fresh_ranges.ranges {
        if let Some(ref previous_range) = previous {
            assert!(current_range.start() >= previous_range.start());

            if previous_range.contains(current_range.start()) {
                previous =
                    Some(*previous_range.start()..=*previous_range.end().max(current_range.end()));
            } else {
                count += previous_range.clone().count();
                previous = Some(current_range);
            }
        } else {
            previous = Some(current_range.clone());
        }
    }

    count += previous.unwrap().count();

    count.to_string()
}

struct PiecewiseRangeInclusive {
    ranges: Vec<RangeInclusive<u64>>,
}

impl Debug for PiecewiseRangeInclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PiecewiseRangeInclusive")
            .field("ranges", &self.ranges)
            .finish()
    }
}

impl PiecewiseRangeInclusive {
    fn sort(&mut self) {
        self.ranges.sort_by(|a, b| a.start().cmp(b.start()));
    }
}

fn parse(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    let (input, ranges) = terminated(
        separated_list1(
            line_ending,
            separated_pair(complete::u64, tag("-"), complete::u64)
                .map(|(first, last)| first..=last),
        ),
        multispace1,
    )
    .parse(input)?;

    let (input, _) = take_while1(|_| true).parse(input)?;

    Ok((input, ranges))
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

        let result = "14";

        assert_eq!(process(input), result);
    }
}
