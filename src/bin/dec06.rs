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

pub fn main() {
    println!("06a: {}", solve_a());
    println!("06b: {}", solve_b());
}

fn solve_a() -> usize {
    let mut total = 0;
    for group in std::fs::read_to_string("input/dec06.txt")
        .unwrap()
        .split("\n\n")
    {
        total += group
            .split('\n')
            .map(|s| s.chars())
            .flatten()
            .collect::<HashSet<char>>()
            .len()
    }
    total
}

fn solve_b() -> usize {
    let mut total = 0;

    for group in std::fs::read_to_string("input/dec06.txt")
        .unwrap()
        .split("\n\n")
    {
        let mut common: Option<HashSet<char>> = None;
        for line in group.split('\n').filter(|l| !l.is_empty()) {
            if let Some(ref c) = common {
                common = Some(c & &line.chars().collect());
            } else {
                common = Some(line.chars().collect());
            }
        }
        total += common.unwrap().len();
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 919);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 642);
    }
}
