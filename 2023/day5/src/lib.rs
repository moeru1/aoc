use itertools::Itertools;

pub fn solve_part1(input: &str) -> String {
    let mut it = input.split_terminator("\n\n");
    let seeds = it.next().unwrap();
    let (_, seeds) = seeds.split_once(':').unwrap();
    let seeds: Vec<u64> = seeds
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let maps = it.map(|table| {
        table.split_terminator('\n').skip(1).map(|row| {
            let (target, source, offset) = row
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            (target, source, offset)
        })
    });

    let mut min_location = u64::MAX;
    for seed in seeds {
        let mut s = seed;
        //println!("seed {seed}");
        for map_s in maps.clone() {
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
    let mut it = input.split_terminator("\n\n");
    let seeds = it.next().unwrap();
    let (_, seeds) = seeds.split_once(':').unwrap();
    let seeds: Vec<(u64, u64)> = seeds
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .tuples()
        .collect();
    //println!("{:?}", seeds);
    let maps = it.map(|table| {
        table.split_terminator('\n').skip(1).map(|row| {
            let (target, source, offset) = row
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            (target, source, offset)
        })
    });

    let maps = maps.map(Map::new);
    let r = Map::map_all(&seeds, maps);
    r.to_string()
}

type Id = usize;
type Val = u64;

struct Map {
    interval_start: Vec<u64>,
    interval_offset: Vec<u64>,
    sorted_intervals_id: Vec<Id>,
    source_to_target: Vec<u64>,
}

impl Map {
    fn new<I>(rows: I) -> Map
    where
        I: IntoIterator<Item = (u64, u64, u64)>,
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
                let id = self.sorted_intervals_id[pos - 1];
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
        assert!(
            self.interval_start.len() == self.source_to_target.len()
                && self.source_to_target.len() == self.interval_offset.len()
        );
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

    fn intersection(
        (start, end): (u64, u64),
        (start2, end2): (u64, u64),
    ) -> Option<(u64, u64)> {
        let istart = u64::max(start, start2);
        let iend = u64::min(end, end2);
        if istart <= iend {
            Some((istart, iend))
        } else {
            None
        }
    }

    fn remap(&self, (start, end): (u64, u64)) -> Vec<(u64, u64)> {
        //if start == 0 && end == 0 {
        //    println!("0!");
        //    return vec![];
        //}
        let mut intervals = Vec::new();
        
        for &id in self.sorted_intervals_id.iter() {
            let id_src = self.interval_start[id];
            let id_off = self.interval_offset[id];
            let id_end: u64 = (id_src + id_off - 1) as u64;
            let interval = Self::intersection((id_src, id_end), (start, end));
            if let Some(inter) = interval {
                let target = self.source_to_target[id] as i64;
                let shift: i64 = target - id_src as i64;
                intervals.push((inter, shift));
                if start == 46 && end == 56 {
                    //println!("(src, end) = ({},{}), interval = {:?}", id_src, id_end, inter);
                    //println!("target = {}, shift = {}", target, shift);
                }
            }
        }
        if intervals.is_empty() {
            return vec![(start, end)];
        }
        let mut remapped = Vec::from_iter(
            intervals
                .iter()
                .map(|&((l, r), shift)| ((l as i64 + shift) as u64, (r as i64 + shift) as u64)),
        );
        let n_1 = intervals.len() - 1;
        for (i, interval) in intervals.iter().enumerate().take(n_1) {
            //println!("i = {i}, len = {}", intervals.len());
            let ((_, r), _) = interval;
            let ((l_next, _), _) = intervals[i + 1];
            if l_next > r + 1 {
                remapped.push((r + 1, l_next - 1));
            }
        }

        let ((l_first, _), _) = intervals.first().unwrap();
        let ((_, r_last), _) = intervals.last().unwrap();
        if l_first > &start {
            remapped.push((start, l_first - 1));
        }
        if r_last < &end {
            remapped.push((r_last + 1, end));
        }

        return remapped;
    }

    fn map_all<I>(seeds: &[(u64, u64)], maps: I) -> u64
    where
        I: IntoIterator<Item = Map>,
    {
        let maps = maps.into_iter().collect::<Vec<_>>();
        let min = seeds
            .iter()
            .map(|&(start, offset)| (start, start + offset - 1))
            .map(|seed_interval| {
                let mut current_intervals = vec![seed_interval];
                let mut new_intervals = Vec::new();

                for (i, m) in maps.iter().enumerate() {
                    //println!("map = {i}");
                    for &curr_interval in &current_intervals {
                        //println!("{:?} mapped to:", curr_interval);
                        let mut remapped = m.remap(curr_interval);
                        //println!("{:?}", remapped);
                        new_intervals.append(&mut remapped);
                    }
                    current_intervals = new_intervals;
                    new_intervals = Vec::new();
                }
                //println!("current intervals = {:?}", current_intervals);
                let min = current_intervals
                    .iter()
                    .map(|&(left, _right)| left)
                    .min()
                    .unwrap();
                //println!("min = {:?}", min);
                min
            })
            .min();
        //println!("min_all = {:?}", min);
        min.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"seeds: 79 14 55 13

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
        assert_eq!(result, "46");
    }
}
