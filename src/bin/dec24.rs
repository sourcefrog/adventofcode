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

#![allow(dead_code, unused_imports)]

use std::collections::BTreeSet;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::{IResult, Parser};

pub fn main() {
    println!("24a: {}", solve_a());
    println!("24b: {}", solve_b());
}

fn solve_a() -> usize {
    solve_type_a(&load())
}

fn solve_type_a(s: &str) -> usize {
    let mut black: BTreeSet<(isize, isize)> = BTreeSet::new();
    for line in parse(s) {
        let coord = reduce(&line);
        if !black.insert(coord) {
            black.remove(&coord);
        }
    }
    black.len()
}

fn reduce(line: &[&str]) -> (isize, isize) {
    let mut x = 0;
    let mut y = 0;
    for d in line {
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
    (x, y)
}

fn countchar(s: &str, c: char) -> isize {
    s.chars().filter(|x| *x == c).count() as isize
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

fn solve_b() -> isize {
    0
}

fn load() -> String {
    std::fs::read_to_string("input/dec24.txt").unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {}

    #[test]
    fn solution_b() {}
}
