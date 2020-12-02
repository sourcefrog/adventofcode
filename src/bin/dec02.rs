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
            let a = p.password.chars().filter(|c| *c == p.char).count();
            a >= p.a && a <= p.b
        })
        .count()
}

fn solve_b() -> usize {
    parse(&load_input())
        .iter()
        .filter(|p| {
            let ca = p.password.chars().nth(p.a - 1).unwrap();
            let cb = p.password.chars().nth(p.b - 1).unwrap();
            // Valid if exactly one position is this character
            (p.char == ca) != (p.char == cb)
        })
        .count()
}

#[derive(Debug)]
struct Password {
    a: usize,
    b: usize,
    char: char,
    password: String,
}

fn load_input() -> String {
    std::fs::read_to_string("input/dec02.txt").unwrap()
}

fn parse(s: &str) -> Vec<Password> {
    let re = regex::Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let mut v = Vec::new();
    for line in s.lines() {
        let caps = re.captures(line).expect("failed to parse");
        let p = Password {
            a: caps.get(1).unwrap().as_str().parse().unwrap(),
            b: caps.get(2).unwrap().as_str().parse().unwrap(),
            char: caps.get(3).unwrap().as_str().chars().next().unwrap(),
            password: caps.get(4).unwrap().as_str().to_owned(),
        };
        v.push(p);
    }
    v
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
