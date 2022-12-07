use core::str::Lines;
use std::fs;
use std::ops::RangeInclusive;

fn part_1(input: Lines) -> i32 {
    fn get_start_end(str: &str) -> [i32; 2] {
        let (begin, end) = str.split_once('-').expect("no - separator");
        [begin.parse().expect("NaN"), end.parse().expect("NaN")]
    }
    input
        .filter(|l| {
            let (left_str, right_str) = l.split_once(',').expect("no , separator");
            let (l, r) = (get_start_end(left_str), get_start_end(right_str));
            (l[0] >= r[0] && l[1] <= r[1]) || (r[0] >= l[0] && r[1] <= l[1])
        })
        .count() as i32
}

fn part_2(input: Lines) -> i32 {
    fn into_range(str: &str) -> RangeInclusive<i32> {
        let (begin, end) = str.split_once('-').expect("no - separator");
        begin.parse().expect("NaN")..=end.parse().expect("NaN")
    }
    input
        .filter_map(|l| {
            let (left_str, right_str) = l.split_once(',').expect("no , separator");
            let (mut left_range, right_range) = (into_range(left_str), into_range(right_str));
            left_range.find(|lvalue| right_range.contains(lvalue))
        })
        .count() as i32
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Couldn't open file");
    println!(
        "Part1: {}, Part2: {}",
        part_1(data.lines()),
        part_2(data.lines())
    );
}
