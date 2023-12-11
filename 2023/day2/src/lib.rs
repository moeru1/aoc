use std::cmp::max;

fn solve_line(line: &str) -> bool {
    let l = line.split(":").nth(1).unwrap().split(";").map(|group| {
        group.split(",").map(|cubes| {
            let mut two = cubes.split_whitespace().take(2);
            let num: u32 = two.next().unwrap().parse().unwrap();
            let color = two.next().unwrap();
            (num, color)
        })
    });
    for mut group in l {
        if group
            .find(|(num, color)| {
                if num > &12 && color == &"red" {
                    return true;
                } else if num > &13 && color == &"green" {
                    return true;
                } else if num > &14 && color == &"blue" {
                    return true;
                }
                return false;
            })
            .is_some()
        {
            return true;
        }
    }
    return false;
}
pub fn solve_part1(input: &str) -> String {
    let found: usize = input
        .split_terminator('\n')
        .enumerate()
        .filter_map(
            |(i, line)| {
                if !solve_line(line) {
                    Some(i + 1)
                } else {
                    None
                }
            },
        )
        .sum();
    return found.to_string();
}

fn min_cubes(line: &str) -> u32 {
    let l = line.split(":").nth(1).unwrap().split(";").map(|group| {
        group.split(",").map(|cubes| {
            let mut two = cubes.split_whitespace().take(2);
            let num: u32 = two.next().unwrap().parse().unwrap();
            let color = two.next().unwrap();
            (num, color)
        })
    });
    let mut max_blue = 0;
    let mut max_green = 0;
    let mut max_red = 0;
    for group in l {
        for (num, color) in group {
            match color.as_bytes()[0] {
                b'b' => { 
                    max_blue = max(max_blue, num); 
                }
                b'g' => {
                    max_green = max(max_green, num);
                } 
                b'r' => {
                    max_red = max(max_red, num);
                }
                _ => {panic!()}
            }
        }
    }
    max_blue * max_green * max_red
}

pub fn solve_part2(input: &str) -> String {
    let sum: u64 = input.split_terminator('\n').map(|line| min_cubes(line) as u64).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "2286");
    }
}
