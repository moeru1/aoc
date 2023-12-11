use day2::solve_part1;
use day2::solve_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./bigboy03.txt").unwrap();
    println!("silver: {}", solve_part1(&file));
    println!("gold: {}", solve_part2(&file));
}
