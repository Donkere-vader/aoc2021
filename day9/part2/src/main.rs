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

fn get_adjacent(map: &HeightMap, cord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();

    for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let new_cord_concept: (i32, i32) = (cord.0 as i32 + offset.0 as i32, cord.1 as i32 + offset.1 as i32);
        if new_cord_concept.0 < 0 || new_cord_concept.0 >= map[0].len() as i32 || new_cord_concept.1 < 0 || new_cord_concept.1 >= map.len() as i32 { continue }
        let new_cord = (new_cord_concept.0 as usize, new_cord_concept.1 as usize);
        adjacent.push(new_cord);
    }

    adjacent
}

fn get_lowest_values(heightmap: &HeightMap) -> Vec<(usize, usize)> {
    let mut lowest_values: Vec<(usize, usize)> = Vec::new();

    for (y, row) in heightmap.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let mut low_value = true;
            let adjacent = get_adjacent(heightmap, (x, y));
            for adjacent_value in adjacent.iter() {
                if *value >= heightmap[adjacent_value.1][adjacent_value.0] {
                    low_value = false;
                    break;
                }
            }
            if low_value {
                lowest_values.push((x, y));
            }
        }
    }

    lowest_values
}

fn main() {
    let heightmap = get_puzzle_input();
    let lowest_values = get_lowest_values(&heightmap);
    let mut basins = Vec::new();

    for lowest_square in lowest_values.iter() {
        let mut found_neighbours: Vec<(usize, usize)> = vec![];
        let mut squares_to_check: Vec<(usize, usize)> = vec![*lowest_square];
        while squares_to_check.len() > 0 {
            let square = squares_to_check.remove(0);
            if heightmap[square.1][square.0] == 9 { continue } else {
                if !found_neighbours.contains(&square) {
                    found_neighbours.push(square);
                }
            }
            let neighbours = get_adjacent(&heightmap, square);
            for n in neighbours.iter() {
                if !found_neighbours.contains(n) {
                    squares_to_check.push(*n);
                }
            }
        }
        basins.push(found_neighbours);
    }

    let mut basin_sizes = Vec::new();
    for basin in basins.iter() {
        basin_sizes.push(basin.len());
    }
    basin_sizes.sort();
    basin_sizes.reverse();

    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}
