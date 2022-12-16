//! https://adventofcode.com/2022/day/16

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use regex::Regex;

fn main() {
    // println!("{}", solve_a(&input()));
    // println!("{}", solve_b(&input()));
    println!("{}", solve_b(EX));
    // println!("{}", solve_b(&input()));
}

struct Valve {
    rate: usize,
    tun: Vec<String>,
}

fn input() -> String {
    std::fs::read_to_string("input/16.txt").unwrap()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Opt<'a> {
    opened: Vec<&'a str>,
    pos: [&'a str; 2],
    flow: usize,
    rem: usize,
    whose: usize,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct SKy<'a> {
    opened: Vec<&'a str>,
    pos: [&'a str; 2],
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

fn solve_b(input: &str) -> usize {
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
    // If we have managed to get to the same positions, with more flow, there's no point looking any deeper.
    let mut seen: HashMap<SKy, usize> = Default::default();
    let mut que = Vec::from([Opt {
        opened: Default::default(),
        flow: 0,
        pos: ["AA".into(), "AA".into()],
        rem: 26,
        whose: 0,
    }]);
    // let mut best_opt = None;
    let mut best = 0;
    // let mut all_seen: HashSet<Vec<String>> = Default::default();
    let mut cnt = 0;
    let mut seen_hits = 0;
    while let Some(o) = que.pop() {
        // println!("{o:?}");
        cnt += 1;
        // if cnt % 100000 == 0 {
        //     println!("{o:?}");
        //     println!("{best}");
        // }
        if o.flow > best {
            best = o.flow;
            // best_opt = Some(o.clone());
            println!("new best opt {o:#?}");
            println!("queue {}, seen {}, hits {seen_hits}", que.len(), seen.len());
        }
        if o.rem == 0 {
            continue;
        }
        // let sk = SKy {
        //     opened: o.opened.clone(),
        //     pos: o.pos.clone(),
        // };
        // if let Some(seen_flow) = seen.get(&sk) {
        //     if *seen_flow > o.flow {
        //         seen_hits += 1;
        //         continue;
        //     }
        // }
        // seen.insert(sk, o.flow);
        let rem = o.rem - o.whose;
        let loc = o.pos[o.whose];
        let here = &vs[&loc];
        let whose_next = 1 - o.whose;
        let mut next_opts = Vec::new();
        if !o.opened.contains(&loc) && here.rate > 0 {
            // open this valve
            let mut opened = o.opened.clone();
            opened.push(loc.clone());
            opened.sort();
            let pos = o.pos.clone();
            next_opts.push(Opt {
                opened,
                flow: o.flow + (here.rate * (o.rem - 1)),
                pos,
                rem,
                whose: 1 - o.whose,
            })
        }
        for n in &here.tun {
            // if o.pos[1 - o.whose] == *n {
            //     continue;
            // }
            // it might be worth revisiting a place that's already open, to get somewhere else
            let mut pos = o.pos.clone();
            pos[o.whose] = n.as_str();
            next_opts.push(Opt {
                opened: o.opened.clone(),
                rem,
                flow: o.flow,
                pos,
                whose: 1 - o.whose,
            })
        }
        // also an option to just sit here!
        next_opts.push(Opt {
            whose: whose_next,
            rem,
            ..o
        });
        for o in next_opts {
            let sk = SKy {
                opened: o.opened.clone(),
                pos: o.pos.clone(),
            };
            if seen.get(&sk).cloned().unwrap_or(0) <= o.flow {
                //&& !que.contains(&o) {
                seen.insert(sk, o.flow);
                que.push(o);
            }
        }
        que.sort_by_key(|o| o.flow);
    }
    // not 974
    // not 2007
    best
}

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

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(&input()), 9900);

    // }
}
