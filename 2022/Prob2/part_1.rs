use std::fs;
use std::convert::{TryFrom, TryInto};

const ROCK: isize = 1;
const PAPER: isize = 2;
const SCISSORS: isize = 3;
const WIN: isize = 6;
const DRAW: isize = 3;
const LOSE: isize = 0;

enum FirstCol {
    A = ROCK,
    B = PAPER,
    C = SCISSORS
}

impl TryFrom<&str> for FirstCol {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "a" | "A" => Ok(Self::A),
            "b" | "B" => Ok(Self::B),
            "c" | "C" => Ok(Self::C),
            _ => Err(()) 
        }
    }
}

enum SecondCol {
    X = ROCK, 
    Y = PAPER,
    Z = SCISSORS
}

impl TryFrom<&str> for SecondCol {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x" | "X" => Ok(Self::X),
            "y" | "Y" => Ok(Self::Y),
            "z" | "Z" => Ok(Self::Z),
            _ => Err(()) 
        }
    }
}

fn game((opp, player): (isize, isize)) -> isize{
    player + match (opp, player) {
        (ROCK, PAPER) | (PAPER, SCISSORS) | (SCISSORS, ROCK) => WIN,
        (ROCK, ROCK) | (PAPER, PAPER) | (SCISSORS, SCISSORS) => DRAW,
        _ => LOSE
    }
}


fn main(){

    let text = fs::read_to_string("./input.txt").unwrap();

    let mut points = 0;
    for line in text.lines() {
        let chars = line.split(' ').collect::<Vec<&str>>();

        let first = chars[0];
        let second = chars [1];

        let opp: FirstCol = first.try_into().unwrap();
        let player: SecondCol = second.try_into().unwrap();

        points += game((opp as isize, player as isize));
    }

    dbg!(points);

}