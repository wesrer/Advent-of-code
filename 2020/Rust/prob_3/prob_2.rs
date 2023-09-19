fn read_from_file() -> String {
    let file = std::fs::read("data").unwrap();
    String::from_utf8_lossy(&file).to_string()
}

fn make_one_move(row: usize, col: usize, move_down: usize, move_right: usize, width: usize, height: usize) -> Option<(usize, usize)> {
    let fin_row = row + move_down;
    
    if fin_row > height {
        return None;
    } 

    let mut fin_col = col + move_right;
    if fin_col > width {
        fin_col -= width;
    }

    Some((fin_row, fin_col))
}

fn traverse_slope(height: usize, width: usize, field: &Vec<Vec<char>>, move_down: usize, move_right: usize) -> usize {
    let mut row = 1;
    let mut col = 1;
    let mut trees = 0;

    loop {
        if let Some(x) = make_one_move(row, col, move_down, move_right, width, height) {
            (row, col) = x;
            if field[row - 1][col - 1] == '#' {
                trees += 1;
            }
        } else {
            break;
        }
    }

    trees
}

fn main() {
    let file_str = read_from_file();

    let field = file_str.as_str().split('\n');
    let field = field
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let height = field.len();
    let width = field[0].len();

    let trees = traverse_slope(height, width, &field, 1, 1) *
    traverse_slope(height, width, &field, 1, 3) *
    traverse_slope(height, width, &field, 1, 5) *
    traverse_slope(height, width, &field, 1, 7) *
    traverse_slope(height, width, &field, 2, 1);

    dbg!(trees);
}
