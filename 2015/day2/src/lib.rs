use nom::{
    bytes::complete::tag, character::complete::newline, character::complete::u32,
    multi::separated_list1, sequence::preceded, sequence::tuple, IResult,
};

#[derive(Debug)]
struct RectBox {
    l: u32,
    h: u32,
    w: u32,
}

impl RectBox {
    fn area(&self) -> u32 {
        let l = self.l;
        let h = self.h;
        let w = self.w;
        2 * l * w + 2 * w * h + 2 * h * l
    }
    fn area_smallest_side(&self) -> u32 {
        let (s1, s2) = self.smallest_side();
        s1 * s2
    }
    fn perimeter_smallest_side(&self) -> u32 {
        let (s1, s2) = self.smallest_side();
        2 * s1 + 2 * s2
    }
    fn smallest_side(&self) -> (u32, u32) {
        let mut v = [self.l, self.h, self.w];
        v.sort();
        (v[0], v[1])
    }
}

fn parse_single(input: &str) -> IResult<&str, u32> {
    preceded(tag("x"), u32)(input)
}

fn parse_rectbox(input: &str) -> IResult<&str, RectBox> {
    let (remaining, (l, h, w)) = tuple((u32, parse_single, parse_single))(input)?;
    Ok((remaining, RectBox { l, h, w }))
}

fn parser(input: &str) -> IResult<&str, Vec<RectBox>> {
    let (remaining, boxes) = separated_list1(newline, parse_rectbox)(input)?;
    Ok((remaining, boxes))
}

pub fn solve_part1(input: &str) -> String {
    let (_, boxes) = parser(input).unwrap();
    let paper_needed: u32 = boxes
        .iter()
        .map(|rbox| rbox.area() + rbox.area_smallest_side())
        .sum();
    paper_needed.to_string()
}

pub fn solve_part2(input: &str) -> String {
    let (_, boxes) = parser(input).unwrap();
    let ribbon_needed: u32 = boxes
        .iter()
        .map(|rbox| {
             println!("{}", rbox.perimeter_smallest_side());
             rbox.perimeter_smallest_side() + 
             rbox.l * rbox.w * rbox.h
        })
        .sum();
    ribbon_needed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2x3x4\n1x1x10";

    #[test]
    fn part1_works() {
        //let result = solve_part1(INPUT);
        //assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "48");
    }
}
