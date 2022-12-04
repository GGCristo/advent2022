use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::Lines;

fn get_priority(n: &u8) -> i32 {
    if (97..123).contains(n) {
        (n - 96) as i32
    } else {
        (n - 38) as i32
    }
}

fn part1(data: Lines) -> i32 {
    data.map(|l| l.as_bytes().split_at(l.len() / 2))
        .filter_map(|t| {
            let mut left = HashSet::<_>::from_iter(t.0);
            t.1.into_iter().find_map(|c| left.take(c))
        })
        .map(get_priority)
        .sum()
}

fn part2(data: Lines, group_number: usize) -> i32 {
    let v = data.map(|l| l.as_bytes()).collect::<Vec<_>>();
    let mut total_priority = 0;
    for group in v.chunks(group_number) {
        let mut counter_record = HashMap::new();
        for rucksack in group {
            for c in HashSet::<_>::from_iter(*rucksack) {
                let entry = counter_record
                    .entry(*c)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
                if *entry == group_number {
                    total_priority += get_priority(c);
                    break; // This also break the outter loop, bcs this only happens in the last rucksack
                }
            }
        }
    }
    total_priority
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't open file");
    println!(
        "Part1: {} Part2: {}",
        part1(input.lines()),
        part2(input.lines(), 3)
    );
}
