use std::fs;

fn get_puzzle_input() -> Vec<usize> {
    let mut puzzle_input = Vec::new();

    let file_contents = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");

    for num_str in file_contents.split(",") {
        puzzle_input.push(num_str.parse().unwrap());
    }

    puzzle_input
}

fn main() {
    let puzzle_input = get_puzzle_input();

    let max_position = puzzle_input.iter().max().unwrap();
    let min_position = puzzle_input.iter().min().unwrap();
    let mut fuel_spent = Vec::new();
    for _ in 0..(max_position - min_position) { fuel_spent.push(0usize) }

    for crab_position in puzzle_input.iter() {
        for idx in 0..fuel_spent.len() {
            let fuel_to_idx = (*crab_position as i64 - idx as i64).abs() as usize;
            fuel_spent[idx] += fuel_to_idx;
        }
    }

    for (idx, value) in fuel_spent.iter().enumerate() {
        println!("{}: {}", idx, value);
    }

    let optimal_consumption = fuel_spent.iter().min().unwrap();
    let optimal_position = fuel_spent.iter().position(|&c| c == *optimal_consumption).unwrap();

    println!("Optimal position: {}\nFuel spent: {}", optimal_position, optimal_consumption);
}
