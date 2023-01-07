pub type StartEndTuple = (usize, usize);
fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();

    let mut subsets = 0;
    for line in file.lines() {
        let (f, s) = get_parts(line);
        let (f, s) = (get_range(f), get_range(s));
        let (big, small) = order_ranges(f, s);

        if is_range_subset(big, small) {
            subsets += 1
        }
    }

    dbg!(subsets);
}

fn get_parts(line: &str) -> (&str, &str) {
    let parts = line.split(',').collect::<Vec<&str>>();
    (parts[0], parts[1])
}

fn get_range(word: &str) -> StartEndTuple {
    let parts = word.split('-').collect::<Vec<&str>>();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

// This could be generic over any T: PartialOrd
fn order_ranges(f: StartEndTuple, s: StartEndTuple) -> (StartEndTuple, StartEndTuple) {
    if f.1 - f.0 > s.1 - s.0 {
        (f, s)
    } else {
        (s, f)
    }
}

fn is_range_subset(b: StartEndTuple, s: StartEndTuple) -> bool {
    ((b.0..=b.1).contains(&s.0)) && ((b.0..=b.1).contains(&s.1))
}
