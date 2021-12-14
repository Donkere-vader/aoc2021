use std::fs;
use std::collections::HashMap;
use std::cell::Cell;

type Polymer = Vec<char>;
type PolymerMap = HashMap<(char, char), Cell<usize>>;

const STEPS: usize = 40;

fn get_puzzle_input() -> (Polymer, HashMap<(char, char), char>) {
    let mut rules = HashMap::new();
    
    let file_content = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");
    let x = file_content.split("\n\n").collect::<Vec<&str>>();
    let template = x[0].chars().collect::<Polymer>();
    let rules_str = x[1];

    for line in rules_str.split("\n") {
        let s = line.split(" -> ").collect::<Vec<&str>>();
        let t = s[0].chars().collect::<Polymer>();
        rules.insert((t[0], t[1]), s[1].chars().collect::<Polymer>()[0]);
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
    let (polymer, rules) = get_puzzle_input();

    let mut polymer_map: PolymerMap = HashMap::new();

    for pair in get_pairs(&polymer) {
        match polymer_map.get_mut(&pair) {
            Some(entry) => { entry.replace(entry.get() + 1); },
            None => { polymer_map.insert(pair, Cell::new(1)); }
        }
    }

    for step in 0..STEPS {
        println!("Step: {}", step);
        let mut new_polymer_map: PolymerMap = HashMap::new();

        for key in polymer_map.keys() {
            let rule_result_element = rules.get(&key).unwrap();
            let new_pairs = vec![
                (key.0, *rule_result_element),
                (*rule_result_element, key.1)
            ];

            for p in new_pairs.iter() {
                match new_polymer_map.get_mut(p) {
                    Some(entry) => { entry.replace(entry.get() + polymer_map.get(key).unwrap().get()); },
                    None => { new_polymer_map.insert(*p, Cell::new(polymer_map.get(key).unwrap().get())); }
                }
            }
        }

        polymer_map = new_polymer_map;
    }

    let mut occurances: HashMap<char, Cell<usize>> = HashMap::new();

    for pair in polymer_map.keys() {
        for c in vec![pair.0, pair.1].iter() {
            match occurances.get_mut(c) {
                Some(value) => { value.replace(value.get() + polymer_map.get(pair).unwrap().get()); },
                None => { occurances.insert(*c, Cell::new(polymer_map.get(pair).unwrap().get())); },
            }
        }
    }

    println!();
    for key in occurances.keys() {
        println!("{} -> {}", key, occurances.get(key).unwrap().get());
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

    println!("{} - {} = {}", (most_common.0 as f64 / 2.0).ceil(), (least_common.0 as f64 / 2.0).ceil(), (most_common.0 as f64 / 2.0).ceil() - (least_common.0 as f64 / 2.0).ceil());
}
