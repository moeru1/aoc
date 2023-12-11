use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use utf8_chars::BufReadCharsExt;
fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut reader = BufReader::new(file);
    let mut floor = 0;
    let mut basement_char = 0;
    for (i ,c) in reader.chars().filter_map(|c| c.ok()).enumerate() {
        floor = floor
            + match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
        if floor <= -1 && basement_char == 0 {
            basement_char = i + 1
        }
    }
    print!("{} {}", floor, basement_char);
    Ok(())
}
