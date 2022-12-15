//! https://adventofcode.com/2022/day/15

use std::cmp::{max, min};
use std::collections::BTreeSet;

use aoclib::{point, Point};
use itertools::Itertools;
use regex::Regex;

fn main() {
    // println!("{}", solve_a(&input(), 2000000));
    println!("{}", solve_b(&input(), 4000000));
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

fn solve_b(input: &str, maxco: isize) -> isize {
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
    // We can write constraints on x+y and x-y? Call them u=x+y and v=x-y.
    // Each sensor marks out a square in u-v space. There must be one coordinate
    // in the range that is inside none of them?
    // That seems to mean all but one row has the entire range colored in...
    // Every sensor we see puts some constraints on what answers are possible.
    // But how do we find out what remains.
    // Each sensor excludes a square, leaving four possibilities u<u1, u>u2, v<v1, v>v2.
    // But all the sensors are ANDed together.
    // Could we start by working out which row it can possibly be on?
    // The row it's on ry must be such that there's at least one unmarked
    // position. That is there exists some rx such that for all sensors
    // the radius minus the distance to the row is less than the distance from
    // s.x to that row.
    // OK how do we find it?
    // Let's keep a list of (u1,v1,u2,v2) possibilities. Look at each sensor,
    // and then at all current possibilities. Each possibility might be eliminated or
    // split or left alone. Hopefully eventually there is one left!

    // Essentially this system forms a set of constraints like
    // x > 0 && x < 4000000 ...
    // && abs(x - x1) + abs(y - y1) > r1
    // && ....

    // The x distance term for each row can _at best_ be when x is either 0 or 4M.
    // That's not to say the sensor is necessarily at either side, probably not.

    // But if for any sensor both of them are impossible then this row is impossible.

    // But we could have a layout with diamonds across the board none of which is
    // individually enough to dismiss any row...

    // Since we know there's exactly one point, it must lay exactly one step outside of one or more
    // diamonds. Probably, just outside of four or more, although they might overlap in complex ways.

    // We can remember the u and v coordinates for just-outside each diamond, and then just test each
    // combination. There can't be that many?

    let mut us = BTreeSet::new();
    let mut vs = BTreeSet::new();
    for (s, b) in &sbs {
        let rad = (s.x - b.x).abs() + (s.y - b.y).abs();
        let su = s.x + s.y;
        let sv = s.x - s.y;
        us.insert(su - rad - 1);
        us.insert(su + rad + 1);
        vs.insert(sv - rad - 1);
        vs.insert(sv + rad + 1);
    }
    let mut ok = Vec::new();
    'uv: for (u, v) in us.iter().cartesian_product(vs.iter()) {
        for (s, b) in &sbs {
            let rad = (s.x - b.x).abs() + (s.y - b.y).abs();
            let su = s.x + s.y;
            let sv = s.x - s.y;
            if (u - su).abs() + (v - sv).abs() <= rad {
                continue 'uv; // too close
            }
        }
        ok.push((u, v));
        println!("maybe ok u={u},v={v}");
    }
    let mut really = Vec::new();
    'uv: for (u, v) in ok {
        let x = (u + v) / 2;
        let y = u - x;
        assert_eq!(x + y, *u);
        assert_eq!(x - y, *v);
        if !((0..=maxco).contains(&x) && (0..=maxco).contains(&y)) {
            println!("{x},{y} out of range");
            continue;
        }
        for (s, b) in &sbs {
            let rad = (s.x - b.x).abs() + (s.y - b.y).abs();
            if (x - s.x).abs() + (y - s.y).abs() <= rad {
                println!("{x},{y} too close to {s:?}");
                continue 'uv;
            }
        }
        println!("{x},{y} is ok?");
        really.push((x, y));
    }
    assert_eq!(really.len(), 1);
    really[0].0 * 4_000_000 + really[0].1
    // 717757712462 is too low
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "\
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
";

    #[test]
    fn example() {
        assert_eq!(solve_a(EXAMPLE, 10), 26);

        assert_eq!(solve_b(EXAMPLE, 20), 56000011);
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
