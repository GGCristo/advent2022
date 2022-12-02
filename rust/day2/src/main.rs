use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Points = HashMap<&'static str, HashMap<&'static str, i32>>;

fn get_points_1(points_record: &Points, left: &str, right: &str) -> i32 {
    let outcome = &points_record["outcome"];
    points_record["shapes"][right]
        + if left == right {
            outcome["draw"]
        } else {
            match (left, right) {
                ("rock", "paper") | ("scissor", "rock") | ("paper", "scissor") => outcome["win"],
                _ => outcome["lose"],
            }
        }
}

fn get_points_2(points_record: &Points, left: &str, outcome: &str) -> i32 {
    let shapes_record = &points_record["shapes"];
    points_record["outcome"][outcome]
        + match (left, outcome) {
            ("rock", "win") | ("scissor", "lose") => shapes_record["paper"],
            ("paper", "win") | ("rock", "lose") => shapes_record["scissor"],
            ("scissor", "win") | ("paper", "lose") => shapes_record["rock"],
            _ => shapes_record[left],
        }
}

fn main() {
    let points_record = HashMap::from([
        (
            "shapes",
            HashMap::from([("rock", 1), ("paper", 2), ("scissor", 3)]),
        ),
        (
            "outcome",
            HashMap::from([("lose", 0), ("draw", 3), ("win", 6)]),
        ),
    ]);
    let encoding_1 = HashMap::from([
        ("A", "rock"),
        ("B", "paper"),
        ("C", "scissor"),
        ("X", "rock"),
        ("Y", "paper"),
        ("Z", "scissor"),
    ]);
    let encoding_2 = HashMap::from([("X", "lose"), ("Y", "draw"), ("Z", "win")]);
    let buffer = BufReader::new(File::open("input.txt").unwrap());
    let (mut points1, mut points2) = (0, 0);
    for line in buffer.lines() {
        let line = line.unwrap();
        let (left, right) = line.split_once(' ').unwrap();
        points1 += get_points_1(&points_record, encoding_1[left], encoding_1[right]);
        points2 += get_points_2(&points_record, encoding_1[left], encoding_2[right]);
    }
    println!("Part1: {} Part2: {}", points1, points2);
}
