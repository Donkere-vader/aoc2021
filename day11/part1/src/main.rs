use std::fs;
use termion::{color, style};

const STEPS: usize = 100;

fn get_puzzle_input() -> Vec<Vec<u8>> {
    let mut puzzle_input = Vec::new();

    let file_contents = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");

    for line in file_contents.split("\n") {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }

        puzzle_input.push(row);
    }

    puzzle_input
}

fn get_adjacent(map: &Vec<Vec<u8>>, cord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();

    for offset in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (-1, -1), (1, -1)] {
        let new_cord_concept: (i32, i32) = (cord.0 as i32 + offset.0 as i32, cord.1 as i32 + offset.1 as i32);
        if new_cord_concept.0 < 0 || new_cord_concept.0 >= map[0].len() as i32 || new_cord_concept.1 < 0 || new_cord_concept.1 >= map.len() as i32 { continue }
        let new_cord = (new_cord_concept.0 as usize, new_cord_concept.1 as usize);
        adjacent.push(new_cord);
    }

    adjacent
}

fn main() {
    let mut puzzle_input = get_puzzle_input();
    let mut flashes = 0;

    println!("Before any steps:");
    for row in puzzle_input.iter() {
        for num in row.iter() {
            print!("{}", num);
        }
        println!();
    }
    println!("\n");
    
    for step in 0..STEPS {
        let mut to_flash: Vec<(usize, usize)> = Vec::new();
        let mut flashed: Vec<(usize, usize)> = Vec::new();

        for y in 0..puzzle_input.len() {
            for x in 0..puzzle_input[0].len() {
                puzzle_input[y][x] += 1;
                if puzzle_input[y][x] > 9 {
                    to_flash.push((x, y));
                }
            }
        }

        while to_flash.len() != 0 {
            let coord = to_flash.remove(0);
            if flashed.contains(&coord) { continue }
            puzzle_input[coord.1][coord.0] = 0;
            flashes += 1;
    
            let adjacent = get_adjacent(&puzzle_input, coord);
            for c in adjacent.iter() {
                if flashed.contains(&c) { continue }
                puzzle_input[c.1][c.0] += 1;
                if puzzle_input[c.1][c.0] > 9 {
                    to_flash.push(*c);
                }
            }
            flashed.push(coord);
        }

        println!("After step {}", step + 1);
        for (y, row) in puzzle_input.iter().enumerate() {
            for (x, num) in row.iter().enumerate() {
                if flashed.contains(&(x, y)) {
                    print!("{}{}{}{}", color::Bg(color::Red), style::Bold, num, style::Reset);
                } else {
                    print!("{}", num);
                }
            }
            println!();
        }
        println!("\n");
    }

    println!("Flashed octopusses: {}", flashes);
}
