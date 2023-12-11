use std::convert::identity;
use std::str;

fn get_indices(line: &str) -> (usize, usize) {
    let max_idx = line.len() - 1;
    let idx1 = line
        .chars()
        .enumerate()
        .find(|(_, c)| c.is_ascii_digit());
    let idx1 = idx1.map_or(max_idx, |(i,_)| i);
    // we are enumerating in reverse order
    let idx2_back = line
        .chars()
        .rev()
        .enumerate()
        .find(|(_, c)| c.is_ascii_digit());
    let idx2_back = idx2_back.map_or(max_idx, |(i, _)| i);

    let idx2 = max_idx - idx2_back;
    //println!("{} {}", idx1,idx2);
    return (idx1, idx2);
}

fn calibration_value(line: &str) -> u32 {
    let mut it = line.chars().filter(|c| c.is_ascii_digit());
    let mut it2 = it.clone();
    let f = it.nth(0).unwrap();
    let g = it2.nth_back(0).unwrap();
    let st = [f, g];
    return st.iter().collect::<String>().parse().unwrap();
}

pub fn solve_part1(input: &str) -> String {
    let a = input.lines().map(|line| calibration_value(line));
    let c: u32 = a.sum();
    return c.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.replace("one", "o1e");
        let line = line.replace("two", "t2o");
        let line = line.replace("three", "t3o");
        let line = line.replace("four", "4");
        let line = line.replace("five", "5e");
        let line = line.replace("six", "6");
        let line = line.replace("seven", "7n");
        let line = line.replace("eight", "e8t");
        let line = line.replace("nine", "n9e");
        let value = calibration_value(&line);
        sum += value;
    }
    return sum.to_string();
}
/*
pub fn solve_part2(input: &str) -> String {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let nums_rev = nums.map(|name| name.chars().rev().collect::<String>().leak() as &str);
    let mut accum: u32 = 0;
    for line in input.lines() {
        let (lb, ub) = get_indices(line);
        let slice = &line[..lb];
        let mut first_num: Option<u8> = None;
        let mut last_num: Option<u8> = None;
        for i in 0..lb {
            if let Some(_) = first_num {
                break;
            }
            for (idx, &name) in nums.iter().enumerate() {
                let slice_i = &line[i..lb];
                //println!("{:?}", slice_i);
                //println!("num: {:?}", name);
                if let Some(_) = slice_i.strip_prefix(name) {
                //    println!("FOUND!!! {}", name);
                    first_num = Some((idx+1) as u8);
                    break;
                }
            }
        }
        //ub... MAX
        //slice[ub+k] k<= MAX
        for i in ub..line.len() {
            if let Some(_) = last_num {
                break;
            }
            for (idx, &name_rev) in nums_rev.iter().enumerate() {
                let name_len = name_rev.len();
                //a1 = ub
                //[a1,a2,a3,....,ai,ai+1,...,an]
                let slice_i = &line[ub..i];
                println!("SLICE: {}", slice_i);
                let slice_rev = slice_i.as_bytes().iter().rev().take(name_len);
                let to_print = slice_rev.clone();
                println!("{:?}", to_print.map(|&u| u as char).collect::<String>());
                println!("num: {:?}", name_rev);
                if slice_rev.eq(name_rev.as_bytes().iter()) {
                    last_num = Some((idx+1) as u8);
                    break;
                }
            }
        }
        if first_num == None {
            let ch = line.as_bytes()[lb];
            first_num = Some(ch - b'0');
        }
        if last_num == None {
            let ch = line.as_bytes()[ub];
            println!("last = {}", ch as char);
            last_num = Some(ch - b'0');
        }
        println!("{line} got nums {},{}", first_num.unwrap(), last_num.unwrap());
        accum += (first_num.unwrap() * 10) as u32 + last_num.unwrap() as u32;
    }
    accum.to_string()
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT2: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "142");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT2);
        assert_eq!(result, "281")
        //assert_eq!(result, "48");
    }
}
