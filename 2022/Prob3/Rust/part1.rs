use std::collections::{HashMap, HashSet};

fn main() {
    let filename = "../input.txt";
    let file = std::fs::read_to_string(filename).unwrap();

    let priorties = create_priority_map();

    let mut common_priorities_sum = 0;

    for line in file.lines() {
        let last_index = line.len() / 2;
        let first_half = (&line[..last_index]).chars().collect::<HashSet<char>>();
        let common_char = &line[last_index..]
            .chars()
            .filter(|v| first_half.contains(v))
            .collect::<Vec<char>>();

        // Since there can be multiple copies of the same letter, we use just the first one
        common_priorities_sum += priorties[&common_char[0]];
    }

    dbg!(common_priorities_sum);
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
