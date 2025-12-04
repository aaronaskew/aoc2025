use std::collections::HashMap;

use glam::IVec2;

const DIRECTIONS: [IVec2; 8] = [
    IVec2 { x: -1, y: -1 },
    IVec2 { x: 0, y: -1 },
    IVec2 { x: 1, y: -1 },
    IVec2 { x: -1, y: 0 },
    IVec2 { x: 1, y: 0 },
    IVec2 { x: -1, y: 1 },
    IVec2 { x: 0, y: 1 },
    IVec2 { x: 1, y: 1 },
];

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let grid: HashMap<IVec2, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| (IVec2::new(x as i32, y as i32), ch))
        })
        .collect();

    let mut count = 0;

    for y in 0..input.lines().count() {
        for x in 0..input.lines().next().unwrap().chars().count() {
            if *grid.get(&IVec2::new(x as i32, y as i32)).unwrap() == '@'
                && DIRECTIONS
                    .iter()
                    .filter(|dir| {
                        let position = IVec2::new(x as i32, y as i32) + **dir;
                        if let Some(ch) = grid.get(&position) {
                            *ch == '@'
                        } else {
                            false
                        }
                    })
                    .count()
                    < 4
            {
                count += 1;
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let result = "13";

        assert_eq!(process(input), result);
    }
}
