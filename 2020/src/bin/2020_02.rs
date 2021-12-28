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

//! Solution to https://adventofcode.com/2020/day/2.

pub fn main() {
    println!("02a: {}", solve_a());
    println!("02b: {}", solve_b());
}

fn solve_a() -> usize {
    solve_type_a(&load_input())
}

fn solve_type_a(s: &str) -> usize {
    parse(s)
        .iter()
        .filter(|p| {
            let a = p.p.iter().filter(|c| **c == p.c).count();
            a >= p.a && a <= p.b
        })
        .count()
}

fn solve_b() -> usize {
    parse(&load_input())
        .iter()
        .filter(|p| {
            let ca = p.p[p.a - 1];
            let cb = p.p[p.b - 1];
            // Valid if exactly one position is this character
            (p.c == ca) != (p.c == cb)
        })
        .count()
}

#[derive(Debug)]
struct Password {
    a: usize,
    b: usize,
    c: char,
    p: Vec<char>,
}

fn load_input() -> String {
    std::fs::read_to_string("input/dec02.txt").unwrap()
}

fn parse(input: &str) -> Vec<Password> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut fields = line.split(&['-', ' ', ':'][..]);
        let a: usize = fields.next().unwrap().parse().unwrap();
        let b: usize = fields.next().unwrap().parse().unwrap();
        let c: char = fields.next().unwrap().chars().next().unwrap();
        assert_eq!(fields.next().unwrap(), "");
        let p = fields.next().unwrap().chars().collect();
        let p = Password { a, b, c, p };
        result.push(p);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let example_1 = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";
        assert_eq!(solve_type_a(example_1), 2);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 398);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 562);
    }
}
