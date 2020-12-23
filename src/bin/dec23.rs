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

#![allow(unused_imports, dead_code, unused_mut)]

use std::collections::BTreeSet;

const INPUT: &'static str = "562893147";
// const INPUT: &'static str = "389125467";

pub fn main() {
    println!("23a: {}", Ring::from_str(INPUT).play_a());
    // println!("23b: {}", Ring::from_str(INPUT).solve_b());
}

struct Ring {
    v: Vec<usize>,
    n: usize,
}

impl Ring {
    fn new(v: Vec<usize>) -> Ring {
        Ring { n: v.len(), v }
    }

    fn remove_3(&mut self, pos: usize) -> Vec<usize> {
        let mut r = Vec::new();
        let mut rmpos = pos + 1;
        for i in 0..3 {
            if rmpos < self.v.len() {
                r.push(self.v.remove(rmpos));
            } else {
                r.push(self.v.remove(0));
            }
        }
        r
    }

    fn insert_after(&mut self, pos: usize, vals: Vec<usize>) {
        if pos + 1 >= self.v.len() {
            self.v.extend_from_slice(&vals)
        } else {
            for (i, v) in vals.iter().enumerate() {
                self.v.insert(pos + 1 + i, *v)
            }
        }
    }

    fn from_str(s: &str) -> Ring {
        Ring::new(
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        )
    }

    fn position(&self, a: usize) -> Option<usize> {
        self.v.iter().position(|x| *x == a)
    }

    fn wrap_sub(&self, a: usize, b: usize) -> usize {
        let r = if a > b { a - b } else { a + self.n - b };
        assert!(r >= 1 && r <= self.n);
        r
    }

    // fn wrap_add(&self, a: usize, b: usize) -> usize {
    //     let mut r = a + b;
    //     if r > self.n {
    //         r -= self.n
    //     }
    //     assert!(r >= 1 && r <= self.n);
    //     r
    // }

    fn play_a(&mut self) -> String {
        let mut current = 0;
        for mv in 1..=100 {
            println!("move {}", mv);
            let current_val = self.v[current];
            println!(
                "cups: {:?}, current value {} in position {}",
                self.v, current_val, current,
            );
            let taken = self.remove_3(current);
            println!("taken {:?}", taken);
            let mut dest_pos: Option<usize> = None;
            for i in 1..self.n {
                let dest_val = self.wrap_sub(current_val, i);
                dbg!(dest_val);
                dest_pos = self.position(dest_val);
                if dest_pos.is_some() {
                    assert_eq!(self.v[dest_pos.unwrap()], dest_val);
                    println!(
                        "dest val {} is position {} in {:?}",
                        dest_val,
                        dest_pos.unwrap(),
                        self.v
                    );
                    break;
                }
            }
            self.insert_after(dest_pos.unwrap(), taken);
            debug_assert_eq!(self.v.len(), self.n);
            self.check();
            current = (self.position(current_val).unwrap() + 1) % self.n;
            println!();
        }
        println!("final: {:?}", self.v);
        self.a_result()
        // 78459236 is wrong
    }

    fn a_result(&self) -> String {
        let mut r = String::new();
        let one_pos = self.v.iter().position(|x| *x == 1).unwrap();
        for i in 1..self.n {
            r.push(std::char::from_digit(self.v[(one_pos + i) % self.n] as u32, 10).unwrap())
        }
        r
    }

    fn check(&self) {
        for i in 1..=self.n {
            debug_assert!(
                self.v.iter().find(|x| **x == i).is_some(),
                "couldn't find {} in {:?}",
                i,
                self.v
            );
        }
    }

    fn solve_b(&mut self) -> usize {
        self.v.extend(self.n..=1_000_000);
        assert_eq!(self.v.len(), 1_000_000);
        self.n = self.v.len();
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(Ring::from_str(INPUT).play_a(), "38925764");
    }

    #[test]
    fn solution_b() {
        // assert_eq!(solve_b(), 0);
    }
}
