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

//! Solve https://adventofcode.com/2020/day/9.

pub fn main() {
    println!("09a: {}", solve_a());
    println!("09b: {}", solve_b());
}

fn solve_a() -> usize {
    let v = load();
    for i in 25..v.len() {
        if !sum_is_in(v[i], &v[(i - 25)..i]) {
            return v[i];
        }
    }
    unreachable!()
}

fn sum_is_in(s: usize, p: &[usize]) -> bool {
    for i in 0..(p.len() - 1) {
        for j in (i + 1)..(p.len()) {
            if p[i] + p[j] == s {
                return true;
            }
        }
    }
    false
}

fn solve_b() -> usize {
    let target = solve_a();
    let v = load();
    for i in 0..(v.len() - 1) {
        let mut sum = v[i];
        for j in (i + 1)..(v.len()) {
            sum += v[j];
            let range = &v[i..=j];
            if target == sum {
                return range.iter().min().unwrap() + range.iter().max().unwrap();
            }
        }
    }
    unreachable!()
}

fn load() -> Vec<usize> {
    std::fs::read_to_string("input/dec09.txt")
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
        assert_eq!(solve_a(), 167829540);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 28045630);
    }
}
