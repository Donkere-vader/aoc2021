use std::fs;

const CHUNCK_OPENERS: [char; 4] = ['(', '[', '<', '{'];
const CHUNCK_CLOSERS: [char; 4] = [')', ']', '>', '}'];

fn get_puzzle_input() -> Vec<String> {
    let mut puzzle_input = Vec::new();

    let file_contents = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");

    for row in file_contents.split("\n") {
        puzzle_input.push(row.to_string());
    }

    puzzle_input
}

fn check_line(line: &String) -> (bool, (char, char)) {
    let mut characters_stack = Vec::new();

    for c in line.chars() {
        // println!("\t{} {:?}", c, characters_stack);
        if CHUNCK_OPENERS.contains(&c) {
            let expected_char = match c {
                '(' => ')',
                '{' => '}',
                '<' => '>',
                '[' => ']', 
                _ => panic!("Invalid char {}", c),
            };
            characters_stack.push(expected_char);
        } else if CHUNCK_CLOSERS.contains(&c) {
            let expected_char = characters_stack.remove(characters_stack.len() - 1);
            if c != expected_char {
                return (false, (expected_char, c));
            }
        } else {
            panic!("Invalid char: {}", c);
        }
    }

    // if characters_stack.len() > 0 {
    //     return (false, (characters_stack[0], ' '));
    // }

    (true, ('a', 'a'))
}

fn main() {
    let puzzle_input = get_puzzle_input();

    let mut syntax_error_score = 0;

    for (idx, line) in puzzle_input.iter().enumerate() {
        let checked = check_line(line);
        if checked.0 == false {
            println!("Expected {}, but found {} instead (ln: {})", checked.1.0, checked.1.1, idx + 1);

            syntax_error_score += match checked.1.1 {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137, 
                _ => 0,
            }
        }
    }

    println!("Syntax error score: {}", syntax_error_score);
}
