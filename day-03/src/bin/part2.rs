fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    input.lines().map(max_joltage).sum::<u64>().to_string()
}

fn max_joltage(bank: &str) -> u64 {
    const NUM_BATTERIES: usize = 12;

    let mut batteries: Vec<char> = vec![];

    let mut start_idx = 0;

    for i in 0..NUM_BATTERIES {
        let (index, battery) = bank[start_idx..(bank.len() - NUM_BATTERIES + 1 + i)]
            .chars()
            .enumerate()
            .max_by(|(ia, a), (ib, b)| match a.cmp(b) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => ib.cmp(ia),
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            })
            .expect("should be able to get max");

        batteries.push(battery);

        start_idx = start_idx + index + 1;
    }

    batteries
        .iter()
        .collect::<String>()
        .parse()
        .expect("shoudl parse")
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

        let result = "3121910778619";

        assert_eq!(process(input), result);
    }
}
