//! https://adventofcode.com/2022/day/19

#![allow(dead_code, unused_imports)]

use std::cmp::{max, min};

use itertools::Itertools;

static EX:&str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

fn main() {
    // println!("{}", solve_a(EX));
    println!("{}", solve_a(&input()));
    // println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/19.txt").unwrap()
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    // Indexed by [produced][consumed]
    costs: [[usize; 4]; 4],
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBS: usize = 2;
const GEODE: usize = 3;

fn parse(input: &str) -> Vec<Blueprint> {
    let re = regex::Regex::new(
        "Blueprint (\\d+): Each ore robot costs (\\d+) ore\\. \
Each clay robot costs (\\d+) ore\\. \
Each obsidian robot costs (\\d+) ore and (\\d+) clay\\. \
Each geode robot costs (\\d+) ore and (\\d+) obsidian\\.",
    )
    .unwrap();
    input
        .lines()
        .map(|l| {
            let c = re
                .captures(l)
                .unwrap()
                .iter()
                .skip(1) // skip $0
                .map(|g| g.unwrap().as_str().parse::<usize>().unwrap())
                .collect_vec();
            assert_eq!(c.len(), 7);
            let mut costs = [[0; 4]; 4];
            costs[ORE][ORE] = c[1];
            costs[CLAY][ORE] = c[2];
            costs[OBS][ORE] = c[3];
            costs[OBS][CLAY] = c[4];
            costs[GEODE][ORE] = c[5];
            costs[GEODE][OBS] = c[6];
            Blueprint { id: c[0], costs }
        })
        .collect()
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct St {
    robots: [usize; 4],
    res: [usize; 4],
}

impl St {
    fn add_res(&self, coll: &[usize; 4]) -> St {
        let mut s = self.clone();
        for i in 0..4 {
            s.res[i] += coll[i];
        }
        s
    }

    fn afford(&self, costs: &[usize; 4]) -> bool {
        for i in 0..4 {
            if self.res[i] < costs[i] {
                return false;
            }
        }
        true
    }

    fn succ(&self, bp: &Blueprint) -> Vec<St> {
        let coll = self.robots.clone();
        let mut succs = Vec::new();
        // Just collect without building
        succs.push(self.add_res(&coll));
        // Build each robot if possible.
        for i in 0..4 {
            if self.afford(&bp.costs[i]) {
                let mut s = self.clone();
                for j in 0..4 {
                    s.res[j] -= bp.costs[i][j];
                }
                s.robots[i] += 1;
                s = s.add_res(&coll);
                succs.push(s);
            }
        }
        succs
    }

    fn strictly_better(&self, other: &St) -> bool {
        for i in 0..4 {
            if self.robots[i] < other.robots[i] || self.res[i] < other.res[i] {
                return false;
            }
        }
        true
    }
}

fn solve_a(input: &str) -> usize {
    let bps = parse(input);
    let mut totql = 0;
    for bp in &bps {
        println!("{bp:#?}");
        let mut sts = vec![St {
            robots: [1, 0, 0, 0],
            res: [0; 4],
        }];
        for m in 1..=24 {
            let mut succs = sts.into_iter().flat_map(|s| s.succ(bp)).collect_vec();
            // println!("minute {m} new states:");
            // println!("{succ:#?}");
            succs.sort();
            succs.dedup();
            // TODO: Maybe trim out states that are strictly inferior.
            sts = succs;
            println!("minute {m}, {} states", sts.len());
            let mut shrunk = Vec::new();
            for st in sts {
                if !shrunk.iter().any(|x: &St| x.strictly_better(&st)) {
                    shrunk.push(st)
                }
            }
            println!("shrunk to {}", shrunk.len());
            sts = shrunk;
        }
        let best_geodes = sts.iter().map(|st| st.res[GEODE]).max().unwrap();
        println!("final best geodes of bp {}: {}", bp.id, best_geodes);
        totql += bp.id * best_geodes;
    }
    totql
}

fn solve_b(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(solve_a(EX), 33);
    }

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(&input()), 0);
    // }
}
