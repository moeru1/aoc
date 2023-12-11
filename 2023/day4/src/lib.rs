use std::collections::HashSet;

pub fn solve_part1(input: &str) -> String {
    let sum: u32 = input
        .split_terminator('\n')
        .map(|line| {
            let (winning, numbers) = line.split_once('|').unwrap();
            let winning = winning.split_ascii_whitespace();
            let winning_set: HashSet<&str> = HashSet::from_iter(winning);
            let numbers = numbers.split_ascii_whitespace();
            let count = numbers.filter_map(|n| winning_set.get(n)).count();
            if count == 0 {
                0
            } else {
                1 << (count - 1)
            }
        })
        .sum();
    sum.to_string()
}

fn calc_spawned(i: usize, card: &mut [Option<usize>], matching_count: &[usize]) -> usize {
    if i >= card.len() {
        return 0;
    }
    if let Some(spawned) = card[i] {
        return spawned;
    }
    let count = matching_count[i];
    let mut spawned = 1;
    for j in (i + 1)..=(i + count) {
        spawned += calc_spawned(j, card, matching_count);
    }
    card[i] = Some(spawned);
    return spawned;
}
pub fn solve_part2(input: &str) -> String {
    let matching_count = input
        .split_terminator('\n')
        .map(|line| {
            let (winning, numbers) = line.split_once('|').unwrap();
            let winning = winning.split_ascii_whitespace();
            let winning_set: HashSet<&str> = HashSet::from_iter(winning);
            let numbers = numbers.split_ascii_whitespace();
            numbers.filter(|n| winning_set.contains(n)).count()
        })
        .collect::<Vec<_>>();
    let n = matching_count.len();
    let mut card: Vec<Option<usize>> = vec![None; n];
    let spawned = (0..n).into_iter().fold(0, |acc, i| {
        acc + calc_spawned(i, &mut card, &matching_count)
    });
    spawned.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "30");
    }
}
