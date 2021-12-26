use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "input.txt";

#[derive(Clone)]
enum Directions {
    Up(i64),
    Down(i64),
    Forward(i64),
}

fn main() {
    let input_arr = read_directions_from_file();

    println!("part-one : {}", calculate_pos(input_arr.clone()));
    println!("part-two : {}", calculate_pos_with_aim(input_arr));
}

fn read_directions_from_file() -> Vec<Directions> {
    let file = File::open(FILENAME).expect("couldn't open file");
    let lines = BufReader::new(file).lines();

    lines
        .map(|line| {
            let line = line.unwrap();
            let mut split = line.split(" ");

            let dir = split.next().unwrap();
            let num = split.next().unwrap();
            let num = num.parse::<i64>().unwrap();

            match dir {
                "down" => Directions::Down(num),
                "up" => Directions::Up(num),
                "forward" => Directions::Forward(num),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn calculate_pos(dirs: Vec<Directions>) -> i64 {
    let mut hor: i64 = 0;
    let mut depth: i64 = 0;

    dirs.iter().for_each(|dir| match dir {
        Directions::Up(x) => depth -= x,
        Directions::Down(x) => depth += x,
        Directions::Forward(x) => hor += x,
    });

    hor * depth
}

fn calculate_pos_with_aim(dirs: Vec<Directions>) -> i64 {
    let mut hor: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;

    dirs.iter().for_each(|dir| match dir {
        Directions::Up(x) => aim -= x,
        Directions::Down(x) => aim += x,
        Directions::Forward(x) => {
            hor += x;
            depth += aim * x;
        }
    });

    hor * depth
}
