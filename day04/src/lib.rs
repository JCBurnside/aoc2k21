use std::{mem::MaybeUninit, str::FromStr};

fn transpose<T: Copy, const DX: usize, const DY: usize>(data: [[T; DX]; DY]) -> [[T; DY]; DX] {
    let mut output = [[MaybeUninit::<T>::uninit(); DY]; DX];
    for (y, row) in data.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            output[x][y].write(*t);
        }
    }
    output.map(|row| row.map(|it| unsafe { it.assume_init() }))
}

#[derive(Clone,Copy)]
struct Board {
    data: [[u8; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn mark(&mut self, space: u8) {
        if let Some((x, y)) = self.data.iter().enumerate().find_map(|(y, row)| {
            if let Some(x) = row.iter().position(|num| num == &space) {
                Some((x, y))
            } else {
                None
            }
        }) {
            self.marked[y][x] = true;
        }
    }

    fn new(data: [[u8; 5]; 5]) -> Self {
        Self {
            data,
            marked: [[false; 5]; 5],
        }
    }

    fn check_bingo(&self) -> bool {
        //check rows
        self.marked.iter().any(|row| row.iter().all(|it| *it)) ||
        //check cols
        transpose(self.marked).iter().any(|row| row.iter().all(|it|*it))

    }
}

fn parse(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let mut boards = vec![];
    let mut current_board = [[None; 5]; 5];
    let mut current_row = 0;
    lines.next();//skip first blank line
    for line in lines {
        if line.is_empty() {
            if current_row != 5 {
                panic!("Too few rows");
            }
            boards.push(Board::new(
                current_board.map(|row| row.map(|it| it.unwrap())),
            ));
            current_board = [[None; 5]; 5];
            current_row = 0;
        } else {
            for (x, num) in line
                .split(' ')
                .filter(|sub| !sub.is_empty())
                .map(<u8 as FromStr>::from_str)
                .map(Result::unwrap)
                .enumerate()
            {
                current_board[current_row][x] = Some(num);
            }
            current_row += 1;
        }
    }
    if current_row == 5 {
        boards.push(Board::new(
            current_board.map(|row| row.map(|it| it.unwrap())),
        ));
    } else if current_row != 0 {
        panic!("Incomplete board");
    }
    (
        first
            .split(',')
            .map(<u8 as FromStr>::from_str)
            .map(Result::unwrap)
            .collect(),
        boards,
    )
}

fn part1(calls: &[u8], mut boards: Vec<Board>) -> u32 {
    for call in calls {
        for board in boards.iter_mut() {
            board.mark(*call);
        }
        if let Some(winner) = boards.iter().find(|board| board.check_bingo()) {
            
            let sum_of_unmarked = winner
                .data
                .iter()
                .flatten()
                .zip(winner.marked.iter().flatten())
                .filter_map(|(num, mark)| if !mark { Some(num) } else { None })
                .fold(0u32, |lhs, rhs| lhs + (*rhs as u32));
            return sum_of_unmarked * (*call as u32)
        }
    }
    panic!("no winners?");
}

fn part2(calls: &[u8],mut boards : Vec<Board>) -> u32 {
    let mut final_winner= MaybeUninit::uninit();
    let mut final_call = 0;
    for call in calls {
        for board in boards.iter_mut() {
            board.mark(*call);
        }
        let mut to_remove = Vec::new();
        for (pos,winner) in boards.iter().enumerate().filter(|(_,board)| board.check_bingo()) {
            to_remove.push(pos);
            final_winner.write(winner.clone());
            final_call = call.clone();
        }

        for (idx,pos) in to_remove.iter().enumerate() {
            boards.remove(*pos - idx);
        }
    }
    let final_winner = unsafe { final_winner.assume_init() };
    let sum_of_unmarked = final_winner
                .data
                .iter()
                .flatten()
                .zip(final_winner.marked.iter().flatten())
                .filter_map(|(num, mark)| if !mark { Some(num) } else { None })
                .fold(0u32, |lhs, rhs| lhs + (*rhs as u32));
    sum_of_unmarked * (final_call as u32)
}

pub fn run() {
    println!("day 04:");
    let (calls,boards) = parse(include_str!("input.txt"));
    println!("part 1: {}", part1(&calls,boards.clone()));
    println!("part 2: {}", part2(&calls,boards));
}

#[cfg(test)]
mod tests {
    const TEST : &'static str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";
    #[test]
    fn part1() {
        let (calls,boards) = super::parse(TEST);
        assert_eq!(4512,super::part1(&calls,boards));
    }

    #[test]
    fn part2() {
        let (calls, boards) = super::parse(TEST);
        assert_eq!(1924,super::part2(&calls,boards));
    }
}
