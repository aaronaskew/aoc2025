use std::{collections::HashMap, ops::RangeInclusive};

use glam::UVec2;
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

    let grid_compressor = GridCompressor::new(&red_tiles);

    let red_tiles_compressed = red_tiles
        .iter()
        .map(|position| grid_compressor.compressed_coordinate(position).unwrap())
        .collect::<Vec<_>>();

    let borders: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = red_tiles_compressed
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| {
            (
                if a.x <= b.x { a.x..=b.x } else { b.x..=a.x },
                if a.y <= b.y { a.y..=b.y } else { b.y..=a.y },
            )
        })
        .collect();

    let floor = Floor::new(red_tiles_compressed.clone(), borders);

    floor
        .red_tiles
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
        .filter_map(|v| {
            let x_range = if v[0].x < v[1].x {
                v[0].x..=v[1].x
            } else {
                v[1].x..=v[0].x
            };
            let y_range = if v[0].y < v[1].y {
                v[0].y..=v[1].y
            } else {
                v[1].y..=v[0].y
            };

            for x in x_range {
                for y in y_range.clone() {
                    if !floor.contains(&UVec2::new(x, y)) {
                        return None;
                    }
                }
            }

            Some(area(
                &grid_compressor.decompress_coordinate(v[0]).unwrap(),
                &grid_compressor.decompress_coordinate(v[1]).unwrap(),
            ))
        })
        .max()
        .unwrap()
        .to_string()
}

#[derive(Debug)]
struct GridCompressor {
    x_map: HashMap<u32, usize>,
    y_map: HashMap<u32, usize>,
    xs: Vec<u32>,
    ys: Vec<u32>,
}

impl GridCompressor {
    fn new(coordinates: &[UVec2]) -> Self {
        let xs = coordinates
            .iter()
            .map(|pos| pos.x)
            .sorted()
            .unique()
            .collect::<Vec<_>>();

        let ys = coordinates
            .iter()
            .map(|pos| pos.y)
            .sorted()
            .unique()
            .collect::<Vec<_>>();

        let x_map = xs.iter().enumerate().map(|(i, x)| (*x, i)).collect();
        let y_map = ys.iter().enumerate().map(|(i, y)| (*y, i)).collect();

        Self {
            x_map,
            y_map,
            xs,
            ys,
        }
    }

    fn compressed_coordinate(&self, uncompressed_position: &UVec2) -> Option<UVec2> {
        let x = self.x_map.get(&uncompressed_position.x)?;
        let y = self.y_map.get(&uncompressed_position.y)?;

        Some(UVec2::new(*x as u32, *y as u32))
    }

    fn decompress_coordinate(&self, compressed_position: &UVec2) -> Option<UVec2> {
        Some(UVec2::new(
            self.xs[compressed_position.x as usize],
            self.ys[compressed_position.y as usize],
        ))
    }
}

#[derive(Debug)]
struct Floor {
    red_tiles: Vec<UVec2>,
    borders: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>,
}

impl Floor {
    fn new(
        red_tile_positions: Vec<UVec2>,
        borders: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>,
    ) -> Self {
        Self {
            red_tiles: red_tile_positions,
            borders,
        }
    }

    fn contains(&self, position: &UVec2) -> bool {
        let result;

        for border in self
            .borders
            .iter()
            .filter(|(_, border_y)| border_y.contains(&position.y))
            .sorted_by(|(a_x, _), (b_x, _)| a_x.start().cmp(b_x.start()))
        {
            if border.0.contains(&position.x) {
                result = true;
                return result;
            }
        }

        self.borders
            .iter()
            .filter(|(_, border_y)| border_y.contains(&position.y))
            .sorted_by(|(a_x, _), (b_x, _)| a_x.start().cmp(b_x.start()))
            .filter(|(border_x, _)| position.x > *border_x.end())
            .count()
            % 2
            != 0
    }
}

fn area(a: &UVec2, b: &UVec2) -> u64 {
    (b.x.abs_diff(a.x) + 1) as u64 * (b.y.abs_diff(a.y) + 1) as u64
}

fn parse(input: &str) -> IResult<&str, Vec<UVec2>> {
    separated_list1(
        line_ending,
        separated_pair(complete::u32, tag(","), complete::u32).map(|(x, y)| UVec2::new(x, y)),
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

        let result = "24";

        assert_eq!(process(input), result);
    }
}
