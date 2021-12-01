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

    for idx in 1..puzzle_input.len() {
        let prev_item = puzzle_input[idx - 1];
        let item = puzzle_input[idx];

        if item > prev_item {
            increased += 1;
        }
    }

    println!("Total increased: {}", increased);
}
