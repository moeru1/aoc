use std::collections::HashMap;
use std::str;
use std::array::from_fn;

struct Symbol(usize, usize);
struct Number(usize, usize);

fn adjacents((i, j): (usize, usize), grid: &Vec<&[u8]>) -> Vec<(usize, usize)> {
    let offsets = [-1, 0, 1];
    let mut adjacents = Vec::new();
    for off_i in offsets {
        for off_j in offsets {
            if off_i == 0 && off_j == 0 {
                continue;
            }
            let ai = (i as i32 + off_i) as usize;
            let aj = (j as i32 + off_j) as usize;
            if let Some(Some(val)) = grid.get(ai).map(|row| row.get(aj)) {
                adjacents.push((ai, aj));
            }
        }
    }
    return adjacents;
    // * * *
    // * x *
    // * * *
}

fn find_number_nonmarked((i, j): (usize, usize), grid: &Vec<&[u8]>, marked: &mut Vec<Vec<bool>>) -> Option<u32> {
    let mut first_digit = 0;
    let mut last_digit = 0;
    let row = grid[i];
    if !row[j].is_ascii_digit() {
        return None;
    }
    //search forward 
    for ji in (0..=j).rev() {
        if marked[i][ji] {
            return None ;
        }
        if !row[ji].is_ascii_digit() {
            break;
        }
        first_digit = ji;
    }
    //search backwards
    for ji in j..row.len() {
        if marked[i][ji] {
            return None;
        }
         if !row[ji].is_ascii_digit() {
            break;
        }
        last_digit = ji;
    }
    for ji in first_digit..=last_digit {
        marked[i][ji] = true;
    }
    return Some(
        str::from_utf8(&row[first_digit..=last_digit])
        .unwrap()
        .parse::<u32>()
        .unwrap()
        );
}

pub fn solve_part1(input: &str) -> String {
    let grid = input
        .split_terminator('\n')
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let mut adj_list = HashMap::new();
    let n = grid.len();
    let m = grid[0].len();
    let mut marked: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let mut sum = 0;
    for (i, &row) in grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if !ch.is_ascii_digit() && ch != b'.' {
                for (ai, aj) in adjacents((i, j), &grid) {
                    let num = find_number_nonmarked((ai, aj), &grid, &mut marked);
                    if let Some(num) = num { 
                        let list = adj_list.entry((i, j)).or_insert(Vec::new());
                        list.push(num);
                        sum += num;
                    }
                }
            }
        }
    }
    sum.to_string()
}

pub fn solve_part2(input: &str) -> String {
    let grid = input
        .split_terminator('\n')
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let mut adj_list = HashMap::new();
    let n = grid.len();
    let m = grid[0].len();
    let mut marked: Vec<Vec<bool>> = vec![vec![false; m]; n];
    for (i, &row) in grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if !ch.is_ascii_digit() && ch != b'.' {
                for (ai, aj) in adjacents((i, j), &grid) {
                    let num = find_number_nonmarked((ai, aj), &grid, &mut marked);
                    if let Some(num) = num { 
                        let list = adj_list.entry((i, j)).or_insert(Vec::new());
                        list.push(num);
                    }
                }
            }
        }
    }
    let mut sum: u64 = 0;
    for (sym_pos, list) in &adj_list {
        let (i, j) = sym_pos;
        if grid[*i][*j] == b'*' && list.len() == 2{
            sum += (list.iter().product::<u32>()) as u64
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "4361");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        //assert_eq!(result, "48");
    }
}
