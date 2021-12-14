use std::fs;
use std::collections::HashMap;
use std::cell::Cell;

type Polymer = Vec<char>;
type Rule = ((char, char), char);
const STEPS: usize = 10;

fn get_puzzle_input() -> (Polymer, Vec<Rule>) {
    let mut rules = Vec::new();
    
    let file_content = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");
    let x = file_content.split("\n\n").collect::<Vec<&str>>();
    let template = x[0].chars().collect::<Polymer>();
    let rules_str = x[1];

    for line in rules_str.split("\n") {
        let s = line.split(" -> ").collect::<Vec<&str>>();
        let t = s[0].chars().collect::<Polymer>();
        let rule = ((t[0], t[1]), s[1].chars().collect::<Polymer>()[0]);
        rules.push(rule);
    }

    (template, rules)
}

fn get_pairs(polymer: &Polymer) -> Vec<(char, char)> {
    let mut pairs = Vec::new();
    
    for i in 0..polymer.len() - 1 {
        pairs.push((polymer[i], polymer[i + 1]))
    }

    pairs
}

fn main() {
    let (mut polymer, rules) = get_puzzle_input();

    for _step in 0..STEPS {
        let mut insertions = Vec::new();

        for (idx, pair) in get_pairs(&polymer).iter().enumerate() {
            for rule in rules.iter() {
                if *pair == rule.0 {
                    insertions.push((idx + 1, rule.1));
                    break;
                }
            }
        }

        for (idx, insertion) in insertions.iter().enumerate() {
            polymer.insert(insertion.0 + idx, insertion.1);
        }
    }

    let mut occurances: HashMap<char, Cell<usize>> = HashMap::new();

    for c in polymer.iter() {
        match occurances.get_mut(c) {
            Some(value) => { value.replace(value.get() + 1); },
            None => { occurances.insert(*c, Cell::new(1)); },
        }
    }

    let mut most_common = (occurances.get(&polymer[0]).unwrap().get(), polymer[0]);
    let mut least_common = (occurances.get(&polymer[0]).unwrap().get(), polymer[0]);

    for key in occurances.keys() {
        let value = occurances.get(key).unwrap().get();
        if value > most_common.0 {
            most_common = (value, *key);
        } else if value < least_common.0 {
            least_common = (value, *key);
        }
    }

    println!("{} - {} = {}", most_common.0, least_common.0, most_common.0 - least_common.0);
}
