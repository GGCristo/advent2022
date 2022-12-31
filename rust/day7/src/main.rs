mod file_system;
mod handler;

use handler::Handler;

fn main() {
    let handler = Handler::from_file(include_str!("../input.txt"));
    // print!("{handler}");
    println!("Part 1: {}", handler.get_total_size_not_greater_than(100_000));
    println!("Part 2: {}", handler.space_freed(70_000_000, 30_000_000));
}
