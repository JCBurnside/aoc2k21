use std::{str::FromStr, collections::HashMap, cmp::Ordering};


#[derive(parse_display::Display,parse_display::FromStr,Clone, Copy, Hash, PartialEq, Eq, Ord, Debug)]
#[display("{0},{1}")]
struct Point(i32,i32);

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.1.partial_cmp(&other.1) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.0.partial_cmp(&other.0)
    }
}


#[derive(parse_display::Display,parse_display::FromStr,Clone, Copy, Debug, PartialEq, Eq)]
#[display("{start} -> {end}")]
struct Line {
    start : Point,
    end : Point,
}
fn coerce_one(lhs:i32,rhs:i32) -> i32 {
    match lhs.cmp(&rhs) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

impl Line {
    fn is_straight(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn get_line_points(&self) -> Vec<Point> {
        let mut current_point = self.start;
        let change = (coerce_one(self.start.0, self.end.0),coerce_one(self.start.1,self.end.1));
        let mut out = vec![self.start];
        while current_point != self.end {
            current_point.0 += change.0;
            current_point.1 += change.1;
            out.push(current_point);
        }
        out
    }
}

fn part1(vents:&[Line]) -> usize {
    let mut map = HashMap::<Point,u8>::new();
    for vent in vents.iter().copied().filter(Line::is_straight) {
        for point in vent.get_line_points() {
            *map.entry(point).or_insert(0) += 1;
        }
    }
    
    map.values().filter(|v| v>=&&2).count()
}

fn part2(vents:&[Line])->usize {
    let mut map = HashMap::<Point,u8>::new();
    for vent in vents{
        for point in vent.get_line_points() {
            *map.entry(point).or_insert(0) += 1;
        }
    }
    
    map.values().filter(|v| v>=&&2).count()
}

fn parse(input:&str) -> Vec<Line> {
    input.lines().map(Line::from_str).map(Result::unwrap).collect()
}

pub fn run() {
    println!("day 05:");
    let data = parse(include_str!("input.txt"));
    println!("part 1: {}", part1(&data));
    println!("part 2: {}", part2(&data));
}


#[cfg(test)]
mod tests {
    const TEST : &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    #[test]
    fn part1() {
        let parsed = super::parse(TEST);
        assert_eq!(5,super::part1(&parsed));
    }

    #[test]
    fn part2() {
        let parsed = super::parse(TEST);
        assert_eq!(12,super::part2(&parsed));
    }

    #[test]
    fn parse() {
        use super::{Line,Point};
        let parse = super::parse(TEST);
        assert_eq!(parse, vec![
            Line { start : Point(0,9), end : Point(5,9) },
            Line { start : Point(8,0), end : Point(0,8) },
            Line { start : Point(9,4), end : Point(3,4) },
            Line { start : Point(2,2), end : Point(2,1) },
            Line { start : Point(7,0), end : Point(7,4) },
            Line { start : Point(6,4), end : Point(2,0) },
            Line { start : Point(0,9), end : Point(2,9) },
            Line { start : Point(3,4), end : Point(1,4) },
            Line { start : Point(0,0), end : Point(8,8) },
            Line { start : Point(5,5), end : Point(8,2) },
        ])
    }

    #[test]
    fn line_test() {
        use super::Point;
        let parsed = super::parse(TEST);
        for (actual,expected) in parsed.into_iter().map(|line| line.get_line_points()).zip(vec![
            vec![Point(0,9),Point(1,9),Point(2,9),Point(3,9),Point(4,9),Point(5,9),],
            vec![Point(8,0),Point(7,1),Point(6,2),Point(5,3),Point(4,4),Point(3,5),Point(2,6),Point(1,7),Point(0,8),],
            vec![Point(9,4),Point(8,4),Point(7,4),Point(6,4),Point(5,4),Point(4,4),Point(3,4),],
            vec![Point(2,2),Point(2,1),],
            vec![Point(7,0),Point(7,1),Point(7,2),Point(7,3),Point(7,4),],
            vec![Point(6,4),Point(5,3),Point(4,2),Point(3,1),Point(2,0),],
            vec![Point(0,9),Point(1,9),Point(2,9),],
            vec![Point(3,4),Point(2,4),Point(1,4),],
            vec![Point(0,0),Point(1,1),Point(2,2),Point(3,3),Point(4,4),Point(5,5),Point(6,6),Point(7,7),Point(8,8),],
            vec![Point(5,5),Point(6,4),Point(7,3),Point(8,2),],
        ].into_iter()) {
            assert_eq!(expected,actual);
        }
    }
}
