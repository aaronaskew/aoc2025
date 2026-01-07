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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Path {
    has_dac: bool,
    has_fft: bool,
    last_node: String,
}

impl Path {
    fn new(last_node: &str) -> Self {
        Self {
            has_dac: last_node == "dac",
            has_fft: last_node == "fft",
            last_node: last_node.to_string(),
        }
    }

    fn is_problematic(&self) -> bool {
        self.last_node == "out" && self.has_dac && self.has_fft
    }

    fn update(&mut self, new_node: String) {
        if new_node == "dac" {
            self.has_dac = true;
        }

        if new_node == "fft" {
            self.has_fft = true;
        }

        self.last_node = new_node;
    }
}

impl Devices {
    fn new(devices: HashMap<String, Vec<String>>) -> Self {
        Self { devices }
    }

    fn num_paths(&self) -> usize {
        count_paths(
            Path::new("svr"),
            |path| {
                let mut new_paths = vec![];

                if let Some(new_nodes) = self.devices.get(&path.last_node) {
                    for new_node in new_nodes {
                        let mut new_path = path.clone();

                        new_path.update(new_node.clone());

                        new_paths.push(new_path.clone());
                    }
                }

                new_paths.into_iter()
            },
            |path| path.is_problematic(),
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        let result = "2";

        assert_eq!(process(input), result);
    }
}
