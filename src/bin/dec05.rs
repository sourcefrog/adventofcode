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

//! Solution to https://adventofcode.com/2020/day/5.

pub fn main() {
    println!("04a: {}", solve_a());
    println!("04b: {}", solve_b());
}

fn solve_a() -> usize {
    *all_ids().iter().max().unwrap()
}

fn all_ids() -> Vec<usize> {
    std::fs::read_to_string("input/dec05.txt")
        .unwrap()
        .lines()
        .map(|l| decode(l))
        .collect()
}

fn decode(l: &str) -> usize {
    assert_eq!(l.len(), 10);
    let mut row = 0;
    for c in l.chars().take(7) {
        row *= 2;
        if c == 'B' {
            row |= 1;
        }
    }
    let mut col = 0;
    for c in l.chars().skip(7) {
        col *= 2;
        if c == 'R' {
            col |= 1;
        }
    }
    row * 8 + col
}

fn solve_b() -> usize {
    let ids = &all_ids();
    for i in ids {
        if ids.contains(&(i + 2)) && !&ids.contains(&(i + 1)) {
            return i + 1;
        }
    }
    panic!()
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
