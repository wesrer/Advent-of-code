// Input : A text file with a bunch of data of Calories.
//         A blank line means that it's the end of that elf's supplies
//         Find the elf carrying the most calories, and return how the amount of calories he is carrying

use std::fs;

// Assumed invariant: The numbers are always sorted
fn evict_lowest(arr: &mut [u128; 3], val: u128) {
    if val < arr[1] { // val less than mid
        arr[0] = val;
    } else if val < arr[2] {
        *arr = [arr[1], val, arr[2]];
    } else {
        *arr = [arr[1], arr[2], val]
    }
} 

fn main() {
    let data = fs::read_to_string("./input.txt").expect("unable to read file");

    let mut highest_calories_count: [u128; 3] = [0,0,0];
    let mut temp: u128 = 0;
    for c in data.lines() {
        if c == "" {
            if temp > highest_calories_count[0] {
                evict_lowest(&mut highest_calories_count, temp);
            }
            temp = 0;
            continue;
        }

        temp += c.parse::<u128>().expect("could not transform {c} into an int");
    }

    dbg![highest_calories_count];
    dbg![highest_calories_count.iter().sum::<u128>()];


}
