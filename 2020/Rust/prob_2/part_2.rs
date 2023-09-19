fn read_from_file() -> String {
    let file = std::fs::read("data").unwrap();
    String::from_utf8_lossy(&file).to_string()
}

fn parse_line(line: &str) -> bool {
    let mut line_iter = line.split(' ');

    let (policy, letter, pass) = (
        line_iter.next().unwrap(),
        line_iter.next().unwrap(),
        line_iter.next().unwrap(),
    );

    let mut line_iter = policy.split('-');
    let (pos1, pos2) = (
        line_iter.next().unwrap().parse::<usize>().unwrap() - 1,
        line_iter.next().unwrap().parse::<usize>().unwrap() - 1,
    );

    // Pick only the first letter
    let letter: std::primitive::char = letter.chars().next().unwrap();

    let pass: Vec<std::primitive::char> = pass.chars().collect();

    match (pass[pos1], pass[pos2]) {
        (a, b) if a == letter && b == letter => false,
        (a, _) if a == letter => true,
        (_, b) if b == letter => true,
        _ => false,
    }
}

fn main() {
    let file_str = read_from_file();

    let lines: Vec<&str> = file_str.as_str().split('\n').collect();

    let mut count = 0;
    for line in lines.iter() {
        if parse_line(line) {
            count += 1
        }
    }

    dbg!(count);
}
