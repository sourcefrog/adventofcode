/*!

# Day 19: Not Enough Minerals

<https://adventofcode.com/2022/day/19>

Your scans show that the lava did indeed form obsidian!

The wind has changed direction enough to stop sending lava droplets toward
you, so you and the elephants exit the cave. As you do, you notice a collection
of geodes around the pond. Perhaps you could use the obsidian to create some
geode-cracking robots and break them open?

To collect the obsidian from the bottom of the pond, you'll need waterproof
obsidian-collecting robots. Fortunately, there is an abundant amount of clay
nearby that you can use to make them waterproof.

In order to harvest the clay, you'll need special-purpose clay-collecting
robots. To make any type of robot, you'll need ore, which is also plentiful but
in the opposite direction from the clay.

Collecting ore requires ore-collecting robots with big drills. Fortunately,
you have exactly one ore-collecting robot in your pack that you can use to
kickstart the whole operation.

Each robot can collect 1 of its resource type per minute. It also takes one
minute for the robot factory (also conveniently from your pack) to construct
any type of robot, although it consumes the necessary resources available when
construction begins.

The robot factory has many blueprints (your puzzle input) you can choose from,
but once you've configured it with a blueprint, you can't change it. You'll
need to work out which blueprint is best.

(example)

Determine the quality level of each blueprint by multiplying that blueprint's
ID number with the largest number of geodes that can be opened in 24 minutes
using that blueprint. In this example, the first blueprint has ID 1 and can
open 9 geodes, so its quality level is 9. The second blueprint has ID 2 and can
open 12 geodes, so its quality level is 24. Finally, if you add up the quality
levels of all of the blueprints in the list, you get 33.

Determine the quality level of each blueprint using the largest number of
geodes it could produce in 24 minutes. What do you get if you add up the
quality level of all of the blueprints in your list?

## Part Two

While you were choosing the best blueprint, the elephants found some food on
their own, so you're not in as much of a hurry; you figure you probably have 32
minutes before the wind changes direction again and you'll need to get out of
range of the erupting volcano.

Unfortunately, one of the elephants ate most of your blueprint list! Now, only
the first three blueprints in your list are intact.

In 32 minutes, the largest number of geodes blueprint 1 (from the example
above) can open is 56. One way to achieve that is:
(example)

You no longer have enough blueprints to worry about quality levels. Instead,
for each of the first three blueprints, determine the largest number of geodes
you could open; then, multiply these three values together.

Don't worry about quality levels; instead, just determine the largest number of
geodes you could open using each of the first three blueprints. What do you get
if you multiply these numbers together?

*/

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
    // println!("{}", solve_b(EX));
    // println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
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
    let mut prod = 1;
    let start = Instant::now();
    for bp in bps.iter().take(3) {
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
            // let max_geo_rob = succs.iter().map(|st| st.robots[GEODE]).max().unwrap();
            let max_geodes = succs.iter().map(|st| st.res[GEODE]).max().unwrap();

            for st in succs {
                if st.res[GEODE] >= max_geodes
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
            // println!("{:?}", shrunk.iter().take(10).collect_vec());
            // shrunk.sort_unstable();
            // shrunk.dedup();
            sts = shrunk;
        }
        let best_geodes = sts.iter().map(|st| st.res[GEODE]).max().unwrap();
        println!("final best geodes of bp {}: {}", bp.id, best_geodes);
        prod *= best_geodes;
    }
    prod
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

Part 2, sadly 2604 is too low... But just a dumb mistake still running it on the exmaple.

We want to maximize the number of geodes. Geodes are produced only by geode robots at a rate
of one per turn. So the value of one geode robot is equal to the number of remaining turns.

Does this mean that we should always greedily produce geode robots? Maybe not; conceivably
some other pattern would let us make many more soon afterwards.

The optimal, impossible, structure is one that has the factory build a geode robot every cycle.

Is it true that we should always build a geode robot if we can? Maybe not, if that comes at
the expense of building other robots earlier that will let us build more geode robots later,
enough to make up for the difference.

7938 is also too low :(

Suppose we want to know the shortest path to produce one geode robot as soon as possible.

It's an open question whether the optimal overall answer implies producing a geode robot as
soon as possible? But assume it is helpful. We need some ore and some obsidian....

Maybe we should track robots in terms of their total eventual production? But, we're not really
trying to maximize that, except of obsidian.
*/
