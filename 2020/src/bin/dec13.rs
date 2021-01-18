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

//! Solve https://adventofcode.com/2020/day/13.

pub fn main() {
    println!("13a: {}", solve_a());
    println!("13b: {}", solve_b());
}

fn solve_a() -> usize {
    let inp = std::fs::read_to_string("input/dec13.txt").unwrap();
    let mut l = inp.lines();
    let dt: usize = l.next().unwrap().parse().unwrap();
    let buses: Vec<usize> = l
        .next()
        .unwrap()
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut best_bus = 0;
    let mut best_wait = usize::MAX;
    for &b in &buses {
        if dt % b == 0 {
            return 0;
        }
        let earliest = (dt / b + 1) * b;
        let wait = earliest - dt;
        if wait < best_wait {
            best_wait = wait;
            best_bus = b;
        }
    }
    best_wait * best_bus
}

fn solve_b() -> isize {
    let inp = std::fs::read_to_string("input/dec13.txt").unwrap();
    solve_type_b(&inp.lines().skip(1).next().unwrap())
}

fn solve_type_b(inp: &str) -> isize {
    let mut gaps = Vec::new();
    let mut cycles: Vec<isize> = Vec::new();
    for (i, s) in inp.split(',').enumerate().filter(|(_i, s)| *s != "x") {
        gaps.push(i as isize);
        cycles.push(s.parse().unwrap());
    }
    let mut ans = 0;
    // This assumes all the cycles are all coprime (which they are in the input)
    // -- in fact, they are all simply prime, a big clue.
    // Any future coincidence must therefore be on a multiple of the product of
    // all the cycles considered so far.
    let mut step = 1;
    for (i, c) in cycles.iter().enumerate() {
        loop {
            if (ans + gaps[i]) % c == 0 {
                // println!("ans={} for i={}, c={}", ans, i, c);
                step *= c;
                break;
            }
            ans += step;
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 205);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 803025030761664);
    }

    #[test]
    fn examples_b() {
        assert_eq!(solve_type_b("17,x,13,19"), 3417);
        assert_eq!(solve_type_b("1789,37,47,1889"), 1202161486);
    }
}
