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

pub fn main() {
    println!("14a: {}", solve_a());
    println!("14b: {}", solve_b());
}

fn solve_a() -> u64 {
    let mut maskbits: u64 = 0;
    let mut setbits: u64 = 0;
    let mut mem = vec![0u64; 100000];
    for l in std::fs::read_to_string("input/dec14.txt").unwrap().lines() {
        if let Some(mask) = l.strip_prefix("mask = ") {
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
        } else {
            let l = l.strip_prefix("mem[").unwrap();
            let mut parts = l.split("] = ");
            let addr: usize = parts.next().unwrap().parse().unwrap();
            let val: u64 = parts.next().unwrap().parse().unwrap();
            // dbg!(&addr, &val);
            mem[addr] = (val & !maskbits) | setbits;
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
            let combos = 1 << floatbits.len();
            for i in 0..combos {
                let mut thisaddr = addr;
                for (j, float) in floatbits.iter().enumerate() {
                    if i & (1 << j) != 0 {
                        thisaddr |= (1 << float)
                    } else {
                        thisaddr &= !(1 << float)
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
