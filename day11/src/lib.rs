use std::{thread, time::Duration};

use itertools::Itertools;

fn parse(input : &str) -> [[u8;10];10] {
    let mut output = [[0u8;10];10];
    for (y,line) in input.lines().enumerate() {
        debug_assert_eq!(10,line.len());
        for (x, it) in line.char_indices() {
            output[y][x] = it.to_digit(10).unwrap() as u8;
        }
    }
    output
}

fn part1(mut field: [[u8;10];10]) -> usize {
    let term = console::Term::stdout();
    let pallet = colorous::VIRIDIS;
    let mut flashes = 0;
    for i in 0..100 {
        // println!("----------");
        field.iter_mut().flatten().for_each(|octo| *octo += 1);
        for (y,x) in (0..10).cartesian_product(0..10) {
            // println!("({},{})",x,y);
            
            if field[y][x] == 10 {
                flash(&mut field,x,y)
            }
            
        }

        flashes += field.iter().flatten().filter(|octo| octo > &&9).count();
        field.iter_mut().flatten().filter(|octo| octo >= &&mut 10).for_each(|octo| *octo = 0);

        for row in field {
            for octo in row {
                let color = pallet.eval_rational((9 - octo) as usize, 9);
                term.write_str(&format!("\x1b[48;2;{};{};{}m ",color.r,color.g,color.b)).unwrap();
            }
            term.write_line("\x1b[0m ").unwrap();   
        }
        if i != 99 {

            term.move_cursor_up(10).unwrap();
            term.move_cursor_left(11).unwrap();
        }
        thread::sleep(Duration::from_millis(100))
    }

    flashes
}

fn part2(mut field: [[u8;10];10]) -> usize {
    let term = console::Term::stdout();
    let pallet = colorous::VIRIDIS;
    let mut step = 0;
    while !field.iter().flatten().all(|octo| octo == &0) {
        field.iter_mut().flatten().for_each(|octo| *octo += 1);
        for (y,x) in (0..10).cartesian_product(0..10) {
            // println!("({},{})",x,y);
            
            if field[y][x] == 10 {
                flash(&mut field,x,y)
            }
            
        }

        
        field.iter_mut().flatten().filter(|octo| octo >= &&mut 10).for_each(|octo| *octo = 0);

        for row in field {
            for octo in row {
                let color = pallet.eval_rational((9 - octo) as usize, 9);
                term.write_str(&format!("\x1b[48;2;{};{};{}m ",color.r,color.g,color.b)).unwrap();
            }
            term.write_line("\x1b[0m ").unwrap();   
        }
        if !field.iter().flatten().all(|octo| octo == &0) {

            term.move_cursor_up(10).unwrap();
            term.move_cursor_left(11).unwrap();
        }
        thread::sleep(Duration::from_millis(100));
        step += 1;
    }
     step
}

fn flash(field: &mut [[u8;10];10],x : usize, y : usize) {
    field[y][x] = u8::MAX;
    let mut points = Vec::with_capacity(9);
    for (dx,dy) in (-1..=1).cartesian_product(-1..=1).filter(|p| p != &(0,0)) {
        if y as i32 + dy < 0 || x as i32 + dx < 0 || y as i32 + dy > 9 || x as i32 + dx > 9 { continue; }
        let y = (y as i32 + dy) as usize;
        let x = (x as i32 + dx) as usize;

        if field[y][x] == u8::MAX { continue; } 
        field[y][x] += 1;
        points.push((x,y));
        if field[y][x] >= 10 {
            flash(field, x, y);
        }
    }
    // println!("flashed {},{},{:?}",x,y,points)
}

pub fn run() {
    println!("day 11:");
    let data = parse(include_str!("input.txt"));
    println!("part 1: {}",part1(data));
    println!("part 2: {}",part2(data));
}

#[cfg(test)]
mod tests {
    const TEST_DATA : &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn parse() {
        assert_eq!([
            [5,4,8,3,1,4,3,2,2,3],
            [2,7,4,5,8,5,4,7,1,1],
            [5,2,6,4,5,5,6,1,7,3],
            [6,1,4,1,3,3,6,1,4,6],
            [6,3,5,7,3,8,5,4,7,8],
            [4,1,6,7,5,2,4,6,4,5],
            [2,1,7,6,8,4,1,7,2,1],
            [6,8,8,2,8,8,1,1,3,4],
            [4,8,4,6,8,4,8,5,5,4],
            [5,2,8,3,7,5,1,5,2,6]
        ],super::parse(TEST_DATA));
    }

    #[test]
    fn part1() {
        let field = super::parse(TEST_DATA);
        assert_eq!(1656,super::part1(field))
    }

    #[test]
    fn part2() {
        let field = super::parse(TEST_DATA);
        assert_eq!(195,super::part2(field))
    }
}
