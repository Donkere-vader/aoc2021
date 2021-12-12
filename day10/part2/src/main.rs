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

    (true, ('a', 'a'))
}

fn fix_incomplete_line(line: &String) -> Vec<char> {
    let mut characters_stack = Vec::new();

    for c in line.chars() {
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
            characters_stack.remove(characters_stack.len() - 1);
        } else {
            panic!("Invalid char '{}'", c);
        }
    }

    characters_stack.reverse();
    characters_stack
}

fn main() {
    let puzzle_input = get_puzzle_input();

    let mut auto_completion_scores: Vec<usize> = Vec::new();

    for line in puzzle_input.iter() {
        let checked = check_line(line);
        if checked.0 == false { continue }
        let mut score: usize = 0;
        let missing_chars = fix_incomplete_line(line);
        for c in missing_chars.iter() {
            score *= 5;
            score += match *c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4, 
                _ => 0,
            }
        }
        auto_completion_scores.push(score);
    }

    auto_completion_scores.sort();

    let middle_score = auto_completion_scores[auto_completion_scores.len() / 2];

    println!("Auto completion score: {}", middle_score);
}
