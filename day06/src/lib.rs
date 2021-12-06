use std::str::FromStr;

type Fish = [usize;9];

fn parse(input:&str) -> Fish {
    let mut output = [0;9];
    for fish in input.split(',').map(<u8 as FromStr>::from_str).map(Result::unwrap) {
        output[fish as usize] += 1;
    }
    output
}

fn run_for(mut fish:Fish,days:usize) -> usize {
    let mut swap = [0;9];
    for _ in 0..days {
        for (group,count) in fish.iter().enumerate(){
            match group {
                0=> {
                    swap[8]=*count;
                    swap[6]+=count;
                }
                1..=8 => {
                    swap[group-1] += count;
                }
                _=>unreachable!()
            }
        }
        fish = swap;
        swap = [0;9];
    }
    fish.iter().sum()
}

fn part1(fish:Fish) -> usize { 
    run_for(fish, 80)
}

fn part2(fish:Fish) -> usize {
    run_for(fish, 256)
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
