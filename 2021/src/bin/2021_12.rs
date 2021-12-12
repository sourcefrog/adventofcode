// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/12

use aoclib::Matrix;
use std::collections::BTreeMap;

fn main() {
    let input = input();
    // let input = "start-A
    // start-b
    // A-c
    // A-b
    // b-d
    // A-end
    // b-end
    // ";
    println!("{:?}", solve(&input));
}

fn input() -> String {
    std::fs::read_to_string("input/12.txt").unwrap()
}

fn small(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

fn solve(input: &str) -> (usize, usize) {
    let mut from: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .for_each(|(a, b)| {
            from.entry(a).or_default().push(b);
            from.entry(b).or_default().push(a);
        });
    dbg!(&from);
    let mut inc: Vec<Vec<&str>> = vec![vec!["start"]];
    let mut compl: Vec<Vec<&str>> = Vec::new();
    while let Some(p) = inc.pop() {
        let last = p.last().unwrap();
        for next in &from[last] {
            if p.contains(next) && small(next) {
                continue;
            }
            let mut q = p.clone();
            q.push(next);

            if *next == "end" {
                println!("found {q:?}");
                compl.push(q);
            } else {
                inc.push(q);
            }
        }
    }
    let sol_a = compl.len();

    let mut inc: Vec<Vec<&str>> = vec![vec!["start"]];
    let mut compl: Vec<Vec<&str>> = Vec::new();
    while let Some(p) = inc.pop() {
        let last = p.last().unwrap();
        for next in &from[last] {
            let mut q = p.clone();
            q.push(next);
            if *next == "end" {
                println!("found {q:?}");
                compl.push(q);
                continue;
            }
            if *next == "start" {
                continue;
            }
            if small(next) {
                let cntnext = p.iter().filter(|w| *w == next).count();
                if cntnext >= 2 {
                    continue;
                }
                if cntnext == 1 && anytwo(&p) {
                    continue;
                }
            }

            inc.push(q);
        }
    }

    (sol_a, compl.len())
}

// True if there is already any small room occurring twice
fn anytwo(p: &Vec<&str>) -> bool {
    for w in p {
        if small(w) {
            let c = p.iter().filter(|x| *x == w).count();
            assert!(c >= 1 && c <= 2, "more than two {:?} in {:?}", w, p);
            if c == 2 {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let input = input();
        assert_eq!(solve(&input), (0, 0));
    }
}
