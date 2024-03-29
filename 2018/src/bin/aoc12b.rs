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

#![allow(dead_code)]
//! https://adventofcode.com/2018/day/12
//!
//! Pots are simply binary.
//!
//! Evolves in generations rather than updating in place.
//!
//! The furthest it can possibly propagate out to the left or right is
//! two pots per generation.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

pub fn main() {
    let mut s = String::new();
    File::open("input/input12.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let mut p = Pots::from_string(&s);
    let n_steps = 50_000_000_000u64;
    let mut i = 0;
    loop {
        // 0.. {
        println!("left={}  {}", p.left(), p.format_pots(p.left()..=p.right()));
        let next_p = p.step();
        // if i % 100_000 == 0 {
        //     println!("i={:>10} p={:?}", i, p);
        // }
        // if next_p == p {
        //     break
        // } else {
        //     p = next_p
        // }
        if next_p.is_right_shift(&p) {
            println!("found stable right shift");
            break;
        }
        p = next_p;
        i += 1;
    }
    // The sum of number of pots that have a plant is, the current sum, plus the remaining steps times
    // the number of pots.
    println!(
        "result = {}",
        p.magic() as u64 + (n_steps - i) * p.pots.len() as u64
    );
}

fn from_b(c: u8) -> bool {
    match c {
        b'#' => true,
        b'.' => false,
        e => panic!("unexpected {e:?}"),
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Pots {
    /// Indices of pots that are occupied.
    pub pots: BTreeSet<isize>,
    /// Map of instructions from 5-bool context to new results
    pub inst: Rc<BTreeMap<[bool; 5], bool>>,
}

impl fmt::Debug for Pots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Pots {{ pots={} }}",
            self.format_pots(self.left()..self.right())
        )
    }
}

impl Pots {
    pub fn from_string(s: &str) -> Pots {
        let mut lines = s.lines();
        let pots = Pots::parse_first_line(lines.next().unwrap());
        let mut inst = BTreeMap::default();
        assert_eq!(lines.next().unwrap(), "");
        for l in lines {
            let lb: &[u8] = l.as_ref();
            let mut bs = [false; 5];
            for i in 0..5 {
                bs[i] = from_b(lb[i]);
            }
            let br = from_b(lb[9]);
            assert_eq!(inst.insert(bs, br), None, "key {bs:?} already present");
        }
        Pots {
            pots,
            inst: Rc::new(inst),
        }
    }

    fn parse_first_line(s: &str) -> BTreeSet<isize> {
        let (prefix, bs) = s.split_at(15);
        assert_eq!(prefix, "initial state: ");
        let mut pots = BTreeSet::new();
        for (i, c) in bs.bytes().map(from_b).enumerate() {
            if c {
                pots.insert(i as isize);
            }
        }
        pots
    }

    pub fn set(&mut self, i: isize, b: bool) {
        if b {
            self.pots.insert(i);
        } else {
            self.pots.remove(&i);
        }
    }

    pub fn get(&self, i: isize) -> bool {
        self.pots.contains(&i)
    }

    /// Number of the highest pot that's set
    pub fn right(&self) -> isize {
        *self.pots.iter().next_back().unwrap()
    }

    /// Number of the lowest pot that's set
    pub fn left(&self) -> isize {
        *self.pots.iter().next().unwrap()
    }

    pub fn format_pots<R: IntoIterator<Item = isize>>(&self, r: R) -> String {
        let mut s = String::new();
        for i in r {
            s.push(if self.get(i) { '#' } else { '.' });
        }
        s
    }

    /// Return the values of the 5 pots around i
    fn around(&self, i: isize) -> [bool; 5] {
        let mut a = [false; 5];
        for (j, x) in a.iter_mut().enumerate() {
            *x = self.get(i + (j as isize) - 2);
        }
        a
    }

    /// Produce new pots for the next step
    pub fn step(&self) -> Pots {
        let mut pots = BTreeSet::new();
        for i in (self.left() - 3)..=(self.right() + 3) {
            let a = self.around(i);
            let n = self.inst.get(&a).unwrap_or(&false);
            if *n {
                pots.insert(i);
            }
        }
        Pots {
            pots,
            inst: self.inst.clone(),
        }
    }

    /// Return the sum of pot-numbers that have a plant.
    pub fn magic(&self) -> isize {
        self.pots.iter().sum()
    }

    /// True if self is `other` shifted one space to the right.
    pub fn is_right_shift(&self, other: &Pots) -> bool {
        if self.pots.len() != other.pots.len() {
            return false;
        }
        for i in self.pots.iter() {
            if !other.pots.contains(&(i - 1)) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let p = Pots::from_string(
            "\
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #",
        );
        println!("p = {p:?}");

        assert_eq!(
            p.format_pots(-3..36),
            "...#..#.#..##......###...###..........."
        );
        assert!(!p.get(20));
        assert!(p.get(22));
        assert_eq!(p.around(0), [false, false, true, false, false]);
        assert_eq!(p.around(2), [true, false, false, true, false]);

        let mut p1 = p.step();
        assert_eq!(
            p1.format_pots(-3..36),
            "...#...#....#.....#..#..#..#..........."
        );

        for _i in 2..=20 {
            p1 = p1.step();
        }
        assert_eq!(
            p1.format_pots(-3..36),
            ".#....##....#####...#######....#.#..##."
        );

        assert_eq!(p1.magic(), 325);
    }

    #[test]
    fn stable() {
        let p = Pots::from_string("\
initial state: #.##...#.##.##....#.##...#.##.##...#.##.##.##.##.##.##.##.##...#.##...#.##.##.##.##.##.##....#.##...#.##...#.##.##...#.##...#.##.##.##.##.##.##.#

#..#. => #
.###. => .
..##. => .
....# => .
#...# => .
.#.#. => .
#.#.# => #
#.... => .
#.#.. => #
###.# => .
.#... => #
#.### => .
.#.## => #
..#.. => #
.#### => .
..### => #
...#. => .
##.#. => #
##.## => #
.##.# => #
###.. => .
..#.# => .
...## => #
##... => #
##### => .
#.##. => .
.#..# => #
##..# => .
..... => .
####. => #
#..## => .
.##.. => #
");
        println!("{:?}", p.pots);
        let p1 = p.step();
        println!("{:?}", p1.pots);
        let p1 = p1.step();
        println!("{:?}", p1.pots);
    }
}
