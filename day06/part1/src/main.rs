use std::fs;

const DAYS: usize = 20;

fn get_puzzle_input() -> Vec<usize> {
    let mut puzzle_input = Vec::new();

    let file_contents = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");

    for num in file_contents.split(",") {
        puzzle_input.push(num.parse::<usize>().unwrap());
    }

    puzzle_input
}

fn main() {
    let mut puzzle_input = get_puzzle_input();
    
    println!("Initial state: {:?}", puzzle_input);

    for day in 0..DAYS {
        let mut n_of_new_fish = 0;
        for fish_idx in 0..puzzle_input.len() {
            if puzzle_input[fish_idx] == 0 {
                puzzle_input[fish_idx] = 6;
                n_of_new_fish += 1;
            } else {
                puzzle_input[fish_idx] -= 1;
            }
        }

        for _ in 0..n_of_new_fish {
            puzzle_input.push(8);
        }
        println!("After {: >3} day(s): {:?}", day, puzzle_input);
    }

    println!("{}", puzzle_input.len());
}
