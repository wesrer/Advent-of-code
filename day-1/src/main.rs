use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "input.txt";

fn main() {
    let input_arr = read_numbers_from_input_file();

    let part_1_answer = part_1(input_arr.clone());
    let part_2_answer = part_2(input_arr.clone());

    println!("part 1 answer: {}", part_1_answer);
    println!("part 2 answer: {}", part_2_answer);
}

fn read_numbers_from_input_file() -> Vec<u64> {
    let file = File::open(FILENAME).expect("couldn't open file");
    let lines = BufReader::new(file).lines();

    lines
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect()
}

fn part_1(input_arr: Vec<u64>) -> u64 {
    let mut counter = 0;

    input_arr.iter().reduce(|acc, val| {
        if val > acc {
            counter += 1;
        }
        val
    });
    counter
}

fn part_2(input_arr: Vec<u64>) -> u64 {
    let mut counter = 0;

    input_arr.iter().enumerate().fold(0, |mut acc, (i, val)| {
        if i + 2 < (input_arr.len()) {
            let current_window = val + input_arr[i + 1] + input_arr[i + 2];
            if (current_window > acc) && (acc != 0) {
                counter += 1;
            }
            acc = current_window;
        }
        acc
    });
    counter
}
