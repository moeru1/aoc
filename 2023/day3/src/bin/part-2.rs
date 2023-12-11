use day3::solve_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input").unwrap();
    println!("{}", solve_part2(&file));
}
