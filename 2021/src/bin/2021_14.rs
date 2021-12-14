// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/14

#![allow(unused_imports)]
use std::collections::BTreeMap;

use aoclib::{point, Matrix, Point};

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/14.txt").unwrap()
}

fn count(s: &[char], c: char) -> Option<usize> {
    let n = s.iter().filter(|&&x| x == c).count();
    if n > 0 {Some(n)} else {None}
}

fn solve(input: &str) -> (usize, usize) {
    let template: Vec<char> = input.lines().next().unwrap().chars().collect();
    let m = parse_map(input);
    dbg!(&m);

    let mut s = template.to_owned();
    for step in 1..=10 {
        // println!("{}", s.iter().collect::<String>());
        println!("step {step:2} len {:10} F={:8} O={:8}", s.len(), count(&s, 'F').unwrap(), count(&s, 'O').unwrap());
        s = expand(&s, &m);
    }

    for c in 'A'..='Z' {
        if let Some(n) = count(&s,c) {
            let rn = m.values().filter(|&&x| x == c).count();
            println!("{c} {:8} {:8}", n, rn);
        }
    }

    let most_common_count = ('A'..='Z').into_iter().flat_map(|c| count(&s, c)).max().unwrap();
    let least_common_count = ('A'..='Z').into_iter().flat_map(|c| count(&s, c)).min().unwrap();
    let sol_a = most_common_count - least_common_count;

    // So this is also like matrix exponentiation... One initial pair will produce an expanding 
    // series.... 
    // if we look at just the first O it cycles OO-OKO then OK -> OFK... then OF -> OC... 
    // then OC -> OHC ...

    // I wonder if every pair in the input is in the expansion and if every produced 
    // expansion also expands?

    // It seems so; every time we add len-1 elements - it *almost* doubles.

    // We could say, which one is added by the most rules but that's probably too simple...
    // Although it does seem true here...

    let s = template.clone();
    for i in 0..(s.len()-1) {
        assert!(m.contains_key(&[s[i], s[i+1]]));
    }
    for (k,&v) in &m {
        let nk = [k[0], v];
        assert!(m.contains_key(&nk));
    }


    let sol_b = 0;

    (sol_a, sol_b)
}

fn parse_map(input: &str) -> BTreeMap<[char; 2], char> {
    let mut from: BTreeMap<[char; 2], char> = BTreeMap::new();
    for l in input.lines().skip(2) {
        let k: Vec<char> = l.chars().take(2).collect();
        let k = [k[0], k[1]];
        let v = l.chars().skip(6).next().unwrap();
        from.insert(k, v);
    }
    from
}

fn expand(s: &[char], from: &BTreeMap<[char; 2], char>) -> Vec<char> {
    let mut next = Vec::with_capacity(s.len());
    for i in 0..(s.len() - 1) {
        let k = &s[i..(i + 2)];
        // println!("match {:?}", k);
        next.push(k[0]);
        if let Some(insert) = from.get(k) {
            next.push(*insert);
        }
    }
    next.push(s[s.len()-1]);
    next
}

#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn example() {
        let map = parse_map(EX);
        dbg!(&map);
        assert_eq!(
            expand(&"NNCB".chars().collect::<Vec<char>>(), &map),
            "NCNBCHB".chars().collect::<Vec<char>>()
        );

        assert_eq!(solve(EX).0, 1588);
    }

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 2194);
        assert_eq!(b, 0);
    }
}
