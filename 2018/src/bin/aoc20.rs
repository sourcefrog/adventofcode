// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// #![allow(unused)]

//! https://adventofcode.com/2018/day/20

use std::collections::BTreeSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

type Coord = i32;
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct Point {
    x: Coord,
    y: Coord,
}

impl Point {
    #[allow(unused)]
    pub fn update(&mut self, dir: Dir) {
        *self = self.step(dir);
    }

    pub fn step(self, dir: Dir) -> Point {
        let mut n = self;
        match dir {
            Dir::N => n.y -= 1,
            Dir::S => n.y += 1,
            Dir::E => n.x += 1,
            Dir::W => n.x -= 1,
        };
        n
    }

    pub fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}

#[allow(unused)]
fn pt(x: Coord, y: Coord) -> Point {
    Point { x, y }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[allow(unused)]
static DIRECTIONS: [Dir; 4] = [Dir::N, Dir::S, Dir::E, Dir::W];

impl Dir {
    fn from_char(c: char) -> Dir {
        match c {
            'N' => Dir::N,
            'S' => Dir::S,
            'E' => Dir::E,
            'W' => Dir::W,
            other => panic!("unexpected character {other:?}"),
        }
    }

    #[allow(unused)]
    fn to_char(self) -> char {
        match self {
            Dir::N => 'N',
            Dir::S => 'S',
            Dir::E => 'E',
            Dir::W => 'W',
        }
    }
}

/// Map of rooms that have been visited.
///
/// All we actually have to remember is which E-W and N-S doors we've passed through.
/// If we have gone through a door, then we know there must be rooms on either side
/// of it, and we can't know about rooms if we never pass through a door to or
/// from them.
///
/// Note that we can't pass directly between neighboring rooms unless there is a
/// door.
#[derive(Default, Debug)]
struct Map {
    /// For each entry, there's a door to the north.
    n_doors: BTreeSet<Point>,
    /// For each entry there's a door to the east.
    e_doors: BTreeSet<Point>,
}

impl Map {
    /// Note that you can move from p in direction d (and so also in the
    /// opposite direction.)
    fn record_move(&mut self, p: Point, d: Dir) {
        match d {
            Dir::N => self.n_doors.insert(p),
            Dir::E => self.e_doors.insert(p),
            Dir::S => self.n_doors.insert(p.step(d)),
            Dir::W => self.e_doors.insert(p.step(d)),
        };
    }

    /// Return all rooms reachable through a door from p.
    fn neighbors(&self, p: Point) -> Vec<Point> {
        let mut v = Vec::with_capacity(4);
        if self.n_doors.contains(&p) {
            v.push(p.step(Dir::N))
        }
        if self.e_doors.contains(&p) {
            v.push(p.step(Dir::E))
        }
        let p2 = p.step(Dir::W);
        if self.e_doors.contains(&p2) {
            v.push(p2)
        }
        let p2 = p.step(Dir::S);
        if self.n_doors.contains(&p2) {
            v.push(p2)
        }
        v
    }

    /// Find the number of rooms at least 1000 doors from the origin.
    fn far_rooms(&self) -> usize {
        // Successively visit all neighboring rooms at distance `depth`, that have not yet been
        // seen, until we have no more to visit.
        let mut depth = 0;
        let mut seen = BTreeSet::new();
        let mut next = BTreeSet::new();
        let mut far_count = 0;
        next.insert(Point::origin());
        loop {
            // println!("depth {}, seen={:?}, next={:?}", depth, seen, next);
            let mut new_rooms = BTreeSet::new();
            for r in next {
                assert!(seen.insert(r));
                if depth >= 1000 {
                    far_count += 1;
                }
                for n in self.neighbors(r) {
                    if !seen.contains(&n) {
                        // println!("  visit {:?} from {:?}", n, r);
                        new_rooms.insert(n);
                    }
                }
            }
            next = new_rooms;
            if next.is_empty() {
                break;
            } else {
                depth += 1;
            }
        }
        far_count
    }

    /// Find the longest shortest-path from the origin.
    fn furthest(&self) -> usize {
        // Successively visit all neighboring rooms at distance `depth`, that have not yet been
        // seen, until we have no more to visit.
        let mut depth = 0;
        let mut seen = BTreeSet::new();
        let mut next = BTreeSet::new();
        next.insert(Point::origin());
        loop {
            // println!("depth {}, seen={:?}, next={:?}", depth, seen, next);
            let mut new_rooms = BTreeSet::new();
            for r in next {
                assert!(seen.insert(r));
                for n in self.neighbors(r) {
                    if !seen.contains(&n) {
                        // println!("  visit {:?} from {:?}", n, r);
                        new_rooms.insert(n);
                    }
                }
            }
            next = new_rooms;
            if next.is_empty() {
                return depth;
            } else {
                depth += 1;
            }
        }
    }
}

/// One of these is pushed every time we enter a new nested group, and popped
/// when finishing it. There's also one for the implicit top-level group.
#[derive(Debug)]
struct GroupState {
    /// The positions of active turtles at the point of entering this group.
    /// Each alternate branch will start from here.
    sps: BTreeSet<Point>,

    /// The total accumulated positions of active turtles at the end of
    /// evalutaing each branch. This doesn't include everything they might
    /// have moved through during the branch. But, if sub-branches multiply
    /// turtles, they'll all be here.
    eps: BTreeSet<Point>,
}

fn expand(r: &str) -> Map {
    // Walk through the string from left to right.
    //
    // When you see an open paren, push the map position onto a stack as being
    // where new siblings will start from, and push a new set of saved locations.
    //
    // On getting to a `|`, suspend this turtle (remembering its current location)
    // and start a new one at the map location of the start of this group.
    //
    // Also, keep a stack of sets of suspended turtles, that'll wake up at the
    // position we left them at.
    //
    // So when we see directions, we apply them in parallel to all the positions that were live at
    // the start of this group.
    let mut g = Vec::new();

    let mut map = Map::default();

    // Currently-live turtle positions, for the current branch.
    let mut turs = BTreeSet::new();
    turs.insert(Point::origin());

    for c in r.chars() {
        match c {
            'N' | 'E' | 'S' | 'W' => {
                let dir = Dir::from_char(c);
                let mut newturs = BTreeSet::new();
                for t in turs {
                    map.record_move(t, dir);
                    newturs.insert(t.step(dir));
                }
                turs = newturs;
            }
            '(' => {
                // Remember these starting positions, which will apply to
                // all groups inside.
                let gs = GroupState {
                    sps: turs.clone(),
                    eps: BTreeSet::new(),
                };
                // Hold onto these turtles and move them through the first
                // branch of the group.
                // println!("Starting a new group {:?}", &gs);
                g.push(gs);
            }
            '|' => {
                // Remember these final points we reached, and resume them at
                // the end of this group. Then, create new turtles starting at
                // the beginning.
                let gs = g.last_mut().unwrap();
                gs.eps.extend(&turs);
                turs = gs.sps.clone();
                // println!("Start branch of this group: {:?}", &gs);
            }
            ')' => {
                // All the final positions across all the branches, including
                // the currently active one, are the new current turtle positions.
                // Forget about the group and the start position.
                let gs = g.pop().unwrap();
                // println!("Finish group: {:?}", &gs);
                turs.extend(&gs.eps);
                // println!("After finishing group, turs={:?}", &turs);
            }
            _ => {
                panic!("unexpected char {c:?}");
            }
        }
    }

    // At the end of the string there should be no more groups open.
    assert!(g.is_empty());
    println!("Final turtles: {:?}", &turs);
    println!("{} final points", turs.len());

    map
}

/// Return the canned input with newline, ^ and $ removed.
fn load_input() -> String {
    let mut s = String::with_capacity(100_000);
    File::open("input/input20.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    s = s.trim_end().to_string();
    assert!(s.ends_with('$'), "{s:?}");
    assert!(s.starts_with('^'), "{s:?}");
    s[1..(s.len() - 1)].to_string()
}

fn solve_a() -> usize {
    let inp = load_input();
    let map = expand(&inp);
    map.furthest()
}

fn solve_b() -> usize {
    let inp = load_input();
    let map = expand(&inp);
    map.far_rooms()
}

pub fn main() {
    println!("A: furthest room: {}", solve_a());
    println!("B: far rooms: {}", solve_b());
}

#[cfg(test)]
mod test {
    use super::pt;
    use super::Point;

    #[test]
    fn test_load_input() {
        super::load_input();
    }

    #[test]
    fn test_expand_input() {
        let inst = super::load_input();
        println!("{:?} bytes of input", inst.len());
    }

    #[test]
    fn example0() {
        let map = super::expand("WNE");
        assert_eq!(map.furthest(), 3);
    }

    #[test]
    fn example1() {
        // Example given in the problem statement:
        //
        // #########
        // #.|.|.|.#
        // #-#######
        // #.|.|.|.#
        // #-#####-#
        // #.#.#X|.#
        // #-#-#####
        // #.|.|.|.#
        // #########
        //
        // They're all sorted by x first then y, and y runs downwards. X is the origin in the
        // diagram.
        let map = super::expand("ENWWW(NEEE|SSE(EE|N))");
        let e_doors: Vec<Point> = map.e_doors.iter().cloned().collect();
        assert_eq!(
            e_doors,
            vec![
                pt(-2, -2),
                pt(-2, -1),
                pt(-2, 1),
                pt(-1, -2),
                pt(-1, -1),
                pt(-1, 1),
                pt(0, -2),
                pt(0, -1),
                pt(0, 0),
                pt(0, 1)
            ]
        );
        let n_doors: Vec<Point> = map.n_doors.iter().cloned().collect();
        assert_eq!(
            n_doors,
            vec![pt(-2, -1), pt(-2, 0), pt(-2, 1), pt(-1, 1), pt(1, 0),]
        );
        assert_eq!(map.furthest(), 10);
    }

    #[test]
    fn example2() {
        let map = super::expand("ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN");
        assert_eq!(map.furthest(), 18);
    }

    #[test]
    fn example3() {
        let map = super::expand("ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))");
        assert_eq!(map.furthest(), 23);
    }

    #[test]
    fn example4() {
        let map = super::expand("WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))");
        assert_eq!(map.furthest(), 31);
    }

    #[test]
    fn solve_20a() {
        let map = super::expand(&super::load_input());
        assert_eq!(map.furthest(), 3725);
    }

    #[test]
    fn solve_20b() {
        let map = super::expand(&super::load_input());
        assert_eq!(map.far_rooms(), 8541);
    }
}
