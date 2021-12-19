use std::{collections::HashMap};

#[derive(PartialEq, Eq,Clone, Debug,Hash, PartialOrd, Ord)]
enum CavePoint {
    Start,
    End,
    Big(&'static str),
    Small(&'static str)
}

impl From<&'static str> for CavePoint {
    fn from(s: &'static str) -> Self {
        match s {
            "start" => CavePoint::Start,
            "end" => CavePoint::End,
            s if s.chars().all(|c| c.is_ascii_lowercase()) => CavePoint::Small(s),
            s if s.chars().all(|c| c.is_ascii_uppercase()) => CavePoint::Big(s),
            _=>unreachable!()
        }
    }
}

#[derive(PartialEq, Eq,Debug)]
struct Cave {
    paths : HashMap<CavePoint,Vec<CavePoint>>
}

fn parse(input : &'static str) -> Cave {
    let mut paths = HashMap::with_capacity(input.lines().count());

    for line in input.lines() {
        let (a,b) = line.split_once('-').unwrap();
        let a : CavePoint = a.into();
        let b : CavePoint = b.into();
        (*paths.entry(a.clone()).or_insert(vec![])).push(b.clone());
        (*paths.entry(b).or_insert(vec![])).push(a);
    }
    
    paths.values_mut().for_each(|v| v.sort_unstable()); 
    Cave { paths }
}

fn recursive_search(paths:&HashMap<CavePoint,Vec<CavePoint>>, point:CavePoint,visited : Vec<CavePoint>) -> usize {
    paths[&point].iter().filter_map(|test| {
        // println!("branch {:?}",test);
        let mut visited = visited.clone();
        match test {
            CavePoint::Big(_) => {
                visited.push(test.clone());
                Some(recursive_search(paths, test.clone(), visited))
            }
            CavePoint::Small(_) => {
                if !visited.contains(test) {
                    visited.push(test.clone());
                    Some(recursive_search(paths, test.clone(), visited))
                } else {
                    // println!("Dead end {:?}",visited);
                    None
                }
            }
            CavePoint::End =>{ 
                Some(1) 
            },
            CavePoint::Start => {
                // println!("Dead end {:?}",visited);
                None
            }
        }
    }).sum()
}

fn part1(cave : &Cave) -> usize {
    cave.paths[&CavePoint::Start].iter().map(|point| {
        // println!("start\n{:?}",point);
        let visited = vec![CavePoint::Start,point.clone()];
        recursive_search(&cave.paths, point.clone(), visited)
    }).sum()
}

fn recursive_search_v2(paths:&HashMap<CavePoint,Vec<CavePoint>>, point:CavePoint,visited : Vec<CavePoint>, small_used:bool)-> usize {
    paths[&point].iter().filter_map(|test| {
        // println!("branch {:?}",test);
        let mut visited = visited.clone();
        match test {
            CavePoint::Big(_) => {
                visited.push(test.clone());
                Some(recursive_search_v2(paths, test.clone(), visited,small_used))
            }
            CavePoint::Small(_) if small_used => {
                if !visited.contains(test) {
                    visited.push(test.clone());
                    Some(recursive_search_v2(paths, test.clone(), visited,true))
                } else {
                    // println!("Dead end {:?}",visited);
                    None
                }
            }
            CavePoint::Small(_)=> {
                let dupe = visited.contains(test);
                visited.push(test.clone());
                Some(recursive_search_v2(paths, test.clone(), visited, dupe))
            }
            CavePoint::End =>{ 
                Some(1) 
            },
            CavePoint::Start => {
                // println!("Dead end {:?}",visited);
                None
            }
        }
    }).sum()
}

fn part2(cave : &Cave) -> usize {
    cave.paths[&CavePoint::Start].iter().map(|point| {
        // println!("start\n{:?}",point);
        let visited = vec![CavePoint::Start,point.clone()];
        recursive_search_v2(&cave.paths, point.clone(), visited,false)
    }).sum()
}

pub fn run() {
    println!("day 12:");
    let cave = parse(include_str!("input.txt"));
    println!("part 1: {}", part1(&cave));
    println!("part 2: {}", part2(&cave))
}

#[cfg(test)]
mod tests {
    use crate::CavePoint;

    

    const TEST_DATA_SMALL : &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    const TEST_DATA_MED : &'static str ="dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    const TEST_DATA_LARGE : &'static str ="fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    #[test]
    fn parse() {
        use hmap::*;
        let expected = {
            let mut out = super::Cave {
                paths : hmap!(
                    CavePoint::Big("A") => vec![CavePoint::Start,CavePoint::Small("b"),CavePoint::Small("c"),CavePoint::End],
                    CavePoint::Small("c") => vec![CavePoint::Big("A")],
                    CavePoint::Start => vec![CavePoint::Big("A"),CavePoint::Small("b")],
                    CavePoint::Small("b") => vec![CavePoint::Start,CavePoint::Big("A"),CavePoint::Small("d"),CavePoint::End],
                    CavePoint::End => vec![CavePoint::Big("A"),CavePoint::Small("b")],
                    CavePoint::Small("d") => vec![CavePoint::Small("b")]
                )
            };
            out.paths.values_mut().for_each(|v| v.sort_unstable());
            out
        };
        let actual = {
            let mut out = super::parse(TEST_DATA_SMALL);
            out.paths.values_mut().for_each(|v| v.sort_unstable());
            out
        };
        assert_eq!(expected,actual)
    }

    #[test]
    fn part1() {
        assert_eq!(10, super::part1(&super::parse(TEST_DATA_SMALL)));
        assert_eq!(19, super::part1(&super::parse(TEST_DATA_MED)));
        assert_eq!(226,super::part1(&super::parse(TEST_DATA_LARGE)));
    }

    #[test]
    fn part2() {
        assert_eq!(36,  super::part2(&super::parse(TEST_DATA_SMALL)));
        assert_eq!(103, super::part2(&super::parse(TEST_DATA_MED)));
        assert_eq!(3509,super::part2(&super::parse(TEST_DATA_LARGE)));
    }
}
