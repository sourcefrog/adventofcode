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
    // 0 represents 'not seen yet'; n means 'seen at position n-1'
    // The largest possible value is n.
    let mut pos: Vec<usize> = vec![0; n];
    let mut a: usize;
    for (i, &a) in input[..input.len() - 1].iter().enumerate() {
        pos[a] = i + 1;
    }
    a = *input.last().unwrap();
    for i in input.len() - 1.. {
        let next_a = match pos[a] {
            0 => 0,
            l => i - (l - 1),
        };
        pos[a] = i + 1;
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
