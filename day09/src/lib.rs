use std::iter::once;

use itertools::Itertools;


#[derive(PartialEq, Eq,Debug)]
pub struct Floor {
    pub floor : Vec<u8>,
    pub size : (usize,usize),
}

#[derive(Clone, Copy,PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn get_change_in_coords(&self) -> (i8,i8) {
        match self {
            Direction::North => (0,1),
            Direction::East => (1,0),
            Direction::South => (0,-1),
            Direction::West => (-1,0),
        }
    }

    fn get_oposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl Floor {
    // u8::MAX represents it was at an edge
    fn get_adjacent(&self,point:(usize,usize)) -> [u8;4] {
        [
            point.1.checked_sub(1).map(|y| self.floor[point.0 + (y * self.size.0)]).unwrap_or(u8::MAX),//up
            point.0.checked_sub(1).map(|x| self.floor[x + (point.1 * self.size.0)]).unwrap_or(u8::MAX),//left
            if point.0 == self.size.0 - 1 { u8::MAX } else { self.floor[(point.0 +1) + (point.1 * self.size.0)] }, //right
            if point.1 == self.size.1 - 1 { u8::MAX } else { self.floor[point.0 + ((point.1 +1) * self.size.0)] } //down 
        ]
    }

    

    fn get_idx(&self,point:(usize,usize))->usize {
        point.0 + (self.size.0 * point.1)
    }

    fn get_adjacent_until_9(&mut self,point:(usize,usize)) -> Vec<u8> {
        let idx = self.get_idx(point);
        let value = self.floor[idx];
        if value == 9 {
            return vec![];
        }
        self.floor[idx] = 9;
        [Direction::North,Direction::South,Direction::West,Direction::East]
            .iter()
            .map(Direction::get_change_in_coords)
            .filter_map(|(change_x,change_y)| if (change_x as i32 + point.0 as i32) < 0 || (change_y as i32 + point.1 as i32) < 0 {
                None
            } else if (change_x as i32 + point.0 as i32) >= self.size.0 as i32|| (change_y as i32 + point.1 as i32) >= self.size.1 as i32{
                None
            } else {
                Some(self.get_adjacent_until_9(((change_x as i32 + point.0 as i32) as usize, (change_y as i32 + point.1 as i32) as usize)))
            })
            .flatten()
            .chain(once(value))
            .collect_vec()
            
    }
}

fn part1(floor:&Floor) -> usize {
    (0..floor.size.0).cartesian_product(0..floor.size.1).filter_map(|p| {
        let value = floor.floor[p.0+(p.1*floor.size.0)];
        if floor.get_adjacent(p).iter().all(|adj| &value < adj) {
            Some(value as usize + 1)
        } else {
            None
        }
    }).sum()
    
}

fn part2(mut floor:Floor) -> usize {
    
    (0..floor.size.0).cartesian_product(0..floor.size.1).map(|basin| floor.get_adjacent_until_9(basin)).map(|v| v.len()).sorted().rev().take(3).product()
}

pub fn parse(input:&str)-> Floor {
    let lines = input.lines();
    let y = lines.clone().count();
    let x = lines.clone().next().unwrap().len();
    Floor { floor: lines.flat_map(|line| {
        debug_assert_eq!(line.len(),x,"Line legnths are not the same. Invalid data");
        line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>()
    }).collect(), size: (x,y) }
}

pub fn run() {
    println!("day 05:");
    let floor = parse(include_str!("input.txt"));
    println!("part 1: {}", part1(&floor));
    println!("part 2: {}", part2(floor));
}

#[cfg(test)]
mod tests {

    const TEST_DATA : &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    #[test]
    fn parse() {
        assert_eq!(super::Floor {
            floor : vec![
                1,2,3,
                4,5,6,
                7,8,9
            ],
            size:(3,3)
        }, 
super::parse(
"123
456
789")
);
    }

    #[test]
    fn part1() {
        let data = super::parse(TEST_DATA);
        assert_eq!(15,super::part1(&data));
    }

    #[test]
    fn part2() {
        let data = super::parse(TEST_DATA);
        assert_eq!(1134,super::part2(data));
    }
}
