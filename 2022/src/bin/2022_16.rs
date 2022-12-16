//! https://adventofcode.com/2022/day/16

use std::collections::{BTreeMap, HashSet};

use regex::Regex;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

struct Valve {
    rate: usize,
    tun: Vec<String>,
}

fn input() -> String {
    std::fs::read_to_string("input/16.txt").unwrap()
}

#[derive(Debug)]
struct Opt {
    opened: Vec<String>,
    seen: Vec<String>,
    flow: usize,
    rem: usize,
}

fn solve_a(input: &str) -> usize {
    let mut vs: BTreeMap<String, Valve> = BTreeMap::new();
    let re =
        Regex::new(r"Valve (..) has flow rate=([0-9]+); tunnels? leads? to valves? ([ ,A-Z]*)")
            .unwrap();
    for l in input.lines() {
        dbg!(l);
        let caps = re.captures(l).unwrap();
        let name = caps.get(1).unwrap().as_str().to_string();
        let rate = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let tun = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        vs.insert(name, Valve { rate, tun });
    }
    let mut opts = vec![Opt {
        opened: Vec::new(),
        seen: vec!["AA".into()],
        flow: 0,
        rem: 30,
    }];
    let mut best = 0;
    let mut all_seen: HashSet<Vec<String>> = Default::default();
    while let Some(o) = opts.pop() {
        // println!("{o:?}");
        let loc = o.seen.last().unwrap().clone();
        let here = &vs[&loc];
        best = std::cmp::max(best, o.flow);
        if o.rem == 0 {
            continue;
        }
        if !o.opened.contains(&loc) && here.rate > 0 {
            // open this valve
            let mut opened = o.opened.clone();
            opened.push(loc.clone());
            let rem = o.rem - 1;
            opts.push(Opt {
                opened,
                rem,
                flow: o.flow + (here.rate * rem),
                seen: o.seen.clone(),
            })
        }
        for n in &here.tun {
            if o.seen.contains(&n) {
                continue;
            }
            let mut seen = o.seen.clone();
            seen.push(n.clone());
            // if !all_seen.insert(seen.clone()) {
            //     continue;
            // };
            opts.push(Opt {
                opened: o.opened.clone(),
                rem: o.rem - 1,
                seen,
                flow: o.flow,
                ..o
            })
        }
    }
    best
}

fn solve_b(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 9900);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 9900);
    }
}
