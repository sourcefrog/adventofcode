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

// use std::collections::BTreeMap;
use std::collections::BTreeSet;
// use std::collections::VecDeque;

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

struct Game {
    decks: [Vec<usize>; 2],
}

impl Game {
    fn new(decks: [Vec<usize>; 2]) -> Game {
        Game { decks }
    }

    fn play_game(&mut self) -> usize {
        let mut prev_states: BTreeSet<[Vec<usize>; 2]> = BTreeSet::new();
        loop {
            // println!("decks {:?}", self.decks);
            if !prev_states.insert(self.decks.clone()) {
                // already present
                return 0;
            }
            if self.decks[0].is_empty() {
                return 1;
            } else if self.decks[1].is_empty() {
                return 0;
            }

            let draw = [self.decks[0].remove(0), self.decks[1].remove(0)];
            // println!("draw {}, {}", draw[0], draw[1]);

            let rw: usize = if draw[0] <= self.decks[0].len() && draw[1] <= self.decks[1].len() {
                let sub_decks: [Vec<usize>; 2] = [
                    self.decks[0].iter().take(draw[0]).cloned().collect(),
                    self.decks[1].iter().take(draw[1]).cloned().collect(),
                ];
                // println!("recurse down");
                Game::new(sub_decks).play_game()
            } else {
                (draw[1] > draw[0]) as usize
            };
            self.decks[rw].push(draw[rw]);
            self.decks[rw].push(draw[1 - rw]);
        }
    }
}

fn solve_b() -> usize {
    let mut top_game = Game::new(parse(&load()));
    let winner = top_game.play_game();
    top_game.decks[winner]
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
        let decks = parse(
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
        let mut game = Game::new(decks);
        assert_eq!(game.play_game(), 1);
        assert_eq!(game.decks[0], vec![]);
        assert_eq!(game.decks[1], vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3]);
    }
}
