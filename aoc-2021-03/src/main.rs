fn main() {
    let input = aoc_utils::read_input!(2021, 3);
    let bits: Vec<usize> = input.lines().map(from_binary).collect();
    println!(
        "Part 1: {}",
        part1(&bits, input.lines().next().unwrap().len())
    );
    println!(
        "Part 2: {}",
        part2(&bits, input.lines().next().unwrap().len())
    );
}

fn part1(inputs: &[usize], length: usize) -> usize {
    gamma(inputs, length) * epsilon(inputs, length)
}

fn part2(inputs: &[usize], length: usize) -> usize {
    oxygen_generator_rating(inputs, length) * co2_scubber(inputs, length)
}

fn from_binary(input: &str) -> usize {
    usize::from_str_radix(input, 2).unwrap()
}

fn most_common(inputs: &[usize], position: usize) -> usize {
    let mask = 1 << position;
    let total = inputs.len();
    let ones: usize = inputs.iter().map(|i| (i & mask) >> position).sum();
    (total <= (ones << 1)).into()
}

fn least_common(inputs: &[usize], position: usize) -> usize {
    let mask = 1 << position;
    let total = inputs.len();
    let ones: usize = inputs.iter().map(|i| (i & mask) >> position).sum();
    (total > (ones << 1)).into()
}

fn flip(bits: usize, len: usize) -> usize {
    let mask = (1 << len) - 1;
    (bits ^ mask) & mask
}

fn gamma(inputs: &[usize], length: usize) -> usize {
    let mut most_common_input: usize = 0;

    for position in 0..length {
        most_common_input += most_common(inputs, position) << position;
    }

    most_common_input
}

fn epsilon(inputs: &[usize], length: usize) -> usize {
    flip(gamma(inputs, length), length)
}

fn oxygen_generator_rating(inputs: &[usize], length: usize) -> usize {
    let mut my_inputs: Vec<usize> = inputs.clone().into();

    let mut most_common_input: usize = 0;

    for pos in 0..length {
        let position = length - pos - 1;
        let most_common_digit = most_common(&my_inputs, position);
        most_common_input += most_common_digit << position;

        let mask = 1 << position;
        my_inputs = my_inputs
            .iter()
            .map(|i| *i)
            .filter(|i| ((*i & mask) >> position) == most_common_digit)
            .collect();
    }

    most_common_input
}

fn co2_scubber(inputs: &[usize], length: usize) -> usize {
    let mut my_inputs: Vec<usize> = inputs.clone().into();

    let mut least_common_input: usize = 0;

    for pos in 0..length {
        let position = length - pos - 1;
        let least_common_digit = least_common(&my_inputs, position);
        least_common_input += least_common_digit << position;

        let mask = 1 << position;
        my_inputs = my_inputs
            .iter()
            .map(|i| *i)
            .filter(|i| ((*i & mask) >> position) == least_common_digit)
            .collect();

        if my_inputs.len() == 1 {
            return my_inputs[0];
        }
    }

    least_common_input
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Vec<usize> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| from_binary(s))
        .collect()
    }

    #[test]
    fn converting_from_binary() {
        assert_eq!(from_binary("101010"), 42);
    }

    #[test]
    fn generating_gamma() {
        assert_eq!(gamma(&input(), 5), 22);
    }

    #[test]
    fn generating_epsilon() {
        assert_eq!(epsilon(&input(), 5), 9);
    }

    #[test]
    fn example1() {
        assert_eq!(part1(&input(), 5), 198);
    }

    #[test]
    fn generating_most_common() {
        assert_eq!(most_common(&input(), 4), 1);
        assert_eq!(most_common(&input(), 3), 0);
        assert_eq!(most_common(&input(), 2), 1);
        assert_eq!(most_common(&input(), 1), 1);
        assert_eq!(most_common(&input(), 0), 0);
    }

    #[test]
    fn oxygen_generator_rating_test() {
        assert_eq!(oxygen_generator_rating(&input(), 5), 23);
    }

    #[test]
    fn co2_scubber_test() {
        assert_eq!(co2_scubber(&input(), 5), 10);
    }
}
