use std::collections::{HashMap, HashSet};

use glam::IVec2;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut splitters = HashSet::new();

    let num_rows = input.lines().count();
    // let num_cols = input.lines().next().unwrap().chars().count();
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

    let mut manifold = Manifold {
        num_rows,
        splitters,
        cache: Default::default(),
    };

    manifold.solve(start_pos).to_string()
}

impl Manifold {
    fn solve(&mut self, pos: IVec2) -> u64 {
        if let Some(solution) = self.cache.get(&pos) {
            *solution
        } else {
            let solution = if pos.y as usize >= self.num_rows - 1 {
                1
            } else if self.splitters.contains(&pos) {
                self.solve(pos + IVec2::NEG_X) + self.solve(pos + IVec2::X)
            } else {
                self.solve(pos + IVec2::Y)
            };

            self.cache.insert(pos, solution);

            solution
        }
    }
}

struct Manifold {
    num_rows: usize,
    splitters: HashSet<IVec2>,
    cache: HashMap<IVec2, u64>,
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

        let result = "40";

        assert_eq!(process(input), result);
    }
}
