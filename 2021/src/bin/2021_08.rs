// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/8

// TODO: This might be simpler if all letters were just translated into
// numbers as they're read in, at the expense of a somewhat more
// indirect internal representation, and needing to translate more for
// debugging output, and some risk of bugs in that code.
//
// Also, lit digits could then be represented as bitsets, which would do
// away with the need for sorting, and would correspond pretty nicely to
// the problem domain of wires and lights that are lit or not.

use itertools::Itertools;

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

// The order of characters doesn't matter. Sort them to be easier to look at.
fn sorted_word(s: &str) -> String {
    let mut t = s.chars().collect::<Vec<char>>();
    t.sort();
    t.iter().collect()
}

fn solve(input: &str) -> (usize, usize) {
    let vals: Vec<(Vec<String>, Vec<String>)> = input
        .lines()
        .map(|s| {
            let (l, r) = s.split_once(" | ").unwrap();
            (
                l.split_whitespace().map(sorted_word).collect(),
                r.split_whitespace().map(sorted_word).collect(),
            )
        })
        .collect();
    let sol_a = vals
        .iter()
        .map(|(_l, r)| {
            r.iter()
                .map(|x| x.len())
                .filter(|x| [2, 4, 3, 7].contains(x))
                .count()
        })
        .sum();
    let sol_b = vals.iter().map(|(a, b)| solve_b_line(a, b)).sum();
    (sol_a, sol_b)
}

fn solve_b_line(l: &[String], r: &[String]) -> usize {
    // A new approach: there are only 8 possibilities for the meaning of 'a', then 7 for 'b' and so
    // on: 8! is a bit but not intractable, and many of them can probably be dismissed early. Maybe
    // we can just try them all?

    // mapping[i] = j means that letter i (counting 'a' = 0) corresponds to letter j.
    for mapping in ('a'..='g').permutations(7) {
        if feasible(l, &mapping) {
            // println!(
            //     ">> it's feasible, yay: {}",
            //     mapping.iter().collect::<String>()
            // );
            let digits = r
                .iter()
                .map(|rw| translate(rw, &mapping))
                .map(|rt| DIGITS.iter().position(|x| *x == rt).unwrap())
                .fold(0, |acc, x| acc * 10 + x);
            // dbg!(digits);
            return digits;
        }
    }
    unreachable!();
}

fn feasible(l: &[String], mapping: &[char]) -> bool {
    // Can mapping work for l?
    //
    // First, we can simply translate all words in l through that mapping. Each of them should be a
    // DIGIT.
    l.iter()
        .all(|lw| DIGITS.contains(&translate(lw, mapping).as_str()))
}

fn translate(l: &str, mapping: &[char]) -> String {
    let mut t: Vec<char> = l
        .chars()
        .map(|c| c as u32 - 'a' as u32)
        .map(|i| mapping[i as usize])
        .collect();
    t.sort();
    t.iter().collect::<String>()
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
