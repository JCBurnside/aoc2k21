use parse_display::{Display, FromStr};

#[derive(FromStr, Display)]
enum Direction {
    #[display("down {0}")]
    Down(u32),
    #[display("up {0}")]
    Up(u32),
    #[display("forward {0}")]
    Forward(u32),
}

fn parse(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(<Direction as std::str::FromStr>::from_str)
        .map(Result::unwrap)
        .collect()
}

fn part1(data: &[Direction]) -> u32 {
    let (mut distance, mut depth) = (0, 0);
    for step in data {
        match step {
            Direction::Down(d) => depth += d,
            Direction::Forward(d) => distance += d,
            Direction::Up(d) => depth -= d,
        }
    }
    distance as u32 * depth as u32
}

fn part2(data: &[Direction]) -> u32 {
    let (mut distance, mut depth, mut aim) = (0, 0, 0);
    for step in data {
        match step {
            Direction::Down(units) => aim += units,
            Direction::Up(units) => aim -= units,
            Direction::Forward(units) => {
                distance += units;
                depth += aim * units;
            }
        }
    }
    distance as u32 * depth as u32
}

pub fn run() {
    println!("day 02:");
    let data = parse(include_str!("input.txt"));
    println!("part 1: {}", part1(&data));
    println!("part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    const TEST: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    #[test]
    fn part1() {
        let data = super::parse(TEST);
        assert_eq!(150, super::part1(&data))
    }

    #[test]
    fn part2() {
        let data = super::parse(TEST);
        assert_eq!(900, super::part2(&data));
    }
}
