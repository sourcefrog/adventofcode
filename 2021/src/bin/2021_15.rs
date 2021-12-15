// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/15

#![allow(unused_imports)]
use std::collections::BTreeMap;

use itertools::Itertools;

use aoclib::{point, Matrix, Point};

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/15.txt").unwrap()
}

fn solve(input: &str) -> (u32, u32) {
    let m = Matrix::from_string_lines(input).map(|c| c.to_digit(10).unwrap());

    let sol_a = walk(&m);

    let mw = m.width() as isize;
    let mh = m.height() as isize;
    let m2 = Matrix::from_fn(m.width() * 5, m.height() * 5, |p| {
        let mut v = m[(p.x % mw, p.y % mh)] + (p.x / mw) as u32 + (p.y / mh) as u32;
        while v > 9 {
            v -= 9
        }
        v
    });
    let sol_b = walk(&m2);

    (sol_a, sol_b)
}

/// Return the total risk (cost) of a walk from the top-left to bottom right.
fn walk(m: &Matrix<u32>) -> u32 {
    // Best known risk of a walk to this point.
    let mut best = Matrix::same_size(&m, u32::MAX);
    best[(0usize, 0usize)] = 0;
    // Points whose neighbors we need to consider, with the cost negated because Rust BinaryHeap
    // is a max-heap (that returns the largest first) and we want to look at the cheapest option first.
    // This is corny.
    let bottom_right = m.bottom_right();
    let mut active = std::collections::BinaryHeap::new();
    active.push((0i32, point(0, 0)));
    while let Some((neg_prisk, p)) = active.pop() {
        let prisk = (-neg_prisk) as u32;
        if p == bottom_right {
            // reached it on a lowest-cost-first path, so that's it
            return prisk
        }
        for (q, &qrisk) in m.neighbors4(p) {
            let tot = prisk + qrisk;
            if tot < best[q] {
                best[q] = tot;
                active.push((-(tot as i32), q));
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        // assert_eq!(solve(EX).0, 1588);
    }

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 652);
        assert_eq!(b, 2938);
    }
}
