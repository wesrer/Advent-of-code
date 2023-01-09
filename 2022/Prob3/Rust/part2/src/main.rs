use itertools::{rciter, Itertools};
use std::collections::{HashMap, HashSet};
use std::iter::zip;

fn main() {
    let filename = "../../input.txt";
    let file = std::fs::read_to_string(filename).unwrap();

    attempt1(file.clone());
    attempt2(file.clone());
}

fn attempt2(file: String) -> usize {
    let priorities = create_priority_map();
    let mut prio_sum = 0;

    let lines = file.lines().into_iter().chunks(3);

    for mut line_group in &lines {
        line_group
            .into_iter()
            .map(|line| line.chars().collect::<HashSet<char>>()) // turn each line into a HashSet
            .reduce(|mut acc, e| {
                acc.retain(|v| e.contains(&v)); // Use a Bubble mechanism to reduce the HashSets into a common properties HashSet
                acc
            })
            .unwrap() // The reduce should always return, so we can unwrap
            .drain() // consume the elements of the reduced hashset into an iterator
            .for_each(|x| prio_sum += priorities[&x]); // Add the common sets to the bigger sum
    }
    dbg!(prio_sum)
}

fn attempt1(file: String) -> usize {
    let priorities = create_priority_map();

    let lines = rciter(file.lines());
    let take3_lines = zip(zip(&lines, &lines), &lines);

    let mut prio_sum = 0;
    for ((line1, line2), line3) in take3_lines {
        vec![line1, line2, line3]
            .iter() // turn the lines into a flattened iterator
            .map(|line| line.chars().collect::<HashSet<char>>()) // turn each line into a HashSet
            .reduce(|mut acc, e| {
                acc.retain(|v| e.contains(&v)); // Use a Bubble mechanism to reduce the HashSets into a common properties HashSet
                acc
            })
            .unwrap() // The reduce should always return, so we can unwrap
            .drain() // consume the elements of the reduced hashset into an iterator
            .for_each(|x| prio_sum += priorities[&x]); // Add the common sets to the bigger sum
    }
    dbg!(prio_sum)
}

fn create_priority_map() -> HashMap<char, usize> {
    let mut priorties = HashMap::new();
    for (i, v) in ('a'..='z').enumerate() {
        priorties.insert(v, i + 1);
    }

    for (i, v) in ('A'..='Z').enumerate() {
        priorties.insert(v, i + 27);
    }

    priorties
}
