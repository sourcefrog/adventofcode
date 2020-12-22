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

use std::collections::VecDeque;

pub fn main() {
    println!("22a: {}", solve_a());
    println!("22b: {}", solve_b());
}

fn solve_a() -> usize {
    let mut decks = parse(&load());
    while !(decks[0].is_empty() || decks[1].is_empty()) {
        let draw = [decks[0].pop_front().unwrap(), decks[1].pop_front().unwrap()];
        let winner = (draw[1] > draw[0]) as usize;
        decks[winner].push_back(draw[winner]);
        decks[winner].push_back(draw[1 - winner]);
    }
    let win = if decks[0].is_empty() {
        &decks[1]
    } else {
        &decks[0]
    };
    win.iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * *c)
        .sum()
}

fn solve_b() -> isize {
    0
}

fn load() -> String {
    std::fs::read_to_string("input/dec22.txt").unwrap()
}

fn parse(s: &str) -> [VecDeque<usize>; 2] {
    let mut r = [VecDeque::new(), VecDeque::new()];
    let mut p = 0;
    for l in s.lines() {
        match l {
            "Player 1:" | "" => (),
            "Player 2:" => p = 1,
            num => {
                let n = num.parse::<usize>().unwrap();
                r[p].push_back(n);
            }
        }
    }
    r
}

#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn solution_a() {}

    #[test]
    fn solution_b() {}
}
