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

use std::convert::TryInto;

use aoclib::split_one;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Op {
    Nop(isize),
    Jmp(isize),
    Acc(isize),
}
use Op::*;

enum Outcome {
    Hang(isize),
    Finish(isize),
}

pub fn main() {
    println!("08a: {}", solve_a());
    println!("08b: {}", solve_b());
}

fn solve_a() -> isize {
    solve_type_a(&input())
}

fn solve_type_a(ops: &[Op]) -> isize {
    match run(ops) {
        Outcome::Hang(x) => x,
        _ => panic!(),
    }
}

fn run(ops: &[Op]) -> Outcome {
    let mut acc: isize = 0;
    let mut seen = vec![false; ops.len()];
    let mut pc: usize = 0;
    loop {
        if pc == ops.len() {
            return Outcome::Finish(acc);
        } else if seen[pc] {
            return Outcome::Hang(acc);
        }
        seen[pc] = true;
        match ops[pc] {
            Nop(_) => pc += 1,
            Acc(i) => {
                acc += i;
                pc += 1;
            }
            Jmp(i) => {
                let newpc = (pc as isize) + i;
                pc = newpc.try_into().unwrap();
            }
        }
    }
}

fn solve_b() -> isize {
    solve_type_b(&input())
}

fn solve_type_b(ops: &[Op]) -> isize {
    for i in 0..ops.len() {
        let mut variant = ops.to_vec();
        variant[i] = match ops[i] {
            Jmp(a) => Nop(a),
            Nop(a) => Jmp(a),
            Acc(_a) => continue,
        };
        match run(&variant) {
            Outcome::Finish(x) => return x,
            Outcome::Hang(_) => continue,
        }
    }
    unreachable!();
}

fn input() -> Vec<Op> {
    parse(&std::fs::read_to_string("input/dec08.txt").unwrap())
}

fn parse(s: &str) -> Vec<Op> {
    s.lines()
        .map(|s| {
            let (opstr, arg) = split_one(s, ' ');
            let arg = arg.parse().unwrap();
            match opstr {
                "nop" => Op::Nop(arg),
                "acc" => Op::Acc(arg),
                "jmp" => Op::Jmp(arg),
                other => panic!("{:?}", other),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let example = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";
        assert_eq!(solve_type_a(&parse(example)), 5);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 1521);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 1016);
    }
}
