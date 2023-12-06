use std::fs::read_to_string;

use itertools::Itertools;

static YEAR: &str = "2023";
static DAY: &str = "05";

fn main() {
    let input = &input();
    println!("{YEAR}_{DAY} a {}", solve_a(input));
    println!("{YEAR}_{DAY} b {}", solve_b(input));
}

fn input() -> String {
    read_to_string(format!("{YEAR}/input/{DAY}.txt"))
        .or_else(|_| read_to_string(format!("input/{DAY}.txt")))
        .unwrap()
}

fn solve_a(input: &str) -> u64 {
    let mut lines = input.lines();
    let start_seeds = lines
        .next()
        .expect("Has a first line")
        .strip_prefix("seeds: ")
        .expect("Strip seeds prefix")
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()
        .expect("parse seeds");
    assert!(lines.next().expect("a second line").trim().is_empty());
    let mut maps = Vec::new();
    while let Some(map) = RangeMap::from_lines(&mut lines) {
        // println!("{}", map.name);
        // println!("{:#?}", map.ranges);
        maps.push(map)
    }
    let mut a = start_seeds;
    for map in &maps {
        a = a.iter().map(|s| map.lookup(*s)).collect_vec();
        // dbg!(&map.name, &a);
    }
    *a.iter().min().expect("found a location")
}

fn solve_b(input: &str) -> usize {
    let _ = input;
    0
}

#[derive(Debug)]
struct RangeMap {
    #[allow(dead_code)]
    name: String,
    ranges: Vec<Range>,
}

impl RangeMap {
    fn from_lines<'s>(lines: &mut impl Iterator<Item = &'s str>) -> Option<RangeMap> {
        let name = lines
            .next()?
            .trim()
            .strip_suffix(" map:")
            .expect("is a map name line");
        let mut ranges = Vec::new();
        for l in lines {
            let l = l.trim();
            if l.is_empty() {
                break;
            }
            let (dest_start, src_start, len) = l
                .split_ascii_whitespace()
                .map(|x| x.parse::<u64>().expect("parse number"))
                .collect_tuple()
                .expect("line has 3 fields");
            ranges.push(Range {
                dest_start,
                src_start,
                len,
            })
        }
        Some(RangeMap {
            name: name.to_owned(),
            ranges,
        })
    }

    /// Given a source index return the corresponding destination space index.
    fn lookup(&self, src: u64) -> u64 {
        self.ranges
            .iter()
            .filter_map(|r| r.lookup(src))
            .next()
            .unwrap_or(src)
    }
}

#[derive(Debug)]
struct Range {
    dest_start: u64,
    src_start: u64,
    len: u64,
}

impl Range {
    fn lookup(&self, src: u64) -> Option<u64> {
        if src >= self.src_start && src < (self.src_start + self.len) {
            Some(self.dest_start + src - self.src_start)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    static EXAMPLE_1: &str = indoc! {"\
        seeds: 79 14 55 13

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
        56 93 4
    "};

    #[test]
    fn example_1() {
        assert_eq!(solve_a(EXAMPLE_1), 35);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 324724204);
    }

    #[test]
    fn solution_b() {
        // assert_eq!(solve_b(&input()), 13114317);
    }
}
