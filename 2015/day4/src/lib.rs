use md5;

pub fn solve_part1(input: &str) -> String {
    let found = false;
    let mut counter = 1;
    while !found {
        let input = input.to_owned() + &counter.to_string();
        let digest = md5::compute(&input);
        let hash = format!("{:x}", digest);
        if hash.chars()
            .take(5)
            .collect::<String>() == "00000" {
            return counter.to_string();
        }
        counter += 1;
    }
    unreachable!()
}

pub fn solve_part2(input: &str) -> String {
    let found = false;
    let mut counter = 1;
    while !found {
        let input = input.to_owned() + &counter.to_string();
        let digest = md5::compute(&input);
        let hash = format!("{:x}", digest);
        if hash.chars()
            .take(6)
            .collect::<String>() == "000000" {
            return counter.to_string();
        }
        counter += 1;
    }
    unreachable!()
}




#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abcdef";

    #[test]
    fn it_works() {
        let result = solve_part1(INPUT);
      //  let result = "";
        let ref_solution = "609043";
        assert_eq!(result, ref_solution);
    }

    #[test]
    fn part2_works() {
    }
}
