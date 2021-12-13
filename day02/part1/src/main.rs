use std::fs;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

fn get_puzzle_input() -> Vec<(Direction, i64)> {
    let mut puzzle_input = Vec::new();

    let contents = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");

    for line in contents.split("\n") {
        let splitted_line = line.split(" ").collect::<Vec<&str>>();
        let direction = match &splitted_line[0] {
            &"forward" => Direction::Forward,
            &"up" => Direction::Up,
            &"down" => Direction::Down,
            &&_ => { panic!("Invalid input") }
        };
        let distance = splitted_line[1].parse::<i64>().unwrap();
        puzzle_input.push((direction, distance));
    }

    puzzle_input
}

fn main() {
    let puzzle_input = get_puzzle_input();
    let mut x: i64 = 0;
    let mut z: i64 = 0;

    for instruction in puzzle_input {
        match instruction.0 {
            Direction::Up => { z -= instruction.1 },
            Direction::Down => { z += instruction.1 },
            Direction::Forward => { x += instruction.1 },
        }
    }

    println!("{} * {} = {}", x, z, x * z);
}
