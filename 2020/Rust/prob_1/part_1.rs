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

    for num in nums.iter() {
        let other = 2020 - num;
        if nums.contains(&other) {
            dbg!(num * other);
            // Manually exiting if you find a match
            std::process::exit(0);
        }
    }
}
