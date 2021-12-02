use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref UP_REGEX: Regex = Regex::new(r"up (?P<units>\d+)").unwrap();
    static ref DOWN_REGEX: Regex = Regex::new(r"down (?P<units>\d+)").unwrap();
    static ref FORWARD_REGEX: Regex = Regex::new(r"forward (?P<units>\d+)").unwrap();
}

enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

struct Position1 {
    x: u64,
    y: u64,
}

impl Position1 {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn up(self: &mut Self, units: &u64) {
        self.y -= units;
    }

    fn down(self: &mut Self, units: &u64) {
        self.y += units;
    }

    fn forward(self: &mut Self, units: &u64) {
        self.x += units;
    }
}

struct Position2 {
    x: u64,
    y: u64,
    aim: i64,
}

impl Position2 {
    fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }

    fn up(self: &mut Self, units: &u64) {
        self.aim -= i64::try_from(*units).unwrap();
    }

    fn down(self: &mut Self, units: &u64) {
        self.aim += i64::try_from(*units).unwrap();
    }

    fn forward(self: &mut Self, units: &u64) {
        self.x += units;
        self.y += u64::try_from(self.aim * i64::try_from(*units).unwrap()).unwrap();
    }
}

fn main() {
    let input = aoc_utils::read_input!(2021, 2);
    let commands: Vec<Command> = input.lines().map(parse_command).collect();
    println!("Part 1: {}", part1(&commands));
    println!("Part 2: {}", part2(&commands));
}

fn part1(commands: &[Command]) -> u64 {
    let mut position = Position1::new();
    for command in commands {
        match command {
            Command::Up(units) => position.up(units),
            Command::Down(units) => position.down(units),
            Command::Forward(units) => position.forward(units),
        }
    }

    position.x * position.y
}

fn part2(commands: &[Command]) -> u64 {
    let mut position = Position2::new();
    for command in commands {
        match command {
            Command::Up(units) => position.up(units),
            Command::Down(units) => position.down(units),
            Command::Forward(units) => position.forward(units),
        }
    }

    position.x * position.y
}

fn parse_command(input: &str) -> Command {
    if let Some(capture) = UP_REGEX.captures(&input) {
        return Command::Up(capture.name("units").unwrap().as_str().parse().unwrap());
    } else if let Some(capture) = DOWN_REGEX.captures(&input) {
        return Command::Down(capture.name("units").unwrap().as_str().parse().unwrap());
    } else if let Some(capture) = FORWARD_REGEX.captures(&input) {
        return Command::Forward(capture.name("units").unwrap().as_str().parse().unwrap());
    } else {
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&[
                Command::Forward(5),
                Command::Down(5),
                Command::Forward(8),
                Command::Up(3),
                Command::Down(8),
                Command::Forward(2),
            ]),
            150
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            part2(&[
                Command::Forward(5),
                Command::Down(5),
                Command::Forward(8),
                Command::Up(3),
                Command::Down(8),
                Command::Forward(2),
            ]),
            900
        );
    }
}
