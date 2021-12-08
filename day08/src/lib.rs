use std::str::FromStr;

use itertools::Itertools;

/* format
  0:              2
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5                               9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
 */
#[derive(PartialEq, Eq, Debug)]
struct DisplaySignalMap {
    map: [String; 10],
}

impl FromStr for DisplaySignalMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: [String; 10] = Default::default();
        let mut numbers: Vec<String> = s
            .trim_end()
            .split(' ')
            .map(|s| s.chars().sorted().collect())
            .collect();
        debug_assert_eq!(numbers.len(), 10);
        let one = numbers
            .iter()
            .position(|segment| segment.len() == 2)
            .unwrap();
        map[1] = numbers.remove(one);
        let seven = numbers
            .iter()
            .position(|segment| segment.len() == 3)
            .unwrap();
        map[7] = numbers.remove(seven);
        let four = numbers.iter().position(|seg| seg.len() == 4).unwrap();
        map[4] = numbers.remove(four);
        let eight = numbers.iter().position(|seg| seg.len() == 7).unwrap();
        map[8] = numbers.remove(eight);

        let nine = numbers
            .iter()
            .position(|seg| !map.iter().contains(seg) && map[4].chars().all(|c| seg.contains(c)))
            .unwrap();
        map[9] = numbers.remove(nine);
        let zero = numbers
            .iter()
            .position(|seg| seg.len() == 6 && map[1].chars().all(|c| seg.contains(c)))
            .unwrap();
        map[0] = numbers.remove(zero);
        let three = numbers
            .iter()
            .position(|seg| seg.chars().filter(|c| !map[7].contains(*c)).count() == 2)
            .unwrap();
        map[3] = numbers.remove(three);

        let six = numbers
            .iter()
            .position(|seg| seg.len() == 6 && seg.chars().any(|c| !map[1].contains(c)))
            .unwrap();
        map[6] = numbers.remove(six);
        let five = numbers
            .iter()
            .position(|seg| seg.chars().all(|c| map[9].contains(c)))
            .unwrap();
        map[5] = numbers.remove(five);
        let two = numbers.iter().position(|seg| seg.len() == 5).unwrap();
        map[2] = numbers.remove(two);

        Ok(Self { map })
    }
}
impl DisplaySignalMap {
    fn map(self, digits: &str) -> u32 {
        digits
            .split_ascii_whitespace()
            .map(|seg| seg.chars().sorted().collect::<String>())
            .map(|seg| self.map.iter().position(|m| 
                m == &seg
            ).unwrap())
            .rev()
            .enumerate()
            .map(|(pos, value)| value as u32 * 10u32.pow(pos as u32))
            .sum()
    }
}

fn part1(input: &str) -> usize {
    //use raw string because that is the easiest
    input
        .lines()
        .map(|line| {
            line.split('|')
                .nth(1)
                .unwrap()
                .split(' ')
                .map(str::len)
                .filter(|len| matches!(len, 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

fn part2(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let mut bar = line.split(" | ");
            (bar.next().unwrap(), bar.next().unwrap())
        })
        .map(|(map, value)| (map.parse::<DisplaySignalMap>().unwrap(), value))
        .map(|(map, value)| map.map(value))
        .sum()
}

pub fn run() {
    const DATA: &'static str = include_str!("input.txt");
    println!("day 08:");
    println!("part 1: {}", part1(DATA));
    println!("part 2: {}", part2(DATA));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use itertools::Itertools;

    use crate::DisplaySignalMap;

    const SINGLE_TEST: &'static str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    const MULTI_TEST: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    #[test]
    fn part1() {
        assert_eq!(26, super::part1(MULTI_TEST));
    }

    #[test]
    fn mapper() {
        let line = SINGLE_TEST.split('|').next().unwrap();
        assert_eq!(
            [
                "abcdeg".to_string(),  //0
                "ab".to_string(),      //1
                "acdfg".to_string(),   //2
                "abcdf".to_string(),   //3
                "abef".to_string(),    //4
                "bcdef".to_string(),   //5
                "bcdefg".to_string(),  //6
                "abd".to_string(),     //7
                "abcdefg".to_string(), //8
                "abcdef".to_string()
            ],
            DisplaySignalMap::from_str(line).unwrap().map
        )
    }

    #[test]
    fn part2() {
        assert_eq!(5353, super::part2(SINGLE_TEST));
        assert_eq!(61229, super::part2(MULTI_TEST));
    }
}
