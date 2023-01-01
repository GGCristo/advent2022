use std::cmp::Ordering;

fn part_1(matrix: &Vec<Vec<u8>>) -> usize {
    let mut visible = 0usize;
    let column_len = matrix.len();
    for r in 0..column_len {
        let row_len = matrix[r].len();
        for c in 0..row_len {
            let value = matrix[r][c];
            if (0..c).all(|c_i| matrix[r][c_i] < value)
                || (c + 1..row_len).all(|c_i| matrix[r][c_i] < value)
                || (0..r).all(|r_i| matrix[r_i][c] < value)
                || (r + 1..column_len).all(|r_i| matrix[r_i][c] < value)
                || r == 0
                || c == 0
                || r == row_len - 1
                || c == column_len - 1
            {
                visible += 1;
            }
        }
    }
    visible
}

fn get_view_count<I, F>(indixes: I, cmp1: F, cmp2: u8) -> usize
where
    I: Iterator<Item = usize>,
    F: Fn(usize) -> u8,
{
    let mut count = 0usize;
    for i in indixes {
        match cmp1(i).cmp(&cmp2) {
            Ordering::Less => count += 1,
            _ => return count + 1,
        }
    }
    count
}

fn part_2(matrix: Vec<Vec<u8>>) -> usize {
    let mut max_scenic_score = 0usize;
    let column_len = matrix.len();
    for r in 1..column_len - 1 {
        let row_len = matrix[r].len();
        for c in 1..row_len - 1 {
            let tree_height = matrix[r][c];
            let top_view = get_view_count((0..r).rev(), |r_i| matrix[r_i][c], tree_height);
            let bottom_view = get_view_count(r + 1..column_len, |r_i| matrix[r_i][c], tree_height);
            let left_view = get_view_count((0..c).rev(), |c_i| matrix[r][c_i], tree_height);
            let right_view = get_view_count(c + 1..row_len, |c_i| matrix[r][c_i], tree_height);
            max_scenic_score = std::cmp::max(
                top_view * bottom_view * left_view * right_view,
                max_scenic_score,
            );
        }
    }
    max_scenic_score
}

fn main() {
    let input = include_str!("../input.txt");
    let matrix: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("couldn't convert into range 0-9") as u8)
                .collect()
        })
        .collect();
    println!("Part_1: {}", part_1(&matrix));
    println!("Part_1: {}", part_2(matrix));
}
