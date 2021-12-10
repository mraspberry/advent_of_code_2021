extern crate log;

use log::debug;
use std::str::Lines;

#[derive(Debug, Clone, Copy)]
struct BingoNumber {
    value: u32,
    marked: bool,
}

impl BingoNumber {
    fn new(val: u32) -> BingoNumber {
        BingoNumber {
            value: val,
            marked: false,
        }
    }
    fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Debug)]
struct BingoBoard {
    board: [[BingoNumber; 5]; 5],
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            board: [[BingoNumber::new(0); 5]; 5],
        }
    }
    fn set_pos(&mut self, rownum: usize, v_pos: usize, val: BingoNumber) {
        self.board[rownum][v_pos] = val;
    }
}

fn get_called_nums(called: &str) -> Vec::<u32> {
    let mut nums: Vec<u32> = Vec::new();
    for numstr in called.split(',').into_iter() {
        debug!("numstr: {}", numstr);
        match numstr.trim().parse::<u32>() {
            Ok(ok) => nums.push(ok),
            Err(_) => panic!("Non-numeric item in called numbers")
        };
    }
    return nums
}

fn populate_boards(lines: &mut Lines) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut current_board = 0;
    let mut rownum: usize = 0;

    for line in lines {
        dbg!(&line);
        if line.trim().is_empty() {
            boards.push(BingoBoard::new());
            current_board = boards.len() - 1;
            rownum = 0;
            continue;
        }
        for (i, numstr) in line.split_whitespace().enumerate() {
            boards[current_board].set_pos(rownum, i, BingoNumber::new(numstr.parse::<u32>().unwrap()));
        }
        rownum += 1;
    }
    dbg!(&boards);
    return boards
}

fn solve(input: &str) -> u32 {
    let mut lines = input.lines();
    let called = get_called_nums(lines.next().unwrap());
    return 0u32
}
fn main() {
}

#[test]
fn test_get_called_nums() {
    let ans: Vec<u32> = vec![1u32, 2u32, 3u32, 4u32, 5u32];
    let nums = "1,2,3,4,5";
    assert_eq!(get_called_nums(nums), ans)
}

#[test]
fn test_populate_boards() {
    let input = "22 13 17 11  0\n\
 8  2 23  4 24\n\
21  9 14 16  7\n\
 6 10  3 18  5\n\
 1 12 20 15 19\n\n\
 3 15  0  2 22\n\
 9 18 13 17  5\n\
19  8  7 25 23\n\
20 11 10 24  4\n\
14 21 16 12  6\n
";
    let ans: Vec<BingoBoard> = vec![
        BingoBoard { board: [
            [BingoNumber::new(22), BingoNumber::new(13), BingoNumber::new(17), BingoNumber::new(11), BingoNumber::new(0)],
            [BingoNumber::new(8), BingoNumber::new(2), BingoNumber::new(23), BingoNumber::new(4), BingoNumber::new(24)],
            [BingoNumber::new(21), BingoNumber::new(9), BingoNumber::new(14), BingoNumber::new(16), BingoNumber::new(7)],
            [BingoNumber::new(6), BingoNumber::new(10), BingoNumber::new(3), BingoNumber::new(18), BingoNumber::new(5)],
            [BingoNumber::new(1), BingoNumber::new(12), BingoNumber::new(20), BingoNumber::new(15), BingoNumber::new(19)]
        ]}, BingoBoard {board: [
            [BingoNumber::new(3), BingoNumber::new(15), BingoNumber::new(0), BingoNumber::new(2), BingoNumber::new(22)],
            [BingoNumber::new(9), BingoNumber::new(18), BingoNumber::new(13), BingoNumber::new(17), BingoNumber::new(5)],
            [BingoNumber::new(19), BingoNumber::new(8), BingoNumber::new(7), BingoNumber::new(25), BingoNumber::new(23)],
            [BingoNumber::new(20), BingoNumber::new(11), BingoNumber::new(10), BingoNumber::new(24), BingoNumber::new(4)],
            [BingoNumber::new(14), BingoNumber::new(21), BingoNumber::new(16), BingoNumber::new(12), BingoNumber::new(6)]
        ]}];
    let check = populate_boards(&mut input.lines());
    for b in 0..1 {
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(ans[b].board[i][j].value, check[b].board[i][j].value);
            }
        }
    }
}

#[test]
fn test_example() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n\
22 13 17 11  0\n\
 8  2 23  4 24\n\
21  9 14 16  7\n\
 6 10  3 18  5\n\
 1 12 20 15 19\n\n\
 3 15  0  2 22\n\
 9 18 13 17  5\n\
19  8  7 25 23\n\
20 11 10 24  4\n\
14 21 16 12  6\n\n\
14 21 17 24  4\n\
10 16 15  9 19\n\
18  8 23 26 20\n\
22 11 13  6  5\n\
 2  0 12  3  7\n";
    let ans: u32 = 4512;
    assert_eq!(ans, solve(input));
}
