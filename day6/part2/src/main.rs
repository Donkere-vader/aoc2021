use std::fs;

const DAYS: usize = 256;

fn get_puzzle_input() -> Vec<usize> {
    let mut puzzle_input = Vec::new();

    let file_contents = fs::read_to_string("../puzzle_input.txt").expect("Failed to read puzzle input");

    for num in file_contents.split(",") {
        puzzle_input.push(num.parse::<usize>().unwrap());
    }

    puzzle_input
}

fn get_n_fish_from_start_num(start_num: &usize, days_to_live: &usize) -> usize {
    // let mut n_fish = 1;
    // let mut start_num = *&start_num;
    // let mut days_to_live = *days_to_live;


    // if start_num >= days_to_live {
    //     return n_fish;
    // }

    // days_to_live -= start_num;
    // start_num = -days_to_live;

    // let mut n_child_fish = (start_num / -7) + 1;
    // if start_num  as f64 % -7 as f64 == 0.0 {
    //     n_child_fish -= 1;
    // }

    // for idx in 0..(n_child_fish as usize) {
    //     n_fish += get_n_fish_from_start_num(8, &(days_to_live - (7 * idx) as usize - 1));
    // }

    // n_fish

    let days_to_live = *days_to_live;
    let fish_repro_time = *start_num;

    let mut fishes_interval_queue: [usize; 7] = [0; 7];
    let mut fishes_at_interval: [usize; 7] = [0; 7];
    fishes_at_interval[0] = 1;

    for day in *start_num..days_to_live {
        let interval_idx = (day - start_num) % 7;

        let new_interval_idx = (interval_idx + 2) % 7;
        fishes_interval_queue[new_interval_idx] += fishes_at_interval[interval_idx];

        fishes_at_interval[interval_idx] += fishes_interval_queue[interval_idx];
        fishes_interval_queue[interval_idx] = 0;
        // print!("Day: {: <3} {: <3} {: <2}", day, day - start_num, interval_idx);
        // println!("{:?} {:?}", fishes_at_interval, fishes_interval_queue);
    }

    fishes_at_interval.iter().sum::<usize>() + fishes_interval_queue.iter().sum::<usize>()
}

fn main() {
    let initial_state = get_puzzle_input();

    let mut n_fish = 0;
    for initial_state_number in initial_state.iter() {
        println!("Calculating for initial number: {}", initial_state_number);
        n_fish += get_n_fish_from_start_num(initial_state_number, &DAYS);
    }

    println!("\nNumber of fishes after {} day(s): {}", DAYS, n_fish);
}
