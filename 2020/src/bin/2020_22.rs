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

//! Solve https://adventofcode.com/2020/day/22.

use std::collections::BTreeSet;

pub fn main() {
    println!("22a: {}", solve_a());
    println!("22b: {}", solve_b());
}

fn solve_a() -> usize {
    let mut decks = parse(&load());
    let winner = loop {
        if decks[0].is_empty() {
            break 1;
        }
        if decks[1].is_empty() {
            break 0;
        }
        let draw = [decks[0].remove(0), decks[1].remove(0)];
        let rw = (draw[1] > draw[0]) as usize;
        decks[rw].push(draw[rw]);
        decks[rw].push(draw[1 - rw]);
    };
    decks[winner]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * *c)
        .sum()
}

fn play_recursive(decks: &mut [Vec<usize>; 2]) -> usize {
    let mut prev_states: BTreeSet<[Vec<usize>; 2]> = BTreeSet::new();
    loop {
        // println!("decks {:?}", decks);
        // Interestingly, it's much faster just to clone and insert it, than
        // to check before inserting. I guess the tree walking is much cheaper
        // than the copying.
        if !prev_states.insert(decks.clone()) {
            return 0;
        }
        if decks[0].is_empty() {
            return 1;
        } else if decks[1].is_empty() {
            return 0;
        }

        let draw = [decks[0].remove(0), decks[1].remove(0)];
        // println!("draw {}, {}", draw[0], draw[1]);

        let rw = if draw[0] <= decks[0].len() && draw[1] <= decks[1].len() {
            let mut sub_decks: [Vec<usize>; 2] =
                [decks[0][..draw[0]].to_vec(), decks[1][..draw[1]].to_vec()];
            // println!("recurse down");
            play_recursive(&mut sub_decks)
        } else {
            (draw[1] > draw[0]) as usize
        };
        decks[rw].push(draw[rw]);
        decks[rw].push(draw[1 - rw]);
    }
}

fn solve_b() -> usize {
    let mut decks = parse(&load());
    let winner = play_recursive(&mut decks);
    decks[winner]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * *c)
        .sum()
}

fn load() -> String {
    std::fs::read_to_string("input/dec22.txt").unwrap()
}

fn parse(s: &str) -> [Vec<usize>; 2] {
    let mut r = [Vec::new(), Vec::new()];
    let mut p = 0;
    for l in s.lines() {
        match l.trim() {
            "Player 1:" | "" => (),
            "Player 2:" => p = 1,
            num => {
                let n = num.parse::<usize>().unwrap();
                r[p].push(n);
            }
        }
    }
    r
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 31308);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 33647);
    }

    #[test]
    fn example_b() {
        let mut decks = parse(
            "\
        Player 1:
        9
        2
        6
        3
        1

        Player 2:
        5
        8
        4
        7
        10
        ",
        );

        assert_eq!(play_recursive(&mut decks), 1);
        assert_eq!(decks[0], vec![]);
        assert_eq!(decks[1], vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3]);
    }
}
