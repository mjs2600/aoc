use std::num::ParseIntError;

fn main() {
    let input = aoc_utils::read_input!(2021, 1);
    let measurements: Vec<u64> = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()
        .expect("Invalid input string");
    println!("Depth Increased: {}", part1(&measurements));
    println!("Depth Window Increased: {}", part2(&measurements));
}

fn part1(measurements: &[u64]) -> u64 {
    let mut decreases: u64 = 0;
    let mut current_depth = measurements[0];
    for depth in measurements {
        if *depth > current_depth {
            decreases += 1;
        }
        current_depth = *depth;
    }

    decreases
}

fn part2(measurements: &[u64]) -> u64 {
    let mut decreases: u64 = 0;
    let mut current_depth: u64 = measurements[0..3].iter().sum();
    for depth in measurements.windows(3) {
        let new_depth = depth.iter().sum();
        if new_depth > current_depth {
            decreases += 1;
        }
        current_depth = new_depth;
    }

    decreases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}
