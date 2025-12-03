fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    input.lines().map(max_joltage).sum::<u64>().to_string()
}

fn max_joltage(bank: &str) -> u64 {
    let (i_first, first) = &bank[..(bank.len() - 1)]
        .chars()
        .enumerate()
        .max_by(|(ia, a), (ib, b)| match a.cmp(b) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => ib.cmp(ia),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        })
        .expect("max should be gettable");

    let second = &bank[(*i_first + 1)..]
        .chars()
        .max()
        .expect("max should be gettable");

    format!("{first}{second}").parse().expect("should parse")
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
