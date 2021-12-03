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

fn main() {
    let puzzle_input = get_puzzle_input();
    let mut occurances: [[u32; 2]; BYTE_SIZE] = Default::default(); 
    
    for byte in puzzle_input.iter() {
        for (idx, bit) in byte.iter().enumerate() {
            occurances[idx][*bit as usize] += 1;
        }
    }

    let mut gamma_number_byte: [u8; BYTE_SIZE] = Default::default();
    let mut epsilon_number_byte: [u8; BYTE_SIZE] = Default::default();

    for (idx, bit) in occurances.iter().enumerate() {
        gamma_number_byte[idx] = if bit[0] > bit[1] { 0 } else { 1 }
    }

    for (idx, bit) in gamma_number_byte.iter().enumerate() {
        epsilon_number_byte[idx] = if bit == &0 { 1 } else { 0 }
    }


    let gamma_number = byte_to_usize(&gamma_number_byte);
    let epsilon_number = byte_to_usize(&epsilon_number_byte);

    println!("{} * {} = {}", gamma_number, epsilon_number, gamma_number * epsilon_number);
}
