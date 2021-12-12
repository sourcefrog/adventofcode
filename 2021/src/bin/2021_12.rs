// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/12

use std::collections::BTreeMap;

fn main() {
    let input = input();
    println!("{:?}", solve(&input));
}

fn input() -> String {
    std::fs::read_to_string("input/12.txt").unwrap()
}

/// True if the room is all lowercase, indicating a small room.
fn small(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

fn solve(input: &str) -> (usize, usize) {
    // Connections from room k. (Connections are bidirectional; entries are added in each
    // direction.)
    let mut from: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .for_each(|(a, b)| {
            from.entry(a).or_default().push(b);
            from.entry(b).or_default().push(a);
        });

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
        for next in &from[p.last().unwrap()] {
            if *next == "end" {
                let mut q = p.clone();
                q.push(next);
                compl.push(q);
                continue;
            } else if *next == "start" {
                continue;
            } else if small(next) {
                let cntnext = p.iter().filter(|w| *w == next).count();
                if cntnext >= 2 || (cntnext == 1 && has_two_small(&p)) {
                    continue;
                }
            }
            let mut q = p.clone();
            q.push(next);
            inc.push(q);
        }
    }
    let sol_b = compl.len();

    (sol_a, sol_b)
}

// True if there is already any small room occurring twice
fn has_two_small(p: &[&str]) -> bool {
    for w in p {
        if small(w) {
            let c = p.iter().filter(|x| *x == w).count();
            assert!(c <= 2, "more than two {:?} in {:?}", w, p);
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
        assert_eq!(solve(&input), (5756, 144603));
    }
}
