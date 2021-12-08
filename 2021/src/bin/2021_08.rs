// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/8

use itertools::Itertools;

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

    // combo[i] = j   means that letter i (counting from 0) corresponds to letter j.
    for mapping in ('a'..='g').permutations(7) {
        println!("{}", mapping.iter().collect::<String>());
        if feasible(l, &mapping) {
            println!(
                ">> it's feasible, yay: {}",
                mapping.iter().collect::<String>()
            );
            let digits = r
                .iter()
                .map(|rw| translate(rw, &mapping))
                .map(|rt| DIGITS.iter().position(|x| *x == rt).unwrap())
                .fold(0, |acc, x| acc * 10 + x);
            dbg!(digits);
            return digits;
        }
    }
    unreachable!();
}

const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

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

fn solve_b_line_fail(l: &[String], r: &[String]) -> usize {
    // Find a mapping between letters in `l` and `r` by: finding a digit where there's only one
    // possible solution or where it's sufficiently constrained by previous answers...
    //
    // Keep track of what input letters can possibly map to what output letters. At first, anything
    // is possible.
    use std::collections::HashMap;
    let mut poss: HashMap<char, Vec<char>> = Default::default();
    for i in 'a'..='g' {
        for j in 'a'..='g' {
            poss.entry(i).or_default().push(j);
        }
    }
    // By looking at `r` maybe we can directly determine something about the constituents of some
    // easy digits?
    // for rw in r {
    //     println!("consider right word {rw} of len {}", rw.len());
    //     for (digit, rwd) in DIGITS.iter().enumerate() {
    //         if rwd.len() == rw.len() {
    //             println!("  could be {rwd} ({digit})");
    //         }
    //     }
    // }

    // Definitely known pairs.
    let mut known: HashMap<char, char> = Default::default();

    // Do the unique ones first
    for d in [1, 4, 7, 8] {
        let dlit = DIGITS[d];
        for i in l.iter().filter(|iw| iw.len() == dlit.len()) {
            println!("consider maybe {i} is {dlit} ({d})");
            // The letters in i can only map to the letters in dlit.
            for c in 'a'..='g' {
                if i.contains(c) {
                    poss.get_mut(&c).unwrap().retain(|x| dlit.contains(*x));
                } else {
                    // Also, the letters that are *not* in i cannot map to the letters in dlit, because
                    // they're all accounted for.
                    poss.get_mut(&c).unwrap().retain(|x| !dlit.contains(*x));
                }
                let c_poss = poss.get(&c).unwrap().iter().collect::<String>();
                println!("poss[{c}] = {c_poss}",);
                if c_poss.len() == 1 {
                    println!("{c} is definitely {c_poss}!!");
                    known.insert(c, c_poss.chars().next().unwrap());
                }
                assert!(!c_poss.is_empty());
            }
        }
    }
    // Now do the ambiguous ones?
    for d in [0, 2, 3, 5, 6, 9] {
        let dlit = DIGITS[d];
        for i in l.iter().filter(|iw| iw.len() == dlit.len()) {
            println!("consider maybe {i} is {dlit} ({d})");
        }
    }
    // So essentially we have a 10x10 matrix of possibilities, all of which are initially possible,
    // and we want to find unambiguously which one is which.
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let input = input();
        assert_eq!(solve(&input), (381, 0));
    }
}
