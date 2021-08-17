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

//! Solve https://adventofcode.com/2020/day/23.

// use std::collections::BTreeSet;

const INPUT: &str = "562893147";
// const INPUT: &'static str = "389125467";

pub fn main() {
    println!("23a: {}", Ring::new(parse(INPUT)).solve_a());
    println!("23b: {}", Ring::solve_b());
}

struct Ring {
    n: usize,

    /// succ[i] is the number that comes after i in the ring,
    /// where i>=1
    succ: Vec<usize>,

    /// Number of the first card.
    first: usize,
}

impl Ring {
    fn new(v: Vec<usize>) -> Ring {
        let n = v.len();
        assert_eq!(*v.iter().max().unwrap(), n);
        let mut succ = vec![0; n + 1];
        for i in 0..n - 1 {
            succ[v[i]] = v[i + 1];
        }
        succ[v[n - 1]] = v[0];
        // dbg!(&succ);
        let r = Ring {
            n: v.len(),
            first: v[0],
            succ,
        };
        r.check_slowly();
        r
    }

    /// Remove and return the values of the 3 cup that come after the cup
    /// labeled c.
    fn unlink_3_after(&mut self, c: usize) -> Vec<usize> {
        let mut r = Vec::new();
        let mut prev = c;
        for _i in 0..3 {
            let next = self.succ[prev];
            r.push(next);
            prev = next;
        }
        self.succ[c] = self.succ[prev];
        r
    }

    fn insert_after(&mut self, c: usize, vals: Vec<usize>) {
        let mut prev = c;
        let follows = self.succ[c];
        for v in vals {
            self.succ[prev] = v;
            prev = v;
        }
        self.succ[prev] = follows;
    }

    fn wrap_sub(&self, a: usize, b: usize) -> usize {
        let r = if a > b { a - b } else { a + self.n - b };
        assert!(r >= 1 && r <= self.n);
        r
    }

    fn solve_a(&mut self) -> String {
        self.play(100);

        self.a_result()
    }

    fn play(&mut self, rounds: usize) {
        let mut current = self.first;
        self.check_slowly();

        for _round in 1..=rounds {
            debug_assert!(current > 0 && current <= self.n);
            let taken = self.unlink_3_after(current);
            // println!("taken {:?}", taken);
            let mut dest = current;
            loop {
                dest = self.wrap_sub(dest, 1);
                if !taken.iter().any(|x| *x == dest) {
                    break;
                }
            }
            assert!(dest > 0 && dest <= self.n);
            // println!("insert after {}: {:?}", dest, taken);
            self.insert_after(dest, taken);
            // It's too slow to check during many iterations.
            // self.check_slowly();
            current = self.succ[current];
        }
    }

    fn a_result(&self) -> String {
        let mut r = String::new();
        let mut a = self.succ[1];
        while a != 1 {
            r.push(std::char::from_digit(a as u32, 10).unwrap());
            a = self.succ[a];
        }
        r
    }

    #[cfg(not(debug_assertions))]
    fn check_slowly(&self) {}

    #[cfg(debug_assertions)]
    fn check_slowly(&self) {
        let mut seen = vec![false; self.n + 1];
        debug_assert_eq!(self.n + 1, self.succ.len());
        assert_eq!(self.succ[0], 0);
        assert_eq!(*self.succ[1..].iter().min().unwrap(), 1);
        assert_eq!(*self.succ[1..].iter().max().unwrap(), self.n);
        // Clippy suggests `enumerate` but with one index into two parallel arrays
        // just looping on the index seems simpler.
        #[allow(clippy::needless_range_loop)]
        for i in 1..=self.n {
            assert_ne!(self.succ[i], i);
            assert_ne!(self.succ[i], 0);
            assert!(!seen[i]);
            seen[i] = true;
        }
        assert!(seen[1..].iter().all(|x| *x));
    }

    fn solve_b() -> usize {
        let mut v = parse(INPUT);
        const CUPS: usize = 1_000_000;
        v.extend((v.len() + 1..=CUPS).into_iter());
        assert_eq!(v.len(), CUPS);
        let mut ring = Ring::new(v);
        ring.play(10_000_000);

        let a = ring.succ[1];
        let b = ring.succ[a];
        a * b
    }
}

fn parse(s: &str) -> Vec<usize> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(Ring::new(parse(INPUT)).solve_a(), "38925764");
    }

    #[test]
    fn solution_b() {
        assert_eq!(Ring::solve_b(), 131152940564);
    }
}
