//! https://adventofcode.com/2022/day/15

use std::collections::BTreeSet;

use aoclib::{point, Point};
use regex::Regex;

fn main() {
    println!("{}", solve_a(&input(), 2000000));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/15.txt").unwrap()
}

fn solve_a(input: &str, tgt: isize) -> usize {
    let mut sbs: Vec<(Point, Point)> = Vec::new();
    let re = Regex::new(r"[-\d]+").unwrap();
    for l in input.lines() {
        let c = re
            .find_iter(l)
            .map(|g| g.as_str().parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        assert_eq!(c.len(), 4);
        sbs.push((point(c[0], c[1]), point(c[2], c[3])));
    }
    dbg!(&sbs);
    let mut not: BTreeSet<isize> = Default::default();
    // points that are not a beacon
    // and no further from a sensor than its beacon.
    let mut maxcnt = 0;
    for (s, b) in &sbs {
        let rad = (s.x - b.x).abs() + (s.y - b.y).abs();
        let dy = (s.y - tgt).abs();
        let mx = rad - dy;
        if mx < 0 {
            continue;
        }
        dbg!(s, b, rad, dy, mx);
        let mut cnt = 0;
        for ix in (s.x - mx)..=(s.x + mx) {
            if not.insert(ix) {
                maxcnt += 1
            };
            cnt += 1;
            // maxcnt += 1;
        }
        assert_eq!(cnt, 2 * mx + 1);
        dbg!(not.len());
    }
    for (_, b) in &sbs {
        if b.y == tgt {
            not.remove(&b.x);
            dbg!(&b);
        }
    }
    // it's not 4704090
    dbg!(&maxcnt);
    not.len()
}

fn solve_b(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            solve_a(
                "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
",
                10
            ),
            26
        );
    }

    // #[test]
    // fn solution_a() {
    //     assert_eq!(solve_a(&input()), 9900);
    // }

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(&input()), 9900);
    // }
}
