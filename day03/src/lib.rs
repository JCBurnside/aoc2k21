use std::collections::HashMap;

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| u32::from_str_radix(line, 2))
        .map(Result::unwrap)
        .collect()
}

fn part1<const BITS: u32>(data: &[u32]) -> u32 {
    let mut bits = HashMap::<u32, u32>::new();
    for number in data {
        for bit in 0..BITS {
            let mask = 1u32 << bit;
            if number & mask == mask {
                *(bits.entry(bit).or_insert(0)) += 1;
            }
        }
    }
    let mut gamma = 0u32;
    let mut epislon = 0u32;
    let threshold = data.len() / 2;
    for (bit, set_bits) in bits {
        if set_bits > threshold as u32 {
            gamma |= 1 << bit;
        } else {
            epislon |= 1 << bit;
        }
    }
    gamma * epislon
}

fn split_by_bit_pos(data: Vec<u32>, bit_pos: u32) -> (Vec<u32>, Vec<u32>) {
    let mask = 1 << bit_pos;
    (
        data.iter()
            .copied()
            .filter(|num| num & mask == mask)
            .collect(),
        data.iter()
            .copied()
            .filter(|num| num & mask != mask)
            .collect(),
    )
}

fn part2<const BITS: u32>(data: &[u32]) -> (u32, u32, u32) {
    let most_sig_mask = 1 << BITS - 1;
    let (leading1, leading0): (Vec<_>, Vec<_>) = (
        data.iter()
            .copied()
            .filter(|num| num & most_sig_mask == most_sig_mask)
            .collect(),
        data.iter()
            .copied()
            .filter(|num| num & most_sig_mask != most_sig_mask)
            .collect(),
    );
    let cond = leading1.len() > leading0.len();
    let (mut o2gen_rating_canidates, mut co2scrub_rating_canidates) = if cond {
        (leading1, leading0)
    } else {
        (leading0, leading1)
    };
    let mut bit = BITS - 2;
    while o2gen_rating_canidates.len() > 1 {
        let (ones, zeros) = split_by_bit_pos(o2gen_rating_canidates, bit);
        o2gen_rating_canidates = if ones.len() >= zeros.len() {
            ones
        } else {
            zeros
        };
        bit = bit.saturating_sub(1);
    }

    let mut bit = BITS - 2;
    while co2scrub_rating_canidates.len() > 1 {
        let (ones, zeros) = split_by_bit_pos(co2scrub_rating_canidates, bit);
        co2scrub_rating_canidates = if ones.len() < zeros.len() {
            ones
        } else {
            zeros
        };
        bit = bit.saturating_sub(1);
    }
    let o2 = o2gen_rating_canidates.first().unwrap();
    let co2 = co2scrub_rating_canidates.first().unwrap();
    (o2 * co2, *o2, *co2)
}

pub fn run() {
    println!("day 03:");
    let data = parse(include_str!("input.txt"));
    println!("part 1: {}", part1::<12>(&data));
    println!("part 2: {:?}", part2::<12>(&data));
}

#[cfg(test)]
mod tests {
    const TEST: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    #[test]
    fn part1() {
        let data = super::parse(TEST);
        assert_eq!(198, super::part1::<5>(&data))
    }

    #[test]
    fn part2() {
        let data = super::parse(TEST);
        assert_eq!((230, 23, 10), super::part2::<5>(&data));
    }
}
