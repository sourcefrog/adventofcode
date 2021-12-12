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

/// True if the room is small (in lowercase).
fn small(s: &str) -> bool {
    s.chars().next().unwrap().is_ascii_lowercase()
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
    let mut sol_a = 0;
    while let Some(p) = inc.pop() {
        for next in &from[p.last().unwrap()] {
            if *next == "end" {
                sol_a += 1;
            } else if !p.contains(next) || !small(next) {
                let mut q = p.clone();
                q.push(next);
                inc.push(q);
            }
        }
    }

    let mut inc: Vec<Vec<&str>> = vec![vec!["start"]];
    let mut sol_b = 0;
    while let Some(p) = inc.pop() {
        for next in &from[p.last().unwrap()] {
            if *next == "end" {
                sol_b += 1;
            } else if *next != "start" && (!small(next) || !p.contains(next) || !has_two_small(&p))
            {
                let mut q = p.clone();
                q.push(next);
                inc.push(q);
            }
        }
    }

    (sol_a, sol_b)
}

/// True if there is already any small room occurring twice
fn has_two_small(p: &[&str]) -> bool {
    for (i, w) in p.iter().enumerate() {
        if small(w) && p.iter().skip(i + 1).any(|x| *x == *w) {
            return true;
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
