// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/16

#![allow(clippy::comparison_chain)] // bad warning; it's slower and no simpler
#![allow(unused_imports)]
use std::cmp::max;
use std::collections::{BTreeMap, BTreeSet};
use std::os::unix::prelude::OsStrExt;

use aoclib::{point, Matrix, Point};

enum E {
    Open,
    Close,
    N(u32),
}

impl E {
    fn add(self, b: E) -> E {
        E::P(Box::new(self), Box::new(b)).reduce()
    }

    fn reduce(mut self) -> E {
        loop {
            if self.fexplode(0) {
                continue;
            }
            break;
        }
        self
    }

    fn fexplode(&mut self, depth: usize) -> bool {
        match self {
            E::V(_) => false,
            E::P(..) if depth == 4 => {
                todo!();
                true
            }
            E::P(a, b) => a.fexplode(depth + 1) || b.fexplode(depth + 1),
        }
    }

    fn magnitude(&self) -> usize {
        todo!()
    }
}

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/16.txt").unwrap()
}

fn parse(s: &[u8]) -> (E, &[u8]) {
    if s[0] == b'[' {
        let (left, rest) = parse(&s[1..]);
        assert_eq!(rest[0], b',');
        let (right, rest) = parse(&rest[1..]);
        assert_eq!(rest[0], b']');
        (E::P(Box::new(left), right.into()), &rest[1..])
    } else {
        let v = (s[0] as char).to_digit(10).unwrap();
        (E::V(v), &s[1..])
    }
}

fn parse_line(l: &str) -> E {
    let (e, rest) = parse(l.as_bytes());
    assert_eq!(rest.len(), 0);
    e
}

fn solve(input: &str) -> (usize, u64) {
    let sol_a = input
        .lines()
        .map(|l| parse_line(l))
        .reduce(E::add)
        .unwrap()
        .magnitude();

    let sol_b = 0;

    (sol_a, sol_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn syntax() {
        let ex = "[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        for l in ex.lines().map(|l| l.as_bytes()) {
            let (_e, rest) = parse(l);
            assert_eq!(rest.len(), 0);
        }
    }

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        // assert_eq!(a, 12561);
        // assert_eq!(b, 3785);
    }
}
