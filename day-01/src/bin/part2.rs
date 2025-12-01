fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut value: u8 = 50;

    input
        .lines()
        .map(move |line| {
            let zeroes;
            (value, zeroes) = get_zeroes(value, line);
            zeroes
        })
        .sum::<u32>()
        .to_string()
}

fn get_zeroes(start: u8, instruction: &str) -> (u8, u32) {
    let mut zeroes = 0;

    let (direction, ticks) = instruction.split_at(1);

    let direction = direction.chars().next().unwrap();
    let mut ticks: u32 = ticks.parse().unwrap();

    let revolutions: u32 = ticks / 100;

    zeroes += revolutions;

    ticks -= revolutions * 100;

    if direction == 'R' {
        if start + ticks as u8 >= 100 {
            zeroes += 1;
        }
    } else if start as i32 - ticks as i32 <= 0 && start != 0 {
        zeroes += 1;
    }

    (
        (start as i32
            + match direction {
                'L' => -(ticks as i32),
                'R' => ticks as i32,
                _ => panic!("not allowed"),
            })
        .rem_euclid(100) as u8,
        zeroes,
    )
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

        let result = "6";

        assert_eq!(process(input), result);
    }
}
