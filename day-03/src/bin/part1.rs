use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    input.lines().map(max_joltage).sum::<u32>().to_string()
}

fn max_joltage(bank: &str) -> u32 {
    let bank: Vec<u32> = bank.chars().filter_map(|ch| ch.to_digit(10)).collect();

    assert!(bank.len() >= 2);

    (0..(bank.len() - 1))
        .cartesian_product(1..bank.len())
        .filter_map(|(i_left, i_right)| {
            if i_left < i_right {
                Some(bank[i_left] * 10 + bank[i_right])
            } else {
                None
            }
        })
        .max()
        .expect("should find max")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        let result = "357";

        assert_eq!(process(input), result);
    }
}
