use nom::{
    bytes::complete::tag, character::complete::newline, character::complete::u32,
    multi::separated_list1, sequence::preceded, sequence::tuple, IResult,
};

pub fn solve_part1(input: &str) -> String {
    input.to_string()
}

pub fn solve_part2(input: &str) -> String {
    input.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2x3x4\n1x1x10";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        //assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        //assert_eq!(result, "48");
    }
}
