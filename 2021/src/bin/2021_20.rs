// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/20

#![allow(clippy::comparison_chain)] // bad warning; it's slower and no simpler
#![allow(unused_imports)]
use std::cmp::max;
use std::collections::HashSet;

use ndarray::prelude::*;

use aoclib::{point, Matrix, Point};

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/20.txt").unwrap()
}

fn solve(input: &str) -> (usize, usize) {
    let enh: Vec<bool> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect();
    assert_eq!(enh.len(), 512);
    let ii: Matrix<bool> =
        Matrix::from_string_lines(input.split_once("\n\n").unwrap().1).map(|c| *c == '#');
    let margin = 300;
    let w = ii.width();
    let h = ii.height();
    let ow = w + margin * 2;
    let oh = w + margin * 2;
    let mut o: Matrix<bool> = Matrix::new(ow, oh, false);
    let mut p: Matrix<bool> = Matrix::new(ow, oh, false);
    for x in 0..w {
        for y in 0..h {
            o[(x + margin, y + margin)] = ii[(x, y)];
        }
    }
    // println!("{}", o.to_string_lines());
    let mut sol_a: usize = 0;
    let mut sol_b = 0;

    // Everything in the infinite exterior frontier flips every cycle, because the enhancement of
    // [0;9] is 1 and vice versa. This means in odd cycles there are technically an infinite
    // number of pixels lit, but we only need an answer for even cycles.
    //
    // We could just let them flip back again but the problem is that we get artifacts around the
    // edge. Probably the elegant way to handle this is to special case edge cells, but for here
    // I just trim off one row per cycle and it works.
    for step in 1..=50 {
        let mut n_lit = 0;
        for ox in (step)..(ow as isize - 2 * step) {
            for oy in step..(oh as isize - 2 * step) {
                let mut idx = 0usize;
                for dy in [-1, 0, 1] {
                    for dx in [-1isize, 0, 1] {
                        idx <<= 1;
                        idx |= o[(ox + dx, oy + dy)] as usize;
                    }
                }
                assert!(idx < 512);
                p[(ox, oy)] = enh[idx];
                if enh[idx] {
                    n_lit += 1;
                }
            }
        }
        if step == 2 {
            sol_a = n_lit;
        } else if step == 50 {
            sol_b = n_lit;
        }
        o = p;
        // println!("{}", o.to_string_lines());
        p = Matrix::new(oh, ow, false);
    }

    (sol_a, sol_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 5884);
        assert_eq!(b, 19043);
    }
}
