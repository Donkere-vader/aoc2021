use std::fs;

fn get_puzzle_input() -> Vec<u64> {
    let contents = fs::read_to_string("../puzzle_input.txt").expect("Error reading file");

    let mut puzzle_input = Vec::new();

    for num in contents.split("\n") {
        puzzle_input.push(num.parse::<u64>().unwrap());
    }

    puzzle_input
}

fn main() {
    let puzzle_input = get_puzzle_input();

    let mut increased = 0;

    let mut prev_window_sum = puzzle_input[0] + puzzle_input[1] + puzzle_input[2];
    for idx in 3..puzzle_input.len() {
        let window_sum = puzzle_input[idx - 2] + puzzle_input[idx - 1] + puzzle_input[idx];
        if window_sum > prev_window_sum {
            increased += 1;
        }
        prev_window_sum = window_sum;
    }

    println!("Total increased: {}", increased);
}
