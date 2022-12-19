//! https://adventofcode.com/2022/day/19

#![allow(dead_code, unused_imports)]

use std::{
    cmp::{max, min},
    time::Instant,
};

use itertools::Itertools;

static EX:&str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

fn main() {
    // println!("{}", solve_a(EX));
    println!("{}", solve_b(EX));
    // println!("{}", solve_a(&input()));
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
    // Enough to build any robot.
    rich: [usize; 4],
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
            // enough to build anything:
            let mut rich = [0; 4];
            for i in 0..4 {
                rich[i] = costs.iter().map(|c| c[i]).max().unwrap();
            }
            println!("rich={rich:?}");
            Blueprint {
                id: c[0],
                costs,
                rich,
            }
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
        if (0..4).any(|i| self.res[i] < bp.rich[i]) {
            // Just collect without building, maybe build later.
            succs.push(self.add_res(&coll));
        }
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

    /// True if self has at least as many of each type of robot and resource than other.
    /// Assuming they're from the same minute.
    fn strictly_better(&self, other: &St) -> bool {
        // any state with more geodes is better.
        if self.res[GEODE] > other.res[GEODE] || self.robots[GEODE] > other.robots[GEODE] {
            return true;
        }
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
    let bps = parse(input);
    let mut totql = 0;
    let start = Instant::now();
    for bp in &bps[1..2] {
        println!("{bp:#?}");
        let mut sts = vec![St {
            robots: [1, 0, 0, 0],
            res: [0; 4],
        }];
        for m in 1..=32 {
            // println!("minute {m} new states:");
            // println!("{succ:#?}");
            // succs.sort_unstable();
            // succs.dedup();
            let mut shrunk: Vec<St> = Vec::new();
            let succs: Vec<St> = sts
                .into_iter()
                .flat_map(|s| s.succ(bp))
                .unique()
                .collect_vec();
            // Any state that starts producing geodes earlier is strictly better; throw the rest away?
            let max_geo_rob = succs.iter().map(|st| st.robots[GEODE]).max().unwrap();

            for st in succs {
                if st.robots[GEODE] >= max_geo_rob
                    && !shrunk.iter().any(|x: &St| x.strictly_better(&st))
                {
                    shrunk.push(st)
                }
            }
            println!(
                "{} elapsed, minute {m}, shrunk to {}",
                start.elapsed().as_secs_f64(),
                shrunk.len()
            );
            println!("{:?}", shrunk.iter().take(10).collect_vec());
            // shrunk.sort_unstable();
            // shrunk.dedup();
            sts = shrunk;
        }
        let best_geodes = sts.iter().map(|st| st.res[GEODE]).max().unwrap();
        println!("final best geodes of bp {}: {}", bp.id, best_geodes);
        totql += bp.id * best_geodes;
    }
    totql
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

/*
 This does seem to have optimal substructure of a kind? That, given a certain amount of time
 and number of resources and robots, there is one optimal thing to do with them, and that
 sub-problem probably repeats?

 But it's hard to define "optimal" unless we know what we want to end up with.

Not obvious if we could start from one end or from the other....

There is probably no point refraining from building a robot if we have enough resources to build anything!

*/
