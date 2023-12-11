use std::ops::Range;

use itertools::Itertools;

pub fn solve_part1(input: &str) -> String {
    let mut it = input.split_terminator("\n\n");
    let seeds = it.next().unwrap();
    let (_, seeds) = seeds.split_once(':').unwrap();
    let seeds: Vec<u32> = seeds
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let maps = it
        .map(|table| {
            table.split_terminator('\n').skip(1).map(|row| {
                let (target, source, offset) = row
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap();
                (target, source, offset)
            })
        });

    let mut min_location = u32::MAX;
    for seed in seeds {
        let mut s = seed;
        //println!("seed {seed}");
        for (i, map_s) in maps.clone().enumerate() {
            let map = Map::new(map_s);
            s = map.translate(&s);
            //println!("map {i} {s}");
        }
        min_location = std::cmp::min(min_location, s);
        //println!("seed {seed} location {s}");
    }
    min_location.to_string()
}

pub fn solve_part2(input: &str) -> String {
    input.to_string()
}

type Id = usize;
type Val = u32;

struct Map {
    interval_start: Vec<u32>,
    interval_offset: Vec<u32>,
    sorted_intervals_id: Vec<Id>,
    source_to_target: Vec<u32>,
}

impl Map {
    fn new<I>(rows: I) -> Map
    where
        I: IntoIterator<Item = (u32, u32, u32)>,
    {
        let mut interval_start = Vec::new();
        let mut interval_offset = Vec::new();
        let mut source_to_target = Vec::new();
        for (target_s, source_s, offset) in rows {
            interval_start.push(source_s);
            interval_offset.push(offset);
            source_to_target.push(target_s);
        }
        let n = interval_start.len();
        let mut sorted_intervals_id: Vec<Id> = Vec::from_iter(0..n);
        sorted_intervals_id.sort_by(|&a, &b| interval_start[a].cmp(&interval_start[b]));
     //   for &id in &sorted_intervals_id {
     //       println!("{}, {}", interval_start[id], interval_offset[id]);
     //   }
        Map {
            interval_start,
            interval_offset,
            sorted_intervals_id,
            source_to_target,
        }
    }

    fn containing_interval(&self, val: &Val) -> Option<Id> {
        //println!("containing_interval");
        let found = self
            .sorted_intervals_id
            .binary_search_by(|&probe| self.interval_start[probe].cmp(val));
        match found {
            Ok(pos) => {
                let id = self.sorted_intervals_id[pos];
                return Some(id);
            }
            Err(pos) => {
                if pos == 0 {
                    return None;
                }
                let id = self.sorted_intervals_id[pos-1];
                //println!("{}, {}, {}", id, self.interval_start[id], self.interval_offset[id]);
                let end: i64 = (self.interval_start[id] + self.interval_offset[id]) as i64 - 1;
                if end < *val as i64 {
                    return None;
                } else {
                    //id or i??
                    return Some(id);
                }
            }
        }
    }

    fn translate(&self, val: &Val) -> Val {
        assert!(self.interval_start.len() == self.source_to_target.len() 
                && self.source_to_target.len() == self.interval_offset.len());
        let id = self.containing_interval(val);
        match id {
            None => *val,
            Some(id) => {
                //println!("{} val {} {}",id, val, self.interval_start[id]);
                let diff = val - self.interval_start[id];
                self.source_to_target[id] + diff
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_works() {
        //let s = [0, 2, 3, 5, 8, 13, 21, 34, 55];

        //dbg!(s.binary_search(&1));
        let result = solve_part1(INPUT);
        assert_eq!(result, "35");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        //assert_eq!(result, "48");
    }
}
