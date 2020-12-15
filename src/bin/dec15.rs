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

//! Solve https://adventofcode.com/2020/day/15.
//!
//! Produces https://oeis.org/A181391.

use std::collections::HashMap;

const INPUT: &'static [usize] = &[1, 2, 16, 19, 18, 0];

pub fn main() {
    println!("15a: {}", solve_a());
    println!("15b: {}", solve_b());
}

fn solve_a() -> usize {
    solve(INPUT, 2020)
}

fn solve_b() -> usize {
    solve(&INPUT, 30000000)
}

fn solve(input: &[usize], n: usize) -> usize {
    let mut last_pos: HashMap<usize, usize> = HashMap::new();
    let mut a: usize;
    for (i, &a) in input[..input.len() - 1].iter().enumerate() {
        // println!("{} {}", i, a);
        last_pos.insert(a, i);
    }
    a = *input.last().unwrap();
    // println!("~~~");
    for i in input.len() - 1.. {
        // println!("{} {}", i, a);
        let next_a = last_pos
            .get(&a)
            .and_then(|&j| {
                debug_assert!(j < i);
                Some(i - j)
            })
            .unwrap_or(0);
        last_pos.insert(a, i);
        if i == n - 1 {
            return a;
        }
        a = next_a;
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve(&[0, 3, 6], 10), 0);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 536);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 24065124);
    }
}
