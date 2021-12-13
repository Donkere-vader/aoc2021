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

fn main() {
    let heightmap = get_puzzle_input();

    let mut basin_rows: Vec<Vec<(usize, usize)>> = Vec::new();

    for (y, row) in heightmap.iter().enumerate() {
        let mut new_row = Vec::new();
        for (x, value) in row.iter().enumerate() {
            if *value == 9 {
                if new_row.len() > 0 {
                    basin_rows.push(new_row.to_vec());
                    new_row = Vec::new();
                }
            } else {
                new_row.push((x, y));
            }
        }
        if new_row.len() > 0 {
            basin_rows.push(new_row.to_vec());
        }
    }

    let mut basins: Vec<Vec<(usize, usize)>> = Vec::new();

    while basin_rows.len() > 0 {
        let mut new_basin = basin_rows.remove(0);
        let mut ys = Vec::new();

        let mut x = 0;
        while x < basin_rows.len() {
            for cord in new_basin.iter() {
                if !ys.contains(&cord.1) {
                    ys.push(cord.1);
                }
            }
            let neighbor_y_i32 = basin_rows[x][0].1 as i32 - 1;
            // println!("{: <2} {:?}\t{:?} {} {}", x, new_basin, basin_rows[x], neighbor_y_i32, ys.contains(&(neighbor_y_i32 as usize)));
            if !(neighbor_y_i32 >= 0 && ys.contains(&(neighbor_y_i32 as usize))) { x += 1; continue };
            let neighbor_y = neighbor_y_i32 as usize;
            let mut touch = false;
            for cord in basin_rows[x].iter() {
                for c in new_basin.iter() {
                    if c.1 != neighbor_y { continue }
                    if c.0 == cord.0 {
                        // println!("\t{:?} {:?}", c, cord);
                        touch = true;
                        break
                    }
                }
                if touch { break };
            }

            // println!("\t{}", touch);

            if touch {
                let mut new_addition_to_basin = basin_rows.remove(x);
                new_basin.append(&mut new_addition_to_basin);
            } else {
                x += 1;
            }
        }
        basins.push(new_basin);
    }

    println!("LEN  BASIN");
    for row in basins.iter() {
        let mut r = Vec::new();
        for value in row.iter() {
            r.push(heightmap[value.1][value.0]);
        }
        println!("{: <4} {:?}", format!("{}:", r.len()), r);
    }

    let mut basin_sizes = Vec::new();
    for b in basins.iter() {
        basin_sizes.push(b.len());
    }
    basin_sizes.sort();
    basin_sizes.reverse();

    println!("{} * {} * {} = {}", basin_sizes[0], basin_sizes[1], basin_sizes[2], basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}
