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

//! Solution to https://adventofcode.com/2020/day/7.
use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;

// use itertools::Itertools;

const START: &'static str = "shiny gold";

pub fn main() {
    println!("07a: {}", solve_a());
    println!("07b: {}", solve_b());
}

#[derive(Debug)]
struct Rule {
    container: String,
    contains: Vec<(usize, String)>,
}

fn solve_a() -> usize {
    let mut ans = HashSet::new();
    let mut queue: Vec<String> = Vec::new();
    let rules = load();
    queue.push(START.to_owned());
    while let Some(n) = queue.pop() {
        for rule in rules
            .iter()
            .filter(|rule| rule.contains.iter().any(|(_, bag)| *bag == n))
        {
            if ans.insert(rule.container.clone()) {
                queue.push(rule.container.clone())
            }
        }
    }
    ans.len()
}

fn load() -> Vec<Rule> {
    let mut v = Vec::new();
    let outer_re = Regex::new(r"([a-z ]+) bags contain (.*)\.").unwrap();
    let inner_re = Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();

    for l in std::fs::read_to_string("input/dec07.txt").unwrap().lines() {
        let groups = outer_re.captures(l).unwrap();
        let container = groups.get(1).unwrap().as_str().to_owned();
        let mut contains = Vec::new();

        let b = groups.get(2).unwrap().as_str();
        if b != "no other bags" {
            for j in b.split(", ") {
                let caps = inner_re.captures(j).unwrap();
                let n: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                let name = caps.get(2).unwrap().as_str().to_owned();
                contains.push((n, name));
            }
        }
        v.push(Rule {
            container,
            contains,
        })
    }
    v
}

fn solve_b() -> usize {
    // let rules = load();
    let map: HashMap<String, Vec<(usize, String)>> = load()
        .into_iter()
        .map(|rule| (rule.container, rule.contains))
        .collect();
    let mut total: HashMap<String, usize> = HashMap::new();
    let mut queue = vec![(1, START.to_owned())];
    while let Some((nn, nname)) = queue.pop() {
        for (cn, cname) in &map[&nname] {
            queue.push((cn * nn, cname.clone()));
            *total.entry(cname.clone()).or_default() += cn * nn;
        }
    }
    total.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 257);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 1038);
    }
}
