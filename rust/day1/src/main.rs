use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_calories_log<P: AsRef<Path>>(path: P) -> Vec<(usize, i32)> {
    let file = BufReader::new(File::open(path).expect("Couldn't open file"));
    let mut food_log = vec![];
    let mut sum_calories = 0;
    for line in file.lines() {
        let line = line.expect("Couldn't read line");
        if line.is_empty() {
            food_log.push((food_log.len() + 1, sum_calories));
            sum_calories = 0;
            continue;
        }
        sum_calories += line.parse::<i32>().expect("Couldn't parse into number");
    }
    food_log.push((food_log.len() + 1, sum_calories));
    food_log
}

fn get_max_calories(data: &mut Vec<(usize, i32)>, window: i32) -> &[(usize, i32)] {
    data.sort_unstable_by(|elf1, elf2| elf2.1.cmp(&elf1.1));
    &data[..window as usize]
}

fn main() {
    let mut calories_log = get_calories_log("input.txt");
    let top_3_calories = get_max_calories(&mut calories_log, 3);
    // Part 1
    println!(
        "The elf {} is the one that carries most calories with {} calories",
        top_3_calories[0].0, top_3_calories[0].1
    );
    //Part 2
    println!("Top 3 elves");
    top_3_calories
        .iter()
        .for_each(|(elf, calories)| println!("The elf {elf} carries {calories} calories"));
    println!(
        "Total: {}",
        top_3_calories
            .iter()
            .fold(0, |acc, (_, calories)| acc + calories)
    );
}
