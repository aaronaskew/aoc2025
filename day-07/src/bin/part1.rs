use std::collections::HashSet;

use glam::IVec2;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut splitters = HashSet::new();

    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();
    let mut start_pos = IVec2::NEG_ONE;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.char_indices() {
            if ch == '^' {
                splitters.insert(IVec2::new(x as i32, y as i32));
            } else if ch == 'S' {
                start_pos = IVec2::new(x as i32, y as i32);
            }
        }
    }

    let mut visited_splitters = HashSet::new();

    let mut active_beams = HashSet::new();

    active_beams.insert(start_pos);

    let mut splits = 0;

    while !active_beams.is_empty() {
        for y in 0..num_rows {
            for x in 0..num_cols {
                let pos = IVec2::new(x as i32, y as i32);

                if active_beams.contains(&pos) {
                    if y < num_rows - 1 {
                        for row in (y + 1)..num_rows {
                            let beam_pos = IVec2::new(x as i32, row as i32);
                            if splitters.contains(&beam_pos) {
                                if !visited_splitters.contains(&beam_pos) {
                                    active_beams.insert(beam_pos + IVec2::NEG_X);
                                    active_beams.insert(beam_pos + IVec2::X);

                                    // TODO might need to check for dupes here

                                    splits += 1;

                                    visited_splitters.insert(beam_pos);
                                }

                                break;
                            }
                        }
                    }

                    active_beams.remove(&pos);
                }
            }
        }
    }

    splits.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let result = "21";

        assert_eq!(process(input), result);
    }
}
