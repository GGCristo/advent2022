use regex::Regex;
use std::fs;
use std::str;

type Stack = Vec<char>;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn get_data(input: &str) -> (Vec<Stack>, Vec<Instruction>) {
    let (crates_str, commands) = input
        .split_once("\n\n")
        .expect("there is no separation between crates and commands");
    let mut crate_lines_rev = crates_str.lines().rev();
    let size: usize = crate_lines_rev
        .next()
        .expect("no line with cargo levels")
        .split_whitespace()
        .count();
    let mut cargo = vec![Stack::new(); size];
    let crate_regex = Regex::new(r"\s{4}|\S{3}").unwrap();
    for level in crate_lines_rev {
        crate_regex
            .captures_iter(level)
            .map(|c| c.get(0).unwrap().as_str().trim())
            .enumerate()
            .filter(|(_, c)| !c.is_empty())
            .for_each(|(i, e)| {
                cargo[i].push(e.chars().nth(1).unwrap());
            });
    }
    let commands: Vec<Instruction> = commands
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

fn extend<T>(to: &mut Stack, iter: T, is_stack: bool)
where
    T: DoubleEndedIterator,
    Vec<char>: Extend<<T as Iterator>::Item>,
{
    if is_stack {
        to.extend(iter.rev());
        return;
    }
    to.extend(iter);
}

fn run(mut cargo: Vec<Stack>, commands: &Vec<Instruction>, is_stack: bool) -> String {
    for command in commands {
        let to_drain = (cargo[command.from].len() - command.amount)..;
        let items_to_drain: Vec<_> = cargo[command.from].drain(to_drain).collect();
        extend(&mut cargo[command.to], items_to_drain.into_iter(), is_stack);
    }
    cargo
        .into_iter()
        .filter_map(|v| v.last().copied())
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find file");
    let (cargo, instructions) = get_data(&input);
    println!(
        "Part1: {} Part2: {}",
        run(cargo.clone(), &instructions, true),
        run(cargo.clone(), &instructions, false)
    );
}
