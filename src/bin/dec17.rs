// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cmp::{max, min};
use std::collections::HashSet;

// use adventofcode2020::*;

pub fn main() {
    println!("17a: {}", solve_a());
    println!("17b: {}", solve_b());
}

type P3 = (isize, isize, isize);
type Map = HashSet<P3>;

fn solve_a() -> usize {
    let mut active: Map = HashSet::new();
    for (y, l) in std::fs::read_to_string("input/dec17.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                active.insert((x as isize, y as isize, 0));
            }
        }
    }

    for _i in 0..6 {
        let mut newmap: Map = HashSet::new();
        for x in -8..18 {
            for y in -8..18 {
                for z in -8..18 {
                    let p = (x, y, z);
                    let oldstate = active.contains(&p);
                    let c = litneigh(&active, &p);
                    let newstate = match (oldstate, c) {
                        (true, 2) => true,
                        (true, 3) => true,
                        (true, _) => false,
                        (false, 3) => true,
                        (false, _) => false,
                    };
                    if newstate {
                        newmap.insert(p);
                    }
                }
            }
        }
        active = newmap.clone();
    }

    active.len()
}

fn litneigh(state: &Map, (x, y, z): &P3) -> usize {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if (dx != 0 || dy != 0 || dz != 0) && state.contains(&(x + dx, y + dy, z + dz)) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn litneigh4(state: &Map4, (x, y, z, w): &P4) -> usize {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if (dx != 0 || dy != 0 || dz != 0 || dw != 0)
                        && state.contains(&(x + dx, y + dy, z + dz, w + dw))
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

type P4 = (isize, isize, isize, isize);
type Map4 = HashSet<P4>;

fn solve_b() -> usize {
    let mut active: Map4 = Map4::new();
    for (y, l) in std::fs::read_to_string("input/dec17.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                active.insert((x as isize, y as isize, 0, 0));
            }
        }
    }

    let mut minx = 0;
    let mut maxx = active.iter().map(|p| p.0).max().unwrap();
    let mut miny = 0;
    let mut maxy = active.iter().map(|p| p.1).max().unwrap();
    let mut minz = 0;
    let mut maxz = 0;
    let mut minw = 0;
    let mut maxw = 0;

    for i in 0..6 {
        let mut newmap: Map4 = Map4::new();
        for x in minx - 1..=maxx + 1 {
            for y in miny - 1..=maxy + 1 {
                for z in minz - 1..=maxz + 1 {
                    for w in minw - 1..=maxw + 1 {
                        let p = (x, y, z, w);
                        let oldstate = active.contains(&p);
                        let c = litneigh4(&active, &p);
                        let newstate = match (oldstate, c) {
                            (true, 2) => true,
                            (true, 3) => true,
                            (true, _) => false,
                            (false, 3) => true,
                            (false, _) => false,
                        };
                        if newstate {
                            newmap.insert(p);
                            minx = min(minx, x);
                            maxx = max(maxx, x);
                            miny = min(miny, y);
                            maxy = max(maxy, y);
                            minz = min(minz, z);
                            maxz = max(maxz, z);
                            minw = min(minw, w);
                            maxw = max(maxw, w);
                        }
                    }
                }
            }
        }
        println!(
            "cycle {} x={}..{} y={}..{} z={}..{} w={}..{}",
            i, minx, maxx, miny, maxy, minz, maxz, minw, maxw
        );
        active = newmap.clone();
    }

    active.len()
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 448);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 2400);
    }
}
