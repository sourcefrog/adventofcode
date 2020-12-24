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

//! Solve https://adventofcode.com/2020/day/24.

use fnv::FnvHashSet;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

/// Hex coordinates {x, y} where x increases by 2 for horizontally adjacent
/// hexes.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
struct Hex {
    x: i16,
    y: i16,
}

/// The state of the game, represented as the set of black hexes.
type HexMap = FnvHashSet<Hex>;

pub fn main() {
    println!("24a: {}", solve_a());
    println!("24b: {}", solve_b());
}

fn solve_a() -> usize {
    solve_type_a(&load())
}

fn solve_type_a(s: &str) -> usize {
    let black = load_map(s);
    black.len()
}

fn load_map(s: &str) -> HexMap {
    let mut black: HexMap = Default::default();
    for tokens in parse(s) {
        let h = tokens_to_hex(&tokens);
        if !black.insert(h.clone()) {
            black.remove(&h);
        }
    }
    black
}

fn tokens_to_hex(tokens: &[&str]) -> Hex {
    let mut x = 0;
    let mut y = 0;
    for d in tokens {
        match *d {
            "e" => x += 2,
            "w" => x -= 2,
            "ne" => {
                y += 1;
                x += 1
            }
            "nw" => {
                y += 1;
                x -= 1
            }
            "sw" => {
                y -= 1;
                x -= 1
            }
            "se" => {
                y -= 1;
                x += 1
            }
            _other => panic!(),
        }
    }
    Hex { x, y }
}

fn parse(s: &str) -> Vec<Vec<&str>> {
    try_parse(s).unwrap().1
}

fn try_parse(s: &str) -> IResult<&str, Vec<Vec<&str>>> {
    many1(terminated(
        many1(alt((
            tag("e"),
            tag("w"),
            tag("nw"),
            tag("ne"),
            tag("sw"),
            tag("se"),
        ))),
        newline,
    ))(s)
}

fn neighbors(Hex { x, y }: &Hex) -> [Hex; 6] {
    [
        Hex { x: x + 2, y: *y },
        Hex { x: x - 2, y: *y },
        Hex { x: x + 1, y: y + 1 },
        Hex { x: x + 1, y: y - 1 },
        Hex { x: x - 1, y: y + 1 },
        Hex { x: x - 1, y: y - 1 },
    ]
}

fn solve_b() -> usize {
    solve_type_b(&load())
}

fn solve_type_b(s: &str) -> usize {
    let mut black = load_map(s);
    let mut newblack: HexMap = Default::default();
    let mut interest: HexMap = Default::default();
    for _day in 0..100 {
        for h in &black {
            for hh in &neighbors(h) {
                interest.insert(hh.clone());
            }
        }
        interest.extend(black.iter().cloned());
        for h in &interest {
            // take(3) because we don't care about results higher than 3
            let bns = neighbors(&h)
                .iter()
                .filter(|n| black.contains(n))
                .take(3)
                .count();
            let newstate = if black.contains(&h) {
                !(bns == 0 || bns > 2)
            } else {
                bns == 2
            };
            if newstate {
                let added = newblack.insert(h.clone());
                debug_assert!(added, "{:?} somehow already present", h);
            }
        }
        std::mem::swap(&mut black, &mut newblack);
        newblack.clear();
        interest.clear();
        // println!("day {}: {}", day, black.len());
    }
    black.len()
}

fn load() -> String {
    std::fs::read_to_string("input/dec24.txt").unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 244);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 3665);
    }
}
