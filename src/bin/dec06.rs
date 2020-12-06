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

//! Solution to https://adventofcode.com/2020/day/6.
use std::collections::HashSet;

use itertools::Itertools;

pub fn main() {
    println!("06a: {}", solve_a());
    println!("06b: {}", solve_b());
}

fn solve_a() -> usize {
    std::fs::read_to_string("input/dec06.txt")
        .unwrap()
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|s| s.chars())
                .flatten()
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

fn solve_b() -> usize {
    std::fs::read_to_string("input/dec06.txt")
        .unwrap()
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().collect::<HashSet<char>>())
                .fold1(|ref a, ref b| a & b)
                .expect("group had no lines")
                .len()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 7128);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 3640);
    }
}
