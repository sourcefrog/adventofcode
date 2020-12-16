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

//! Solution to https://adventofcode.com/2020/day/16.

use std::collections::HashSet;

use itertools::Itertools;

use adventofcode2020::*;

pub fn main() {
    println!("16a: {}", solve_a());
    println!("16b: {}", solve_b());
}

fn solve_a() -> usize {
    let input = std::fs::read_to_string("input/dec16.txt").unwrap();
    let mut lines = input.lines();
    let mut fields: Vec<(usize, usize, String)> = Vec::new();
    loop {
        let l = lines.next().unwrap();
        if l.is_empty() {
            break;
        }
        let (name, rest) = split_one(l, ':');
        // dbg!(name, rest);
        let mut words = rest[1..].split(' ');
        let range1 = words.next().unwrap();
        // dbg!(range1);
        let (a, b) = split_one(range1, '-');
        fields.push((a.parse().unwrap(), b.parse().unwrap(), name.to_owned()));
        assert_eq!(words.next().unwrap(), "or");
        let range2 = words.next().unwrap();
        let (a, b) = split_one(range2, '-');
        fields.push((a.parse().unwrap(), b.parse().unwrap(), name.to_owned()));
    }
    assert_eq!(lines.next().unwrap(), "your ticket:");
    lines.next().unwrap();
    lines.next().unwrap();

    assert_eq!(lines.next().unwrap(), "nearby tickets:");
    let mut sum_invalid = 0;
    for l in lines {
        for field in l.split(',').map(|w| w.parse::<usize>().unwrap()) {
            if !is_valid_field(field, &fields) {
                sum_invalid += field
            }
        }
    }
    sum_invalid
}

fn is_valid_field(field: usize, defns: &[(usize, usize, String)]) -> bool {
    for &(a, b, _) in defns {
        if field >= a && field <= b {
            return true;
        }
    }
    false
}

fn possible_fields(field: usize, defns: &[(usize, usize, String)]) -> HashSet<String> {
    let mut r = HashSet::new();
    for (a, b, name) in defns.iter() {
        if field >= *a && field <= *b {
            r.insert(name.clone());
        }
    }
    r
}

fn solve_b() -> usize {
    let input = std::fs::read_to_string("input/dec16.txt").unwrap();
    let mut lines = input.lines();
    let mut defns: Vec<(usize, usize, String)> = Vec::new();
    loop {
        let l = lines.next().unwrap();
        if l.is_empty() {
            break;
        }
        let (name, rest) = split_one(l, ':');
        // dbg!(name, rest);
        let mut words = rest[1..].split(' ');
        let range1 = words.next().unwrap();
        // dbg!(range1);
        let (a, b) = split_one(range1, '-');
        defns.push((a.parse().unwrap(), b.parse().unwrap(), name.to_owned()));
        assert_eq!(words.next().unwrap(), "or");
        let range2 = words.next().unwrap();
        let (a, b) = split_one(range2, '-');
        defns.push((a.parse().unwrap(), b.parse().unwrap(), name.to_owned()));
    }
    assert_eq!(lines.next().unwrap(), "your ticket:");
    let mytick: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|w| w.parse().unwrap())
        .collect();
    lines.next().unwrap();

    assert_eq!(lines.next().unwrap(), "nearby tickets:");
    let mut valid_tickets: Vec<Vec<usize>> = Vec::new();
    for l in lines {
        let tick: Vec<usize> = l.split(',').map(|w| w.parse::<usize>().unwrap()).collect();
        if tick.iter().all(|f| is_valid_field(*f, &defns)) {
            valid_tickets.push(tick)
        }
    }
    // dbg!(valid_tickets);

    // What possible meaning could each column have?
    let n_fields = valid_tickets[0].len();
    let mut narrowing: Vec<HashSet<String>> = Vec::new();
    for field_idx in 0..n_fields {
        for poss in valid_tickets
            .iter()
            .map(|t| t[field_idx])
            .map(|f| possible_fields(f, &defns))
            .fold1(|x, y| x.intersection(&y).cloned().collect())
        {
            // println!("{} => {:#?}", field_idx, poss);
            narrowing.push(poss);
        }
    }

    // Find one column with an unambiguous meaning; assign that; remove it as a possibility
    // for others, and iterate until everything is resolved.
    let mut known: Vec<Option<String>> = vec![None; n_fields];
    for _i in 0..n_fields {
        // find the one value that occurs once
        let (found_idx, found_set) = narrowing
            .iter()
            .enumerate()
            .find(|(_idx, hs)| hs.len() == 1)
            .unwrap();
        let found_name = found_set.iter().next().unwrap().clone();
        // println!("decided {} = {}", found_idx, found_name);
        known[found_idx] = Some(found_name.clone());
        for hs in narrowing.iter_mut() {
            hs.remove(&found_name);
        }
    }

    known
        .iter()
        .enumerate()
        .filter(|(_, s)| s.as_ref().unwrap().starts_with("departure"))
        .map(|(idx, _s)| mytick[idx])
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 26988);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 426362917709);
    }
}
