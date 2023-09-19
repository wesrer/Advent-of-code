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
    let (min, max) = (
        line_iter.next().unwrap().parse::<usize>().unwrap(),
        line_iter.next().unwrap().parse::<usize>().unwrap(),
    );

    // Pick only the first letter
    let letter = letter.chars().next().unwrap();

    let mut count = 0;
    for ch in pass.chars() {
        if ch == letter {
            count += 1;
        }
    }

    count <= max && count >= min
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
