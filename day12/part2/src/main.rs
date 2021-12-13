use std::fs;
use std::collections::HashMap;
use std::cell::Cell;

type Map = HashMap<String, Vec<String>>;

fn get_puzzle_input() -> Map {
    let mut map: Map = HashMap::new();

    let file = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input.");

    for line in file.split("\n") {
        let splitted_line = line.split("-").collect::<Vec<&str>>();
        let from = String::from(splitted_line[0]);
        let to = String::from(splitted_line[1]);

        match map.get_mut(&from) {
            Some(entry) => entry.push(to.to_string()),
            None => { map.insert(from.to_string(), vec![to.to_string()]); },
        }

        match map.get_mut(&to) {
            Some(entry) => entry.push(from),
            None => { map.insert(to, vec![from]); },
        }
    }

    map
}


fn get_paths(map: &Map, node: &String) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let mut path = vec![String::from("start")];
    // let mut visited_small_cave_twice = Cell::new(false);
    traverse(map, node, &mut path, &mut paths, false);

    paths
}

fn traverse(map: &Map, node: &String, path: &mut Vec<String>, paths: &mut Vec<Vec<String>>, visited_small_cave_twice: bool) {
    let entry = map.get(node).unwrap();

    if *node == String::from("end") {
        paths.push(path.to_vec());
    } else {
        for cave in entry.iter() {
            let mut going_to_visit_small_cave_twice = false;
            if cave.to_lowercase() == *cave && path.contains(cave) {
                if visited_small_cave_twice || *cave == String::from("start") {
                    continue
                } else {
                    going_to_visit_small_cave_twice = true;
                }
            }
            path.push(cave.to_string());
            traverse(map, cave, path, paths, going_to_visit_small_cave_twice || visited_small_cave_twice);
            path.remove(path.len() - 1);
        }
    }
}


fn main() {
    let puzzle_input = get_puzzle_input();
    let paths = get_paths(&puzzle_input, &String::from("start"));
    println!("\nNumber of paths: {}", paths.len());

    // for path in paths.iter() {
    //     for node in path.iter() {
    //         print!("{},", node);
    //     }
    //     println!();
    // }
}
