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

//! Solve https://adventofcode.com/2020/day/10.

pub fn main() {
    println!("10a: {}", solve_a());
    println!("10b: {}", solve_b());
}

fn solve_a() -> usize {
    let mut input = load();
    // add the final device
    input.push(input.iter().max().unwrap() + 3);
    input.sort_unstable();
    let mut last = 0;
    let mut steps = [0usize; 4];
    for j in input {
        let diff = j - last;
        debug_assert!((1..=3).contains(&diff));
        steps[diff] += 1;
        last = j;
    }
    steps[1] * steps[3]
}

fn solve_b() -> usize {
    let input = load();
    solve_type_b(input)
}

fn solve_type_b(mut jolt: Vec<usize>) -> usize {
    jolt.push(jolt.iter().max().unwrap() + 3);
    jolt.push(0);
    jolt.sort_unstable();

    // Number of paths leading to jolt[i]:
    let mut paths = vec![0; jolt.len()];
    paths[0] = 1; // only one path: we begin at 0
    for (i, v) in jolt.iter().enumerate().skip(1) {
        for j in 1..=3 {
            if i >= j && (v - jolt[i - j] <= 3) {
                paths[i] += paths[i - j];
            }
        }
        // println!("{} paths to {}", paths[i], v);
    }
    *paths.last().unwrap()
}

fn load() -> Vec<usize> {
    std::fs::read_to_string("input/dec10.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 2484);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 15790581481472);
    }

    #[test]
    fn b_1() {
        assert_eq!(solve_type_b(vec![1, 2]), 2);
    }

    #[test]
    fn b_example_1() {
        let example = "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4"
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        assert_eq!(solve_type_b(example), 8);
    }
}
