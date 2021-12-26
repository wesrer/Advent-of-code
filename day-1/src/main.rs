use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "input.txt";

fn main() {
    let file = File::open(FILENAME).expect("couldn't open file");
    let lines = BufReader::new(file).lines();

    let input_arr: Vec<u64> = lines
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();

    println!("{}", input_arr.len());
    let mut counter = 0;

    input_arr.iter().reduce(|acc, val| {
        if val > acc {
            counter += 1;
        }
        val
    });

    println!("{}", counter);
}
