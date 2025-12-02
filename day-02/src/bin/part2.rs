use std::ops::RangeInclusive;

use nom::{
    IResult, Parser, bytes::complete::tag, multi::separated_list1, sequence::separated_pair,
};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    if let Ok((_, ranges)) = parse(input) {
        return ranges.iter().flat_map(invalid_ids).sum::<u64>().to_string();
    }

    panic!("couldn't process");
}

fn invalid_ids(range: &RangeInclusive<u64>) -> Vec<u64> {
    range
        .clone()
        .filter(|id| {
            let id_str = id.to_string();

            if id_str.len().is_multiple_of(2) {
                let (left, right) = id_str.split_at(id_str.len() / 2);

                if left == right {
                    return true;
                }
            }

            for len in 1..=(id_str.len() / 2) {
                let pattern = id_str.split_at(len).0;

                let matches = id_str.match_indices(pattern);

                if matches.count() * len == id_str.len() {
                    return true;
                }
            }

            false
        })
        .collect()
}

fn parse(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    let (input, ranges) = separated_list1(tag(","), range).parse(input)?;

    Ok((input, ranges))
}

fn range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (input, (start, end)) = separated_pair(
        nom::character::complete::u64,
        tag("-"),
        nom::character::complete::u64,
    )
    .parse(input)?;

    Ok((input, start..=end))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let result = "4174379265";

        assert_eq!(process(input), result);
    }
}
