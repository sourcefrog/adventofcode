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
    while !(decks[0].is_empty() || decks[1].is_empty()) {
        let draw = [decks[0].remove(0), decks[1].remove(0)];
        let winner = (draw[1] > draw[0]) as usize;
        decks[winner].push(draw[winner]);
        decks[winner].push(draw[1 - winner]);
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

struct Game {
    decks: [Vec<usize>; 2],
    prev_states: BTreeSet<[Vec<usize>; 2]>,
}

impl Game {
    fn new(decks: [Vec<usize>; 2]) -> Game {
        Game {
            decks,
            prev_states: Default::default(),
        }
    }

    fn play_game(&mut self) -> usize {
        let winner: usize = loop {
            if self.prev_states.contains(&self.decks) {
                break 0;
            } else {
                self.prev_states.insert(self.decks.clone());
            }

            // println!("decks {:?}", self.decks);

            if self.decks[0].is_empty() {
                break 1;
            } else if self.decks[1].is_empty() {
                break 0;
            }

            let draw = [self.decks[0].remove(0), self.decks[1].remove(0)];
            // println!("draw {}, {}", draw[0], draw[1]);

            let winner: usize = if draw[0] <= self.decks[0].len() && draw[1] <= self.decks[1].len()
            {
                let sub_decks: [Vec<usize>; 2] = [
                    self.decks[0].iter().take(draw[0]).cloned().collect(),
                    self.decks[1].iter().take(draw[1]).cloned().collect(),
                ];
                println!("recurse down");
                Game::new(sub_decks).play_game()
            } else {
                (draw[1] > draw[0]) as usize
            };

            // println!("{} wins", winner);
            self.decks[winner].push(draw[winner]);
            self.decks[winner].push(draw[1 - winner]);
        };
        winner
    }
}

fn solve_b() -> usize {
    let orig_decks = parse(&load());
    let mut top_game = Game {
        decks: orig_decks.clone(),
        prev_states: Default::default(),
    };

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
