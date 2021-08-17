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
// use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;

const START: &str = "shiny gold";

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
    // let outer_re = Regex::new(r"([a-z ]+) bags contain (.*)\.").unwrap();
    // let inner_re = Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();

    std::fs::read_to_string("input/dec07.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let mut splits = l.split(" bags contain ");
            let container = splits.next().unwrap().to_owned();
            let mut contains = Vec::new();
            let b = splits.next().unwrap();
            if b != "no other bags." {
                contains = b
                    .split(", ")
                    .map(|j| {
                        let mut words = j.split(' ');
                        let n = words.next().unwrap().parse().unwrap();
                        let name = words.take(2).collect::<Vec<_>>().join(" ");
                        (n, name)
                    })
                    .collect();
            }
            Rule {
                container,
                contains,
            }
        })
        .collect()
}

fn solve_b() -> usize {
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
