use std::fs;
use std::collections::HashMap;

fn get_puzzle_input() -> Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> {
    let mut puzzle_input = Vec::new();

    let file_contents = fs::read_to_string("../puzzle_input.txt").expect("Error reading puzzle input");
    for row in file_contents.split("\n") {
        let mut callibration_row_vec = Vec::new();
        let mut output_row_vec = Vec::new();
        let calibrations_values_string = row.split(" | ").collect::<Vec<&str>>()[0];
        let output_values_string = row.split(" | ").collect::<Vec<&str>>()[1];

        for number in output_values_string.split(" ") {
            output_row_vec.push(number.chars().collect());
        }
        for number in calibrations_values_string.split(" ") {
            callibration_row_vec.push(number.chars().collect());
        }
        puzzle_input.push((callibration_row_vec, output_row_vec));
    }

    puzzle_input
}

//  000
// 1   2
// 1   2
// 1   2
//  333
// 4   5
// 4   5
// 4   5
//  666

fn anti_vector(source_vec: &Vec<usize>) -> Vec<usize> {
    let mut anti_vec = Vec::new();

    for num in 0..7 {
        if !source_vec.contains(&num) {
            anti_vec.push(num);
        }
    }

    anti_vec
}

fn map_digits(segment_codes: &Vec<Vec<char>>) -> HashMap<Vec<char>, usize> {
    let mut digits_map: HashMap<Vec<char>, usize> = HashMap::new();
    let mut segment_letters: [Vec<char>; 7] = Default::default();
    for idx in 0..7 {
        segment_letters[idx] = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    }
    let mut segment_length_5 = Vec::new();
    let mut segment_length_6 = Vec::new();
    let mut one = Vec::new();
    let mut four = Vec::new();

    for sc in segment_codes.iter() {
        let mut num_segments: Vec<usize> = (0..7).collect::<Vec<usize>>();
        if sc.len() == 2 {
            num_segments = vec![2, 5];
            digits_map.insert(sc.to_vec(), 1);
            one = sc.to_vec();
        }
        else if sc.len() == 4 {
            num_segments = vec![2, 5, 1, 4];
            digits_map.insert(sc.to_vec(), 4);
            four = sc.to_vec();
        }
        else if sc.len() == 3 { num_segments = vec![2, 5, 0]; digits_map.insert(sc.to_vec(), 7); }
        else if sc.len() == 7 { num_segments = vec![0, 1, 2, 3, 4, 5, 6, 7]; digits_map.insert(sc.to_vec(), 8); }
        else if sc.len() == 5 { segment_length_5.push(sc.to_vec()); }
        else if sc.len() == 6 { segment_length_6.push(sc.to_vec()); }

        for idx in anti_vector(&num_segments) {
            for letter in sc.iter() {
                if segment_letters[idx].contains(letter) {
                    segment_letters[idx].retain(|x| x != letter);
                }
            }
        }
    }

    for (idx, sc) in segment_length_5.iter().enumerate() {
        let mut is_three = true;
        for letter in one.iter() {
            if !sc.contains(letter) {
                is_three = false;
                break;
            }
        }
        if is_three {
            digits_map.insert(sc.to_vec(), 3);
            segment_length_5.remove(idx);
            break;
        }
    }

    for (idx, sc) in segment_length_5.iter().enumerate() {
        let mut is_five = true;
        for letter in four.iter() {
            if !one.contains(letter) && !sc.contains(letter) {
                is_five = false;
                break;
            }
        }
        if is_five {
            digits_map.insert(sc.to_vec(), 5);
            segment_length_5.remove(idx);
            break;
        }
    }

    digits_map.insert(segment_length_5[0].to_vec(), 2);

    for (idx, sc) in segment_length_6.iter().enumerate() {
        let mut is_six = false;
        for letter in one.iter() {
            if !sc.contains(letter) {
                is_six = true;
                break
            }
        }
        if is_six {
            digits_map.insert(sc.to_vec(), 6);
            segment_length_6.remove(idx);
            break;
        }
    }

    for (idx, sc) in segment_length_6.iter().enumerate() {
        let mut is_nine = true;
        for letter in four.iter() {
            if !sc.contains(letter) {
                is_nine = false;
                break;
            }
        }
        if is_nine {
            digits_map.insert(sc.to_vec(), 9);
            segment_length_6.remove(idx);
            break;
        }
    }

    digits_map.insert(segment_length_6[0].to_vec(), 0);

    digits_map
}

fn get_value_out_of_map(map: &HashMap<Vec<char>, usize>, key: &Vec<char>) -> usize {
    for map_key in map.keys() {
        if map_key.len() != key.len() { continue; }
        let mut found_it = true;
        for letter in map_key.iter() {
            if !key.contains(letter) {
                found_it = false;
                break;
            }
        }
        if found_it {
            return *map.get(map_key).unwrap();
        }
    }

    panic!("Couldnt find it");
}

fn main() {
    let puzzle_input = get_puzzle_input();

    let mut values: Vec<usize> = Vec::new();

    for row in puzzle_input.iter() {
        let map = map_digits(&row.0);
        let mut value = 0;
        for (idx, segment) in row.1.iter().rev().enumerate() {
            value += get_value_out_of_map(&map, segment) * (10usize.pow(idx as u32))
        }
        values.push(value);
    }

    let mut total = 0;
    for v in values.iter() {
        total += v;
    }

    println!("{}", total);
}
