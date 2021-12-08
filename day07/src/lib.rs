use itertools::Itertools;
use std::str::FromStr;

fn part1(data: &[i32]) -> i32 {
    let median = data.iter().sorted().nth(data.len() / 2).unwrap();
    data.iter().map(|crab| (crab - median).abs()).sum()
}

fn part2(data: &[i32]) -> i32 {
    let mean = data.iter().map(|f| *f as f32).sum::<f32>() / data.len() as f32;
    std::cmp::min(
        data.iter()
            .map(|crab| (1i32..=(crab - mean.floor() as i32).abs()).sum::<i32>())
            .sum(),
        data.iter()
            .map(|crab| (1i32..=(crab - mean.ceil() as i32).abs()).sum::<i32>())
            .sum(),
    )
}

fn parse(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(<i32 as FromStr>::from_str)
        .map(Result::unwrap)
        .collect()
}

pub fn run() {
    println!("day 07:");
    let data = parse(include_str!("input.txt"));
    println!("part 1: {}", part1(&data));
    println!("part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    const TEST: &'static str = "16,1,2,0,4,2,7,1,2,14";
    #[test]
    fn part1() {
        let data = super::parse(TEST);
        assert_eq!(37, super::part1(&data));
    }

    #[test]
    fn part2() {
        let data = super::parse(TEST);
        assert_eq!(168, super::part2(&data));
    }
}
