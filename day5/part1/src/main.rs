use std::fs;

type Coordinate = [usize; 2];
type VentPipe = [Coordinate; 2];

fn get_puzzle_input() -> Vec<VentPipe> {
    let mut puzzle_input = Vec::new();

    let file_contents = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");

    for line in file_contents.split("\n") {
        let coords: Vec<&str> = line.split(" -> ").collect();
        let left_coord_string: Vec<&str> = coords[0].split(",").collect();
        let right_coord_string: Vec<&str> = coords[1].split(",").collect();

        puzzle_input.push([
            [left_coord_string[0].parse::<usize>().unwrap(), left_coord_string[1].parse::<usize>().unwrap()],
            [right_coord_string[0].parse::<usize>().unwrap(), right_coord_string[1].parse::<usize>().unwrap()]
        ]);
    }

    puzzle_input
}

fn get_between_coords(pipe: &VentPipe) -> Vec<Coordinate> {
    let mut coords = Vec::new();
    let mut working_cord = *&pipe[1];
    let mut target_cord = *&pipe[0];
    if pipe[1][0] > pipe[0][0] || pipe[1][1] > pipe[0][1] {
        working_cord = *&pipe[0];
        target_cord = *&pipe[1];
    }
    let offset = [if pipe[1][0] != pipe[0][0] { 1 } else { 0 }, if pipe[1][1] != pipe[0][1] { 1 } else { 0 }];

    coords.push(working_cord);

    while working_cord != target_cord {
        working_cord[0] += offset[0];
        working_cord[1] += offset[1];
        coords.push(*&working_cord);
    }

    coords
}

fn main() {
    let puzzle_input = get_puzzle_input();
    let mut occupied_coords: Vec<Coordinate> = Vec::new();
    let mut dubble_pipes = 0;

    for pipe in puzzle_input {
        if pipe[0][0] == pipe[1][0] || pipe[0][1] == pipe[1][1] {
            let pipe_covers = get_between_coords(&pipe);

            for cord in pipe_covers {
                if occupied_coords.iter().filter(|&n| *n == cord).count() == 1 {
                    dubble_pipes += 1;
                }
                occupied_coords.push(cord);
            }
        }
    }

    // let mut matrix_width = 0;
    // let mut matrix_height = 0;

    // for cord in occupied_coords.iter() {
    //     if cord[0] > matrix_width {
    //         matrix_width = cord[0] + 1;
    //     }
    //     if cord[1] > matrix_height {
    //         matrix_height = cord[1] + 1;
    //     }
    // }

    // let mut matrix: Vec<Vec<usize>> = Vec::new();
    // for _ in 0..matrix_height {
    //     let mut row = Vec::new();
    //     for _ in 0..matrix_width {
    //         row.push(0);
    //     }
    //     matrix.push(row);
    // }

    // for cord in occupied_coords.iter() {
    //     matrix[cord[1]][cord[0]] += 1;
    // }

    // for row in matrix.iter() {
    //     for item in row.iter() {
    //         if *item == 0 {
    //             print!(".");
    //         } else {
    //             print!("{}", item);
    //         }
    //     }
    //     println!();
    // }

    println!("{}", dubble_pipes);
    // println!("{:?}", get_between_coords(&[[0, 0], [0, 8]]));
}
