use std::fs;
use std::collections::HashMap;

type Board = [[i64; 5]; 5];
type NumbersMap = HashMap<i64, Vec<(usize, usize, usize)>>;

fn get_puzzle_input() -> (Vec<i64>, Vec<Board>, NumbersMap) {
    let mut numbers = Vec::new();
    let mut boards = Vec::new();
    let mut numbers_mapped: NumbersMap = HashMap::new();

    let file_as_string = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");
    let file_contents: Vec<&str> = file_as_string.split("\n").collect();

    for num in file_contents[0].split(",") {
        numbers.push(num.parse::<i64>().expect("Failed to parse input"));
    }

    let n_boards = (file_contents.len() - 1) / 6;

    for board_idx in 0..n_boards {
        let mut new_board: Board = Default::default();

        for y in 0..5 {
            let file_row = file_contents[y + (2 + (board_idx * 6))];
            let mut x = 0;
            for str_num in file_row.split(" ") {
                if str_num == "" { continue }
                let num = str_num.parse::<i64>().unwrap();
                new_board[y][x] = num;

                numbers_mapped.entry(num).or_insert(Vec::new()).push((board_idx, x, y));

                x += 1;
            }
        }
        boards.push(new_board);
    }

    (numbers, boards, numbers_mapped)
}

fn main() {
    let (numbers, mut boards, numbers_mapped) = get_puzzle_input();
    
    let mut last_num = 0;
    let mut winning_board_idx = 0;
    for num in numbers.iter() {
        // apply number
        for num_locations in numbers_mapped.get(num) {
            for num_loc in num_locations {
                boards[num_loc.0][num_loc.2][num_loc.1] = -1;
            }
        }

        // check if won
        let mut won = false;
        for (idx, board) in boards.iter().enumerate() {
            for row in board {
                if row.iter().sum::<i64>() == -5 {
                    won = true;
                    winning_board_idx = idx;
                    break;
                }
            }

            if won { break }

            for x in 0..5 {
                if board[0][x] + board[1][x] + board[2][x] + board[3][x] + board[4][x] == -5 {
                    won = true;
                    winning_board_idx = idx;
                    break;
                }
            }

            if won { break }
        }
        if won { last_num = *num; break }
    }

    for board in boards.iter() {
        for row in board.iter() {
            println!("{:?}", row);
        }
        println!("");
    }

    let mut total = 0;
    for row in boards[winning_board_idx].iter() {
        for num in row.iter() {
            if *num > 0 {
                total += num;
            }
        }
    }

    println!("{} * {} = {}", total, last_num, total * last_num);
}
