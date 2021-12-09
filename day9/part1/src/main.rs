use std::fs;

type HeightMap = Vec<Vec<u8>>;

fn get_puzzle_input() -> HeightMap {
    let mut heightmap: HeightMap = Vec::new();

    let file_contents = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");

    for row in file_contents.split("\n") {
        let mut heightmap_row: Vec<u8> = Vec::new();
        for letter in row.chars() {
            heightmap_row.push(letter.to_digit(10).unwrap() as u8);
        }
        heightmap.push(heightmap_row);
    }

    heightmap
}

fn get_adjacent(map: &HeightMap, cord: (usize, usize)) -> Vec<u8> {
    let mut adjacent = Vec::new();

    for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let new_cord_concept: (i32, i32) = (cord.0 as i32 + offset.0 as i32, cord.1 as i32 + offset.1 as i32);
        if new_cord_concept.0 < 0 || new_cord_concept.0 >= map[0].len() as i32 || new_cord_concept.1 < 0 || new_cord_concept.1 >= map.len() as i32 { continue }
        let new_cord = (new_cord_concept.0 as usize, new_cord_concept.1 as usize);
        adjacent.push(map[new_cord.1][new_cord.0]);
    }

    adjacent
}

fn main() {
    let heightmap = get_puzzle_input();
    let mut lowest_values: Vec<usize> = Vec::new();

    for (y, row) in heightmap.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let mut low_value = true;
            let adjacent = get_adjacent(&heightmap, (x, y));
            for adjacent_value in adjacent.iter() {
                if value >= adjacent_value {
                    low_value = false;
                    break;
                }
            }
            if low_value {
                lowest_values.push(*value as usize);
            }
        }
    }

    println!("{:?} sum: {}", lowest_values, lowest_values.iter().sum::<usize>() + lowest_values.len());
}
