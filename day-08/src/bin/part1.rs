use std::collections::HashSet;

use glam::I64Vec3;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input, 1000);
    dbg!(output);
}

fn process(input: &str, max_connections: usize) -> String {
    let (_, junction_boxes) = parse(input).unwrap();

    dbg!(&junction_boxes);

    let mut circuits = vec![];

    // for jb in &junction_boxes {
    //     circuits.push(Circuit {
    //         junction_boxes: HashSet::from([*jb]),
    //     })
    // }

    let mut num_connections = 0;

    junction_boxes
        .iter()
        .permutations(2)
        .map(|v| {
            let mut v = v;
            v.sort();
            (v[0], v[1])
        })
        .unique()
        .map(|(a, b)| {
            let distance = a.distance(b);

            (distance, (a, b))
        })
        .sorted_by(|(distance1, _), (distance2, _)| distance1.cmp(distance2))
        .for_each(|(_, (a, b))| {
            if num_connections < max_connections {
                let circuit_a_exists = find_circuit(&mut circuits, a).is_some();

                let circuit_b_exists = find_circuit(&mut circuits, b).is_some();

                println!("a={:?} b={:?}", a, b);

                match (circuit_a_exists, circuit_b_exists) {
                    (false, false) => {
                        println!("no circuits contain a or b");
                        // create new circuit
                        circuits.push(Circuit {
                            junction_boxes: HashSet::from([*a, *b]),
                        })
                    }
                    (true, false) => {
                        // append to a
                        if let Some(circuit_a) = find_circuit(&mut circuits, a) {
                            println!("circuit contains a but not b: {:?}", circuit_a);
                            circuit_a.junction_boxes.insert(*b);
                        }
                    }
                    (false, true) => {
                        // append to b
                        if let Some(circuit_b) = find_circuit(&mut circuits, b) {
                            println!("circuit contains b but not a: {:?}", circuit_b);
                            circuit_b.junction_boxes.insert(*a);
                        }
                    }
                    (true, true) => {
                        // two circuits
                        let i_a = circuits
                            .iter()
                            .position(|circuit| circuit.contains(a))
                            .unwrap();
                        let i_b = circuits
                            .iter()
                            .position(|circuit| circuit.contains(b))
                            .unwrap();

                        if i_a == i_b {
                            println!("the same circuit already contains a and b, do nothing");
                            println!("this one:  {:?}", circuits[i_a]);
                        } else {
                            let circuit_a = circuits.remove(i_a);
                            let circuit_b = circuits.remove(if i_a < i_b { i_b - 1 } else { i_b });

                            println!("separate circuits contains a and b, combine them");
                            println!("containing a:  {:?}", circuit_a);
                            println!("containing b:  {:?}", circuit_b);

                            let mut junction_boxes = circuit_a.junction_boxes;
                            junction_boxes.extend(circuit_b.junction_boxes);

                            circuits.push(Circuit { junction_boxes });
                        }
                    }
                }
                println!("{:?}\n", circuits);
                num_connections += 1;
            }
        });

    // dbg!(&circuits);

    circuits
        .iter()
        .for_each(|c| println!("circuit: {:?}\nsize: {}", c, c.size()));

    circuits
        .iter()
        .map(|c| c.size())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

fn find_circuit<'a>(
    circuits: &'a mut [Circuit],
    junction_box: &JunctionBox,
) -> Option<&'a mut Circuit> {
    circuits
        .iter_mut()
        .find(|circuit| circuit.contains(junction_box))
}

fn parse(input: &str) -> IResult<&str, Vec<JunctionBox>> {
    separated_list1(
        line_ending,
        separated_list1(tag(","), complete::i64).map(|v| JunctionBox {
            position: I64Vec3::from_slice(&v),
        }),
    )
    .parse(input)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct JunctionBox {
    position: I64Vec3,
}

impl PartialOrd for JunctionBox {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JunctionBox {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.position.x.cmp(&other.position.x) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => match self.position.y.cmp(&other.position.y) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => self.position.z.cmp(&other.position.z),
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            },
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> i64 {
        ((self.position.x - other.position.x).pow(2)
            + (self.position.y - other.position.y).pow(2)
            + (self.position.z - other.position.z).pow(2))
        .isqrt()
    }

    // fn _fdistance(&self, other: &JunctionBox) -> f64 {
    //     ((self.position.x as f64 - other.position.x as f64).powf(2.0)
    //         + (self.position.y as f64 - other.position.y as f64).powf(2.0)
    //         + (self.position.z as f64 - other.position.z as f64).powf(2.0))
    //     .sqrt()
    // }
}

#[derive(Debug)]
struct Circuit {
    junction_boxes: HashSet<JunctionBox>,
}

impl Circuit {
    fn size(&self) -> usize {
        self.junction_boxes
            .iter()
            .inspect(|x| println!("size() inspect jb: {:?}", x))
            .count()
    }

    fn contains(&self, junction_box: &JunctionBox) -> bool {
        self.junction_boxes.contains(junction_box)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let result = "40";

        assert_eq!(process(input, 10), result);
    }
}
