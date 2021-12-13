use std::fs;

type Map = Vec<Vec<u8>>;
type Instruction = (char, usize);

fn get_puzzle_input() -> (Map, Vec<Instruction>) {
    let mut instructions = Vec::new();

    let file_contents = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");
    let x: Vec<&str> = file_contents.split("\n\n").collect::<Vec<&str>>();
    let map_cords = x[0];
    let instructions_str = x[1];

    let mut cords = Vec::new();
    let mut paper_dimensions = [0; 2];
    for line in map_cords.split("\n") {
        let string_cord = line.split(",").collect::<Vec<&str>>();
        let cord = (string_cord[0].parse::<usize>().unwrap(), string_cord[1].parse::<usize>().unwrap());
        cords.push(cord);
        if cord.0 + 1 > paper_dimensions[0] {
            paper_dimensions[0] = cord.0 + 1;
        }
        if cord.1 + 1 > paper_dimensions[1] {
            paper_dimensions[1] = cord.1 + 1;
        }
    }

    let mut paper: Map = vec![vec![0; paper_dimensions[0]]; paper_dimensions[1]];
    for cord in cords {
        paper[cord.1][cord.0] = 1;
    }

    for line in instructions_str.split("\n") {
        let instruc_str = line.split(" ").collect::<Vec<&str>>()[2].split("=").collect::<Vec<&str>>();
        instructions.push((instruc_str[0].chars().collect::<Vec<char>>()[0], instruc_str[1].parse::<usize>().unwrap()));
    }

    (paper, instructions)
}

fn fold(map: &mut Map, instruction: &Instruction) {
    if instruction.0 == 'y' {
        let mut top = Vec::new();
        let mut bottom = Vec::new();
        for y in 0..map.len() {
            if y < instruction.1 {
                top.push(map[y].to_vec());
            } else if y > instruction.1 {
                bottom.push(map[y].to_vec());
            }
        }

        for delta_y in 0..bottom.len() {
            for (idx, item) in bottom[delta_y].iter().enumerate() {
                if *item == 1 {
                    map[instruction.1 - delta_y - 1][idx] = 1;
                }
            };
        }
        while map.len() > instruction.1 {
            map.remove(map.len() - 1);
        }
    } else if instruction.0 == 'x' {
        let mut left = vec![Vec::new(); map.len()];
        let mut right = vec![Vec::new(); map.len()];
        for x in 0..map[0].len() {
            if x < instruction.1 {
                for y in 0..map.len() { left[y].push(map[y][x]) }
            } else if x > instruction.1 {
                for y in 0..map.len() { right[y].push(map[y][x]) }
            }
        }

        for delta_x in 0..right[0].len() {
            for y in 0..right.len() {
                if right[y][delta_x] == 1 {
                    map[y][instruction.1 - delta_x - 1] = 1;
                }
            }
        }

        while map[0].len() > instruction.1 {
            for y in 0..map.len() {
                let row_width = map[y].len();
                map[y].remove(row_width - 1);
            }
        }
    } else { panic!("Invalid fold instruction: {:?}", instruction); }
}

fn show_map(map: &Map) {
    for row in map.iter() {
        for item in row.iter() {
            if *item == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let (mut map, instructions) = get_puzzle_input();

    // show_map(&map);

    for instruction in instructions.iter() {
        println!("{:?}", instruction);
        fold(&mut map, instruction);
        // show_map(&map);
        // println!();
    }

    let mut dots: usize = 0;
    for row in map.iter() {
        for item in row.iter() {
            dots += *item as usize;
        }
    }

    println!("Dots: {}", dots);
    show_map(&map);
}
