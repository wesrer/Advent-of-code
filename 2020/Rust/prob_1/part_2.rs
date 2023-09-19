fn get_file_string() -> String {
    let file = std::fs::read("data").unwrap();
    let file_str = String::from_utf8_lossy(&file);
    file_str.to_string()
}

fn main() {
    use std::collections::HashSet;
    let file_str: String = get_file_string();

    let nums = file_str
        .as_str()
        .split('\n')
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<HashSet<isize>>();

    for outer in nums.iter() {
        let sum = 2020 - outer;
        for inner in nums.iter() {
            let other = sum - inner;
            if nums.contains(&other) {
                dbg!(inner * outer * other);
                // Manually exiting if you find a single match
                std::process::exit(2);
            }
        }
    }
}
