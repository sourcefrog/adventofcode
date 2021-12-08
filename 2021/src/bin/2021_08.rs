// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/8

use std::fmt::{self, Write};

use itertools::Itertools;

// Signalled wires, or active lights, can all be represented as set
// bit in a u8 (since there are at most 7). 'a' is the low-order bit.
// The order between them doesn't matter.
#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct Lit(u8);

impl Lit {
    fn from_str(s: &str) -> Lit {
        s.chars()
            .map(|c| {
                assert!(('a'..='g').contains(&c));
                1 << (c as u32 - 'a' as u32)
            })
            .fold(0, |acc, x| acc | x)
            .into()
    }

    /// The number of wires or leds lit.
    fn num_set(&self) -> usize {
        self.0.count_ones() as usize
    }

    /// Transform self into output by a permutation vector.
    fn transform(&self, xf: &[usize]) -> Lit {
        debug_assert_eq!(xf.len(), 7);
        (0..=7)
            .filter(|i| self.0 & (1 << i) != 0)
            .map(|i| (1 << xf[i]) as u8)
            .fold(0, |acc, x| acc | x)
            .into()
    }
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (0..=7)
            .filter(|i| (self.0 & (1 << i)) != 0)
            .map(|i| (b'a' + i) as char)
            .try_for_each(|c| f.write_char(c))
    }
}

impl From<u8> for Lit {
    fn from(x: u8) -> Lit {
        Lit(x)
    }
}

const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn main() {
    let input = input();
    println!("{:?}", solve(&input));
}

fn input() -> String {
    std::fs::read_to_string("input/08.txt").unwrap()
}

fn solve(input: &str) -> (usize, usize) {
    let vals: Vec<(Vec<Lit>, Vec<Lit>)> = input
        .lines()
        .map(|s| {
            let (l, r) = s.split_once(" | ").unwrap();
            (
                l.split_whitespace().map(Lit::from_str).collect(),
                r.split_whitespace().map(Lit::from_str).collect(),
            )
        })
        .collect();
    let sol_a = vals
        .iter()
        .map(|(_l, r)| {
            r.iter()
                .map(|x| x.num_set())
                .filter(|x| [2, 3, 4, 7].contains(x))
                .count()
        })
        .sum();
    let digits_lit: Vec<Lit> = DIGITS.iter().map(|s| Lit::from_str(s)).collect();
    let sol_b = vals
        .iter()
        .map(|(a, b)| solve_b_line(a, b, &digits_lit))
        .sum();
    (sol_a, sol_b)
}

fn solve_b_line(l: &[Lit], r: &[Lit], digits_lit: &[Lit]) -> usize {
    // Try every possible mapping.
    for mapping in (0..=7).permutations(7) {
        // Does every number on the left correspond, under this mapping,
        // to some digit, we don't care which?
        if l.iter()
            .all(|lw| digits_lit.contains(&lw.transform(&mapping)))
        {
            // Great, this looks like a solution. Now transform the
            // right side in the same mapping, and combine them as
            // digits.
            return r
                .iter()
                .map(|rlit| rlit.transform(&mapping))
                .map(|rt| digits_lit.iter().position(|x| *x == rt).unwrap())
                .fold(0, |acc, x| acc * 10 + x);
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let input = input();
        assert_eq!(solve(&input), (381, 1023686));
    }
}
