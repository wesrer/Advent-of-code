fn read_from_file() -> String {
    let file = std::fs::read("data").unwrap();
    String::from_utf8_lossy(&file).to_string()
}

fn make_one_move(row: usize, col: usize, width: usize, height: usize) -> Option<(usize, usize)> {
    // You'll fall off the cliff if you are here
    if row == height {
        return None;
    }

    let mut fin_col = col + 3;
    if fin_col > width {
        fin_col -= width;
    }

    Some((row + 1, fin_col))
}

fn main() {
    let file_str = read_from_file();

    let field = file_str.as_str().split('\n');
    let field = field
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let height = field.len();
    let width = field[0].len();

    let mut row = 1;
    let mut col = 1;
    let mut trees = 0;

    loop {
        if let Some(x) = make_one_move(row, col, width, height) {
            (row, col) = x;
            if field[row - 1][col - 1] == '#' {
                trees += 1;
            }
        } else {
            break;
        }
    }

    dbg!(trees);
}
