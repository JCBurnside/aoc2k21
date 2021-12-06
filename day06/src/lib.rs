use std::{str::FromStr, collections::HashMap};

use itertools::Itertools;

#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
struct Fish(u8);

impl Fish {
    fn new() -> Self {
        Self(8)
    }

    fn next_day(&mut self) -> Option<Self> {
        if self.0 == 0 {
            self.0 = 6;
            Some(Self::new())
        } else {
            self.0 -= 1;
            None
        }
    }
}

fn parse(input:&str) -> Vec<Fish> {
    input.split(',').map(<u8 as FromStr>::from_str).map(Result::unwrap).map(Fish).collect()
}

fn run_for(mut fish:Vec<Fish>,days:usize) -> usize {
    for _ in 0..days {
        let mut to_add = Vec::new();
        for fish in fish.iter_mut() {
            if let Some(fish) = fish.next_day() {
                to_add.push(fish);
            }
        }
        fish.append(&mut to_add);
    }

    fish.len()
}

fn part1(fish:Vec<Fish>) -> usize { 
    run_for(fish, 80)
}

fn part2(fish:Vec<Fish>) -> usize {
    let mut map = HashMap::<u8,usize>::from_iter(fish.iter().sorted().group_by(|f| f.0).into_iter().map(|(day,fish)| (day,fish.count())));
    let mut swap : HashMap<u8,usize> = HashMap::with_capacity(9);
    for _ in 0..256 {
        for (group,count) in map.iter().sorted_by_key(|(day,_)| -(**day as i8)){
            match group {
                0=> {
                    *swap.entry(8).or_default() = *count;
                    *swap.entry(6).or_default() += *count;
                }
                1..=8 => {
                    *swap.entry( *group - 1).or_default() += count;
                }
                _=>unreachable!()
            }
        }
        map = swap;
        swap = HashMap::with_capacity(9);
    }
    map.values().sum()
}

pub fn run() {
    println!("day 06:");
    let fish = parse(include_str!("input.txt"));
    println!("part 1: {}",part1(fish.clone()));
    println!("part 2: {}",part2(fish))
}

#[cfg(test)]
mod tests {

    const TEST : &'static str = "3,4,3,1,2";
    #[test]
    fn part1() {
        let fish = super::parse(TEST);
        assert_eq!(5934,super::part1(fish));
    }

    #[test]
    fn part2() {
        let fish = super::parse(TEST);
        assert_eq!(26984457539, super::part2(fish));
    }
}
