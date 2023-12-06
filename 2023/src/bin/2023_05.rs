//! https://adventofcode.com/2023/day/5

use std::cmp::{max, min};
use std::fs::read_to_string;

use itertools::Itertools;

static YEAR: &str = "2023";
static DAY: &str = "05";

type Range = std::ops::Range<u64>;

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
    let (mut a, phases) = parse(input);
    for phases in &phases {
        a = a.iter().map(|s| phases.lookup(*s)).collect_vec();
    }
    *a.iter().min().expect("found a location")
}

fn solve_b(input: &str) -> u64 {
    /*
    In the real data, the ranges are too large to probe every contained value
    individually; instead we need a way to take something analogous to the
    product or composition of two range maps.

    This can be done one step at a time through the stages.

    Given a list of input ranges, we produce a list of output ranges. For each
    input range, match it against each span; typically this will split it into
    up to three parts (before, matching, and after.) Anything that remains unmatched
    against _any_ span keeps its original value.
    */
    // let (mut a, maps) = parse(input);
    let (seeds, phases) = parse(input);
    let mut ranges = seeds
        .into_iter()
        .tuples::<(u64, u64)>()
        .map(|(start, len)| start..(start + len))
        .collect_vec();
    // dbg!(&ranges);
    for phase in &phases {
        ranges = ranges
            .into_iter()
            .flat_map(|range| phase.map_range(range))
            .collect();
        // println!("After {}, ranges: {:#?}", phase.name, ranges);
    }
    ranges.into_iter().map(|r| r.start).min().unwrap()
}

fn parse(input: &str) -> (Vec<u64>, Vec<Phase>) {
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
    let mut phases = Vec::new();
    while let Some(phase) = Phase::from_lines(&mut lines) {
        // println!("{}", map.name);
        // println!("{:#?}", map.ranges);
        phases.push(phase)
    }
    (start_seeds, phases)
}

#[derive(Debug)]
struct Phase {
    #[allow(dead_code)]
    name: String,
    spans: Vec<Span>,
}

impl Phase {
    fn from_lines<'s>(lines: &mut impl Iterator<Item = &'s str>) -> Option<Phase> {
        let name = lines
            .next()?
            .trim()
            .strip_suffix(" map:")
            .expect("is a map name line");
        let mut spans = Vec::new();
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
            spans.push(Span {
                dest_start,
                src_start,
                len,
            })
        }
        Some(Phase {
            name: name.to_owned(),
            spans,
        })
    }

    /// Given a source index return the corresponding destination space index.
    fn lookup(&self, src: u64) -> u64 {
        self.spans
            .iter()
            .filter_map(|r| r.lookup(src))
            .next()
            .unwrap_or(src)
    }

    /// Map a range through all the spans, splitting it into multiple ranges.
    /// Parts that don't match any span remain unchanged.
    fn map_range(&self, r: Range) -> Vec<Range> {
        /* Open question: do the ranges collide? If so, is it important that we dedupe them?

        At least on my input, it turns out not to matter: we never get too many ranges to be a problem,
        and any potential collisions don't seem to cause this to get the wrong result. */
        let orig_len = r.end - r.start;
        let mut matched: Vec<Range> = Vec::new();
        let mut unmatched: Vec<Range> = vec![r];
        for span in &self.spans {
            let mut still_unmatched = Vec::new();
            for a in unmatched {
                // TODO: Split out to aoclib::range.
                if a.end <= span.src_start || a.start >= span.src_end() {
                    still_unmatched.push(a);
                } else {
                    // How much of this is before the span
                    let len_before = span.src_start.saturating_sub(a.start);
                    if len_before > 0 {
                        still_unmatched.push(a.start..(a.start + len_before));
                    }
                    let len_after = a.end.saturating_sub(span.src_end());
                    if len_after > 0 {
                        still_unmatched.push(span.src_end()..(span.src_end() + len_after));
                    }
                    let match_start = max(a.start, span.src_start);
                    let match_end = min(a.end, span.src_end());
                    assert!(match_end > match_start);
                    let match_offset = match_start.checked_sub(span.src_start).unwrap();
                    let match_len = match_end.checked_sub(match_start).unwrap();
                    let match_dst_end = span.dest_start + match_offset + match_len;
                    let match_dst_start = span.dest_start + match_offset;
                    assert!(span.dest_range().contains(&match_dst_start));
                    assert!(
                        match_dst_end <= span.dest_end(),
                        "{match_dst_end} is not in expected destination range {:?}",
                        span.dest_range()
                    );
                    matched.push(match_dst_start..match_dst_end);
                }
            }
            unmatched = still_unmatched;
        }
        // Anything still unmatched is left alone
        matched.append(&mut unmatched);
        // Check that we didn't lose anything or collide -- though colliding might potentially be valid
        // but unspecified...
        assert_eq!(
            matched.iter().map(|r| r.end - r.start).sum::<u64>(),
            orig_len
        );
        matched
    }
}

#[derive(Debug)]
struct Span {
    dest_start: u64,
    src_start: u64,
    len: u64,
}

impl Span {
    fn lookup(&self, src: u64) -> Option<u64> {
        if src >= self.src_start && src < (self.src_start + self.len) {
            Some(self.dest_start + src - self.src_start)
        } else {
            None
        }
    }

    fn src_end(&self) -> u64 {
        self.src_start + self.len
    }

    fn dest_end(&self) -> u64 {
        self.dest_start + self.len
    }

    fn dest_range(&self) -> Range {
        self.dest_start..self.dest_end()
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
    fn example_2() {
        assert_eq!(solve_b(EXAMPLE_1), 46);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 324724204);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 104070862);
    }

    #[test]
    fn test_map_range() {
        let phase = Phase {
            name: "a-to-b".to_owned(),
            spans: vec![Span {
                src_start: 10,
                dest_start: 20,
                len: 5,
            }],
        };
        assert_eq!(phase.map_range(0..5), vec![(0..5)]);
        assert_eq!(phase.map_range(0..9), vec![(0..9)]);
        assert_eq!(phase.map_range(0..10), vec![(0..10)]); // range is semi-open; 10 isn't mapped
        assert_eq!(phase.map_range(0..11), vec![20..21, 0..10]);
        assert_eq!(phase.map_range(0..20), vec![20..25, 0..10, 15..20]);
    }
}
