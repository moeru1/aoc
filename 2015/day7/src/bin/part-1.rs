use day7::solve_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input").unwrap();
    println!("{}", solve_part1(&file));
}
