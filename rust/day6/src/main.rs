use std::collections::HashSet;
use std::fs;
use std::str;

fn run(input: &str, marker_len: usize) -> usize {
    input
        .as_bytes()
        .windows(marker_len)
        .enumerate()
        .find(|(_, b)| HashSet::<_>::from_iter(*b).len() == marker_len)
        .expect("couldn' find unique values")
        .0
        + marker_len
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't open file");
    println!("Part1 {} Part2 {}", run(&input, 4), run(&input, 14));
}
