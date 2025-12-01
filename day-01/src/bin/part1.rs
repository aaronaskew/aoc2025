fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut value: u8 = 50;

    input
        .lines()
        .filter(move |line| {
            value = turn(value, line);
            value == 0
        })
        .count()
        .to_string()
}

fn turn(start: u8, instruction: &str) -> u8 {
    let (direction, ticks) = instruction.split_at(1);

    let direction = direction.chars().next().unwrap();
    let ticks: u32 = ticks.parse().unwrap();

    (start as i32
        + match direction {
            'L' => -(ticks as i32),
            'R' => ticks as i32,
            _ => panic!("not allowed"),
        })
    .rem_euclid(100) as u8
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        let result = "3";

        assert_eq!(process(input), result);
    }
}
