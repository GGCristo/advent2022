use std::fs;
use std::str;

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn get_data(input: &str) -> (Vec<Vec<u8>>, Vec<Instruction>) {
    let (crates_str, commands_str) = input
        .split_once("\n\n")
        .expect("there is no separation between crates and commands");
    let mut crate_lines_rev = crates_str.lines().rev();
    let size: usize = crate_lines_rev
        .next()
        .expect("no line with cargo levels")
        .split_whitespace()
        .count();
    let mut cargo = vec![Vec::new(); size];
    for level in crate_lines_rev {
        level
            .as_bytes()
            .chunks(4)
            .enumerate()
            .filter(|(_, bytes)| bytes[0] == b'[')
            .for_each(|(stack, bytes)| cargo[stack].push(bytes[1]));
    }
    let commands: Vec<Instruction> = commands_str
        .lines()
        .map(|l| {
            let mut num_iter = l
                .split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|str| str.parse::<usize>().expect("NaN"));
            const INSTRUCTION_ERROR: &str = "instruction bad formed";
            Instruction {
                amount: num_iter.next().expect(INSTRUCTION_ERROR),
                from: num_iter.next().expect(INSTRUCTION_ERROR) - 1,
                to: num_iter.next().expect(INSTRUCTION_ERROR) - 1,
            }
        })
        .collect();
    (cargo, commands)
}

fn run(mut cargo: Vec<Vec<u8>>, commands: &Vec<Instruction>, is_stack: bool) -> String {
    for command in commands {
        let from_idx = cargo[command.from].len() - command.amount;
        let (from, to) = if command.to < command.from {
            let (left, rigth) = cargo.split_at_mut(command.from);
            (&mut rigth[0], &mut left[command.to])
        } else {
            let (left, rigth) = cargo.split_at_mut(command.to);
            (&mut left[command.from], &mut rigth[0])
        };
        if is_stack {
            to.extend(from.drain(from_idx..).rev());
        } else {
            to.extend(from.drain(from_idx..));
        }
    }
    cargo
        .into_iter()
        .filter_map(|v| v.last().copied())
        .map(|b| b as char)
        .collect()
}

fn main() {
    // let input = fs::read_to_string("input.txt").expect("Couldn't find file");
    let input = fs::read_to_string("aoc_2022_day05_large_input.txt").expect("Couldn't find file");
    let (cargo, instructions) = get_data(&input);
    println!(
        "Part1: {} Part2: {}",
        run(cargo.clone(), &instructions, true),
        run(cargo, &instructions, false),
    );
}
