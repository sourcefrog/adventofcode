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

/*!
# Approach

In every minute we can only build one robot. Essentially we want to choose a
sequence of robots, and then choose the sequence that eventually builds the most
geodes.

Each time we choose to build a robot we may need to wait some time until the
resources to build it are available. We should only ever wait to build robots
where we already have the productive capacity that we will eventually be able
to build them.

If we can't build any more robots then we we can just wait until the final
cycle without building anything else. Since geodes are never consumed by
building any robots, there is no point building if we have the option to
do anything else.

We can consider all the options by top-down recursion.
*/

#![allow(dead_code, unused_imports)]

use std::{
    cmp::{max, min},
    fmt,
    time::Instant,
};

use itertools::Itertools;

static EX: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

static INPUT: &str = include_str!("../../input/19.txt");

fn main() {
    // println!("{}", solve_a(EX));
    // println!("{}", solve_b(EX));
    // println!("{}", solve_a(&input()));
    println!("{}", solve_b(INPUT));
}

#[derive(Debug, Eq, PartialEq)]
struct Blueprint {
    id: usize,
    // Indexed by [produced][consumed]
    costs: [[usize; 4]; 4],
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBS: usize = 2;
const GEODE: usize = 3;

static RESOURCE_NAME: [&str; 4] = ["ORE", "CLAY", "OBSIDIAN", "GEODE"];

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

#[derive(Debug, Clone, Eq, PartialEq)]
struct St {
    clock: usize,
    robots: [usize; 4],
    res: [usize; 4],
}

impl St {
    /// The starting state.
    const fn start() -> St {
        St {
            clock: 1,
            robots: [1, 0, 0, 0],
            res: [0; 4],
        }
    }

    fn add_res(&self, coll: &[usize; 4]) -> St {
        let mut s = self.clone();
        for i in 0..4 {
            s.res[i] += coll[i];
        }
        s
    }

    fn can_afford(&self, blueprint: &Blueprint, robot_type: usize) -> bool {
        blueprint.costs[robot_type]
            .iter()
            .zip(&self.res)
            .all(|(cost, res)| cost <= res)
    }

    /// Build a robot of the given type in the current cycle if possible; also produce
    /// resources from all previously existing robots.
    #[must_use]
    fn try_build(&self, blueprint: &Blueprint, robot_type: usize) -> Option<St> {
        let costs = blueprint.costs[robot_type];
        if self.can_afford(blueprint, robot_type) {
            let mut res = self.res.clone();
            res.iter_mut()
                .zip(costs)
                .for_each(|(res, cost)| *res = res.checked_sub(cost).unwrap());
            res.iter_mut()
                .zip(&self.robots)
                .for_each(|(res, prod)| *res += prod);
            let mut robots = self.robots.clone();
            robots[robot_type] += 1;
            Some(St {
                res,
                robots,
                clock: self.clock + 1,
            })
        } else {
            None
        }
    }

    /// Return the successor state after all robots produce resources, and
    /// no new robots are built.
    #[must_use]
    fn just_produce(&self) -> St {
        let mut res = self.res.clone();
        res.iter_mut()
            .zip(self.robots)
            .for_each(|(re, ro)| *re += ro);
        St {
            res,
            clock: self.clock + 1,
            ..self.clone()
        }
    }
}

/// Extend this path by just producing resources and no robots to the end.
fn wait_until_end(path: &mut Vec<St>, cycle_limit: usize) {
    let last = path.last().unwrap();
    let mut res = last.res.clone();
    let robots = last.robots.clone();
    for clock in (last.clock + 1)..=cycle_limit {
        res.iter_mut()
            .zip(&robots)
            .for_each(|(res, robots)| *res += robots);
        path.push(St {
            clock,
            res: res.clone(),
            robots: robots.clone(),
        })
    }
}

/// How long do we need to wait to build a given robot type given the
/// current starting resources and number of robots, assuming it's the
/// next robot we build.
///
/// Returns None if it will never be possible; otherwise the sequence of
/// successor states leading up to production of that robot.
#[must_use]
fn wait_and_produce(
    blueprint: &Blueprint,
    start_path: &[St],
    robot_type: usize,
    cycle_limit: usize,
) -> Option<Vec<St>> {
    let last = start_path.last().unwrap();
    if blueprint.costs[robot_type]
        .iter()
        .zip(last.robots)
        .any(|(cost, robots)| *cost > 0 && robots == 0)
    {
        // This is, strictly, redundant with just timing out below, but it
        // seems a bit more efficient to never try to build things that we
        // know can't work out.
        // println!(
        //     "{last:?} does not have the robots to produce {name} robots next",
        //     name = RESOURCE_NAME[robot_type]
        // );
        return None; // will never be the next step
    }
    let mut path: Vec<St> = start_path.into();
    loop {
        let last = path.last().unwrap();
        if last.clock == cycle_limit {
            return None;
        }
        if let Some(produced) = last.try_build(blueprint, robot_type) {
            // println!(
            //     "from {start:?} can build {name} robot in {produced:?}",
            //     start = start_path.last().unwrap(),
            //     name = RESOURCE_NAME[robot_type]
            // );
            path.push(produced);
            return Some(path);
        } else {
            path.push(last.just_produce());
        }
    }
}

/// Return the sequence of states leading to the highest number of
/// geodes, starting from the given path prefix, within a given number of cycles.
#[must_use]
fn find_best_path(blueprint: &Blueprint, start_path: &[St], cycle_limit: usize) -> Vec<St> {
    // Look at the next move, which is either producing one robot or,
    // if no robots can be produced, just waiting until the end.
    //
    // This move might take one or more cycles.
    //
    // For each of the robot types, if we currently have enough resources,
    // we can produce it right away, taking one cycle. Otherwise, if we
    // do not have enough resources but we do have the right type of
    // robots, we can wait until we have enough, and then produce that
    // robot. Otherwise, if we don't have the right kind of robots to
    // produce the inputs, then this can never be the next step.
    //
    // If we can't build any kind of robot then the best option is to
    // just wait out the rest of the cycles, using the robots we already
    // have.
    //
    // Out of all these possible moves, whichever one eventually produces
    // the most geodes is the best.
    let mut best_geodes = 0;
    let mut best_path = None;
    let last_state = start_path.last().unwrap();
    // println!("look for best moves from {last_state:?}");
    for robot_type in 0..4 {
        if let Some(next_path) = wait_and_produce(blueprint, start_path, robot_type, cycle_limit) {
            // Recurse down to find the best case if we make this robot next.
            // println!(
            //     "from {last_state:?} consider building {name}",
            //     name = RESOURCE_NAME[robot_type]
            // );
            let complete_path = find_best_path(blueprint, &next_path, cycle_limit);
            check_path(&complete_path, cycle_limit);
            let rec_geodes = complete_path.last().unwrap().res[GEODE];
            // println!(
            //     "from {last_state:?} building {name} would produce {rec_geodes} geode",
            //     name = RESOURCE_NAME[robot_type]
            // );
            if rec_geodes > best_geodes {
                best_geodes = rec_geodes;
                // println!("found new best path yielding {rec_geodes}: {complete_path:?}");
                best_path = Some(complete_path);
            }
        } else {
            // println!(
            //     "{last_state:?} can't produce {name} next",
            //     name = RESOURCE_NAME[robot_type]
            // );
        }
    }
    let mut final_path = best_path.unwrap_or_else(|| start_path.into());
    wait_until_end(&mut final_path, cycle_limit);
    check_path(&final_path, cycle_limit);
    final_path
}

fn check_path(path: &[St], cycle_limit: usize) {
    assert_eq!(path.len(), cycle_limit, "bad path len {path:#?}");
    assert!(
        (1..cycle_limit).zip(path).all(|(c, st)| c == st.clock),
        "bad clocks in {path:#?}"
    );
}

// fn solve_a(input: &str) -> usize {
//     let bps = parse(input);
//     let mut totql = 0;
//     for bp in &bps {
//         println!("{bp:#?}");
//         let mut sts = vec![St {
//             robots: [1, 0, 0, 0],
//             res: [0; 4],
//         }];
//         for m in 1..=24 {
//             let mut succs = sts.into_iter().flat_map(|s| s.succ(bp)).collect_vec();
//             // println!("minute {m} new states:");
//             // println!("{succ:#?}");
//             succs.sort();
//             succs.dedup();
//             // TODO: Maybe trim out states that are strictly inferior.
//             sts = succs;
//             println!("minute {m}, {} states", sts.len());
//             let mut shrunk = Vec::new();
//             for st in sts {
//                 if !shrunk.iter().any(|x: &St| x.strictly_better(&st)) {
//                     shrunk.push(st)
//                 }
//             }
//             println!("shrunk to {}", shrunk.len());
//             sts = shrunk;
//         }
//         let best_geodes = sts.iter().map(|st| st.res[GEODE]).max().unwrap();
//         println!("final best geodes of bp {}: {}", bp.id, best_geodes);
//         totql += bp.id * best_geodes;
//     }
//     totql
// }

fn solve_b(_input: &str) -> usize {
    let bps = parse(EX);
    let cycle_limit = 25; // TODO: The problem statement quotes the values at the end so this is off by one...
    let sol = find_best_path(&bps[1], &[St::start()], cycle_limit);
    println!("best solution {sol:#?}");
    sol.last().unwrap().res[GEODE]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        // assert_eq!(solve_a(EX), 33);
    }

    #[test]
    fn solution_a() {
        // assert_eq!(solve_a(INPUT), 1981);
    }

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(INPUT), 0);
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
