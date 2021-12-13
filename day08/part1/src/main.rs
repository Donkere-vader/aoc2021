use std::fs;

fn get_puzzle_input() -> Vec<Vec<Vec<char>>> {
    let mut output_values = Vec::new();

    let file_contents = fs::read_to_string("../puzzle_input.txt").expect("Error reading puzzle input");
    for row in file_contents.split("\n") {
        let mut row_vec = Vec::new();
        let output_values_string = row.split(" | ").collect::<Vec<&str>>()[1];

        for number in output_values_string.split(" ") {
            row_vec.push(number.chars().collect());
        }
        output_values.push(row_vec);
    }
    
    output_values
}

fn main() {
    let puzzle_input = get_puzzle_input();
    let mut occurances = 0;
    
    for row in puzzle_input.iter() {
        for output_value in row.iter() {
            if [2, 4, 3, 7].contains(&output_value.len()) {
                occurances += 1;
            }
        }
    }

    println!("{}", occurances);
}
