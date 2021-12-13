use std::fs;

const BYTE_SIZE: usize = 12;

fn get_puzzle_input() -> Vec<[u8; BYTE_SIZE]> {
    let mut bytes = Vec::new();

    let content = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");

    for line in content.split("\n") {
        let mut byte: [u8; BYTE_SIZE] = Default::default();
        for (idx, letter) in line.chars().enumerate() {
            byte[idx] = letter.to_digit(10).unwrap() as u8;
        }
        bytes.push(byte);
    }

    bytes
}

fn byte_to_usize(byte: &[u8; BYTE_SIZE]) -> usize {
    let mut num: usize = 0;

    let mut bit_value: usize = 1;

    for bit in byte.iter().rev() {
        num += *bit as usize * bit_value;
        bit_value *= 2;
    }

    num
}

fn start_filtering(puzzle_input: &Vec<[u8; BYTE_SIZE]>, most_common: bool) -> [u8; BYTE_SIZE] {
    let mut all_bytes = puzzle_input.to_vec();

    for idx in 0..BYTE_SIZE {
        let mut indexes_of_values: [Vec<usize>; 2] = Default::default();

        for (byte_idx, byte) in all_bytes.iter().enumerate() {
            indexes_of_values[byte[idx] as usize].push(byte_idx);
        }

        let most_common_bit = if indexes_of_values[0].len() > indexes_of_values[1].len() { 0 } else { 1 };
        let to_remove_bit = if most_common { if most_common_bit == 0 { 1 } else { 0 }} else { most_common_bit };

        for (idx, idx_to_remove) in indexes_of_values[to_remove_bit].iter().enumerate() {
            all_bytes.remove(idx_to_remove - idx);
        }

        if all_bytes.len() == 1 {
            return all_bytes[0];
        }
    }

    panic!("No value found");
}

fn main() {
    let puzzle_input = get_puzzle_input();

    let oxygen_generator_rating_byte = start_filtering(&puzzle_input, true);
    let co2_scrubber_ratin_byte = start_filtering(&puzzle_input, false);

    println!("{:?} {:?}", oxygen_generator_rating_byte, co2_scrubber_ratin_byte);

    let oxygen_generator_rating = byte_to_usize(&oxygen_generator_rating_byte);
    let co2_scrubber_rating_byte = byte_to_usize(&co2_scrubber_ratin_byte);

    println!("{} * {} = {}", oxygen_generator_rating, co2_scrubber_rating_byte, oxygen_generator_rating * co2_scrubber_rating_byte);
}