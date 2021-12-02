use itertools::Itertools;
use std::str::FromStr;
fn part1(data: &[u32]) -> u32 {
    data.iter()
        .tuple_windows()
        .filter(|(lhs, rhs)| lhs < rhs)
        .count() as u32
}
fn part2(data: &[u32]) -> u32 {
    data.iter()
        .tuple_windows()
        .map(|(one, two, three)| one + two + three)
        .tuple_windows()
        .filter(|(lhs, rhs)| lhs < rhs)
        .count() as u32
}
fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(<u32 as FromStr>::from_str)
        .map(Result::unwrap)
        .collect()
}

pub fn run() {
    let data = parse(include_str!("input.txt"));
    println!("DAY 01:");
    println!("part 1: {}", part1(&data));
    println!("part 2: {}", part2(&data));
}

#[cfg(test)]
mod test {
    const TEST: &'static str = "199
200
208
210
200
207
240
269
260
263";
    #[test]
    fn part1() {
        let data = super::parse(TEST);
        assert_eq!(7, super::part1(&data));
    }

    #[test]
    fn part2() {
        let data = super::parse(TEST);
        assert_eq!(5, super::part2(&data));
    }
}
