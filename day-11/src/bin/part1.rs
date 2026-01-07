use std::collections::HashMap;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
};
use pathfinding::prelude::count_paths;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_, devices) = all_consuming(parse).parse(input).unwrap();

    devices.num_paths().to_string()
}

fn parse(input: &str) -> IResult<&str, Devices> {
    separated_list1(
        line_ending,
        separated_pair(
            alpha1::<&str, Error<&str>>,
            tag(": "),
            separated_list1(tag(" "), alpha1),
        ),
    )
    .map(|devices| {
        Devices::new(
            devices
                .iter()
                .map(|(name, outputs)| {
                    (
                        name.to_string(),
                        outputs.iter().map(|s| s.to_string()).collect(),
                    )
                })
                .collect(),
        )
    })
    .parse(input)
}

#[derive(Debug)]
struct Devices {
    devices: HashMap<String, Vec<String>>,
}

impl Devices {
    fn new(devices: HashMap<String, Vec<String>>) -> Self {
        Self { devices }
    }

    fn num_paths(&self) -> usize {
        count_paths(
            "you".to_string(),
            |device| self.devices.get(device).unwrap().iter().cloned(),
            |device| device == "out",
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        let result = "5";

        assert_eq!(process(input), result);
    }
}
