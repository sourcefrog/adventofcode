//! https://adventofcode.com/2022/day/16

#![allow(unused_imports)]
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use aoclib::shortest_path::ShortestPath;
use itertools::Itertools;
use regex::Regex;

fn main() {
    // println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
    // println!("{}", solve_b(EX));
    // println!("{}", solve_b(&input()));
}

struct Valve {
    rate: usize,
    tun: Vec<String>,
}

fn input() -> String {
    std::fs::read_to_string("input/16.txt").unwrap()
}

#[derive(Ord, PartialOrd, Clone, Debug, Eq, PartialEq, Hash)]
struct St<'a> {
    clk: usize,
    opened: Vec<&'a str>,
    pos: [&'a str; 2],
}

impl<'a> St<'a> {
    #[must_use]
    fn open(&self, name: &'a str) -> St<'a> {
        let mut opened = self.opened.clone();
        opened.push(name);
        opened.sort();
        St {
            opened,
            ..self.clone()
        }
    }

    #[must_use]
    fn move_to(&self, actor: usize, name: &'a str) -> St<'a> {
        let mut pos = self.pos.clone();
        pos[actor] = name;
        St {
            pos,
            ..self.clone()
        }
    }

    #[must_use]
    fn sit(&self) -> St<'a> {
        St { ..self.clone() }
    }
}

fn parse(input: &str) -> BTreeMap<&str, Valve> {
    let mut vs: BTreeMap<&str, Valve> = BTreeMap::new();
    let re =
        Regex::new(r"Valve (..) has flow rate=([0-9]+); tunnels? leads? to valves? ([ ,A-Z]*)")
            .unwrap();
    for l in input.lines() {
        // dbg!(l);
        let caps = re.captures(l).unwrap();
        let name = caps.get(1).unwrap().as_str();
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
    vs
}

fn solve_b(input: &str) -> usize {
    /* OK it's essentially a shortest-path where the states are encoded as
       the set of positions of the two actors, the valves that are already open,
       and the clock. The cost of any step is the sum of the flows of the
       valves that are open at the *start* of the step regardless where we move to.
    */
    let vs = parse(input);
    let rates: BTreeMap<&str, usize> = vs
        .iter()
        .map(|(k, v)| (*k, v.rate))
        .filter(|(_, flow)| *flow > 0)
        .collect();
    let start = St {
        opened: Vec::new(),
        pos: ["AA", "AA"],
        clk: 0,
    };
    let dur = 26;
    let path = ShortestPath::<St<'_>, usize>::find(
        &start,
        |st| st.clk == dur || rates.keys().all(|v| st.opened.contains(v)),
        |st| {
            let cost: usize = rates
                .iter()
                .filter(|(k, _v)| !st.opened.contains(k))
                .map(|(_, v)| v)
                .sum();
            let mut a = vec![st.clone()];
            // each actor can either open a valve at their current location (if there is one) or move to any neighboring location or stay still
            // generate the combinations of all these moves
            for i in 0..2 {
                let mut b = Vec::new();
                for s in a {
                    let p = s.pos[i];
                    if rates.contains_key(p) && !s.opened.contains(&p) {
                        b.push(s.open(p));
                    }
                    for neig in &vs[p].tun {
                        b.push(s.move_to(i, neig));
                    }
                    b.push(s.sit());
                }
                a = b;
            }
            for s in &mut a {
                s.clk += 1;
                s.pos.sort();
            }
            a.sort();
            a.dedup();
            a.into_iter().map(move |st| (st, cost))
        },
    )
    .unwrap();
    let path_states: Vec<_> = path.path().collect_vec();
    dbg!(&path_states);
    dur * rates.values().sum::<usize>() - path.distance()
}

#[allow(dead_code)]
static EX: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex2() {
        assert_eq!(solve_b(&EX), 1707);
    }

    // #[test]
    // fn solution_a() {
    //     assert_eq!(solve_a(&input()), 9900);
    // }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 2171);
    }
}

// The problem maybe is that if we treat it as a search tree it's very deep, 52 half-steps,
// so even modest branching makes it too expensive.
//
// It's not true that we can take the best step in early minutes? (Suppose we want
// to walk past a valve to turn a more valuable one on, then come back to it?)).
//
// I notice there are lots of zeros that we can walk through but there's no point turning
// them on: we already don't turn them on.
//
// We could try to simplify the graph cutting out zeros that are not on shortest
// paths, but that seems a bit indirect.
//
// There are 15 valves we could turn on; naively we could turn them on in 15! orders.
// That's ~1e12, possible but a bit high.
//
// We could calculate the shortest path from AA to each valve. Or, all the shortest paths
// between non-zero valves.
//
/*
 Essentially we're trading off:

 - We'd rather turn on larger valves sooner.
 - Travelling to turn on one valve may put us further away from others that we want to turn on
 - Possibly it's better to turn on a smaller valve if it's closer; or possibly it's better
   to come back and get it later: it depends on the weight.

 This seems like a shortest-path-like optimization but not in the nominally literal
 space. Maybe under some transformation?

 The most promising option all else being equal is one that turns on the most valves
 at the earliest time.

 With 2 actors and 26 steps we probably can eventually turn on all the valves. Maybe it's
 just a question of the ordering and timing of turning them on? But, if they're far away
 maybe it's just not possible?

 Maybe we should preprocess it to throw out the zeros and calculate the shortest paths
 between non-zero valves. It would make it more complicated to play the steps maybe?

 It probably _won't_ work to greedily calculate the best steps for one actor at a time;
 they possibly should move past some easy option to leave it for the other?

*/
