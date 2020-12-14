use std::collections::HashMap;

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

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::{IResult, Parser};

pub fn main() {
    println!("14a: {}", solve_a());
    println!("14b: {}", solve_b());
}

enum Instruction {
    Mask(String),
    Store(u64, u64),
}
use Instruction::*;

fn parse_input(s: &str) -> IResult<&str, Vec<Instruction>> {
    all_consuming(many1(terminated(
        alt((parse_mask, parse_store)),
        multispace1,
    )))(s)
}

fn parse_mask(s: &str) -> IResult<&str, Instruction> {
    preceded(
        tag("mask = "),
        recognize(many0(one_of("01X"))).map(|s: &str| Instruction::Mask(s.to_owned())),
    )(s)
}

fn parse_store(s: &str) -> IResult<&str, Instruction> {
    map(
        pair(
            delimited(
                tag("mem["),
                map_res(digit1, |a: &str| a.parse()),
                tag("] = "),
            ),
            map_res(digit1, |v: &str| v.parse()),
        ),
        |(a, v)| Instruction::Store(a, v),
    )(s)
}

fn solve_a() -> u64 {
    let mut maskbits: u64 = 0;
    let mut setbits: u64 = 0;
    let mut mem = vec![0u64; 100000];
    for inst in parse_input(&std::fs::read_to_string("input/dec14.txt").unwrap())
        .unwrap()
        .1
        .into_iter()
    {
        match inst {
            Mask(mask) => {
                // dbg!(&mask);
                maskbits = 0;
                setbits = 0;
                for (i, c) in mask.chars().enumerate() {
                    let b = 1 << (mask.len() - i - 1);
                    if c != 'X' {
                        maskbits |= b;
                    }
                    if c == '1' {
                        setbits |= b;
                    }
                }
            }
            Store(addr, val) => {
                mem[addr as usize] = (val & !maskbits) | setbits;
            }
        }
    }
    mem.iter().sum()
}

fn solve_b() -> u64 {
    let mut floatbits: Vec<u64> = Vec::new();
    let mut setbits: u64 = 0;
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for l in std::fs::read_to_string("input/dec14.txt").unwrap().lines() {
        if let Some(mask) = l.strip_prefix("mask = ") {
            floatbits.clear();
            setbits = 0;
            for (i, c) in mask.chars().enumerate() {
                let b = 1 << (mask.len() - i - 1);
                match c {
                    'X' => floatbits.push(b),
                    '1' => setbits |= b,
                    _ => (),
                }
            }
        } else {
            let l = l.strip_prefix("mem[").unwrap();
            let mut parts = l.split("] = ");
            let mut addr: u64 = parts.next().unwrap().parse().unwrap();
            let val: u64 = parts.next().unwrap().parse().unwrap();
            addr |= setbits;
            for i in 0..(1 << floatbits.len()) {
                let mut thisaddr = addr;
                for (j, float) in floatbits.iter().enumerate() {
                    if i & (1 << j) != 0 {
                        thisaddr |= float;
                    } else {
                        thisaddr &= !float;
                    }
                }
                mem.insert(thisaddr, val);
            }
        }
    }
    mem.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 11327140210986);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 2308180581795);
    }
}
