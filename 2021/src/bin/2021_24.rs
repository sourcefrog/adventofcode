// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/24

#![allow(unused_imports)]
use std::cmp::max;
use std::collections::BTreeMap;
use std::convert::TryInto;

use similar;

use aoclib::{point, Matrix, Point};
use itertools::Itertools;

fn main() {
    let (a, b) = solve(&input());
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/24.txt").unwrap()
}

type Regs = [isize; 4];

fn regidx(name: &str) -> usize {
    let c = name.chars().next().unwrap();
    assert!(c >= 'w' && c <= 'z', "invalid reg {:?}", name);
    (c as u32 - 'w' as u32) as usize
}

fn run(program: &[&str], mut regs: Regs, input: &mut Vec<isize>) -> Regs {
    for l in program {
        // println!("{}", l);
        let w: Vec<&str> = l.split_whitespace().collect();
        if w[0] == "inp" {
            regs[regidx(w[1])] = input.remove(0)
        } else {
            let a = regidx(w[1]);
            let bname = w[2];
            let bval: isize = bname
                .parse::<isize>()
                .unwrap_or_else(|_| regs[regidx(bname)]);
            match w[0] {
                "mul" => regs[a] *= bval,
                "add" => regs[a] += bval,
                "div" => regs[a] /= bval,
                "mod" => regs[a] %= bval,
                "eql" => regs[a] = (regs[a] == bval) as isize,
                _ => panic!("unimplemented {:?}", l),
            }
        }
    }
    regs
}

fn solve(input: &str) -> (usize, u64) {
    let lines: Vec<&str> = input.lines().collect();
    let chunks: Vec<_> = lines.chunks(18).collect();
    for (i, c) in chunks.iter().enumerate() {
        println!("** chunk {i}");
        assert_eq!(c[0], "inp w");
        assert_eq!(c[1], "mul x 0");
        assert_eq!(c[2], "add x z");
        assert_eq!(c[3], "mod x 26");
        let div_z: isize = c[4].strip_prefix("div z ").unwrap().parse().unwrap();
        let add_x: isize = c[5].strip_prefix("add x ").unwrap().parse().unwrap();
        assert_eq!(c[6], "eql x w");
        assert_eq!(c[7], "eql x 0");
        assert_eq!(c[8], "mul y 0");
        assert_eq!(c[9], "add y 25");
        assert_eq!(c[10], "mul y x");
        assert_eq!(c[11], "add y 1");
        assert_eq!(c[12], "mul z y");
        assert_eq!(c[13], "mul y 0");
        assert_eq!(c[14], "add y w");
        let add_y: isize = c[15].strip_prefix("add y ").unwrap().parse().unwrap();
        assert_eq!(c[16], "mul y x");
        assert_eq!(c[17], "add z y");

        dbg!(div_z, add_x, add_y);
        // for input_digit in (1..=9).rev() {
        //     let regs = run(&c, [0; 4], &mut vec![input_digit]);
        //     println!("input digit {} -> {:?}", input_digit, regs);
        // }
        // break;

        // x = z % 26 + add_x
        // z /= div_z
        // x = (x != input)
        // y = if x 26 else 1
        // z *= y
        // y = input + add_y
        // if x { z += y }
        //
        // To keep z at 0 we want x = 0 so that z is multiplied by 1 and then 0 is added.
        // Is it just all 9s?
        //
        // We can't keep it all at 0 all the way along, because the
        // available inputs aren't enough to match the necessary values.
        //
        // But could we work out what values will produce an 0 from the last
        // module? We only carry across state in z. So we need to find an
        // input and a penultimate z that give the right result?
        //
        // So we can choose any z for the second-last round that generates
        // the maximum input. But... eventually z has to be 0 at the start.
        //
        // Can we do some kind of DFS for valid inputs? It seems like that
        // would converge on visiting all the numbers, which is infeasible.
        //
        // Interestingly div_z is always 1 or 26. And add_y is always positive,
        // while add_x may be negative.
        //
        // or restating
        //
        // x = (z % 26 + add_x) != input
        // z /= div_z
        // if x {
        //   z *= 26
        //   z += input + add_y
        // }
        //
        // Because input and add_y are always positive, how can we end up with
        // z=0 in the final round? It seems like only if x=0. If x were !0,
        // if z was negative it would be too negative to come back to 0, since input+add_y are less
        // than 26. If z is positive or zero then it will end up positive.
        //
        // So we need a prev_z and add_x such that they add up to the input.
        // Add_x is -9; so input digit 9 works if prev_z is 18.
    }
    assert_eq!(chunks.len(), 14);

    println!("final round:");
    let mut prev = one_round(
        &chunks[13],
        &[St {
            digits: Vec::new(),
            z: 0,
        }],
    );
    dbg!(&prev);

    // println!("2nd last round:");
    // for iround in (0..=12).rev() {
    //     prev = one_round(&chunks[iround], &prev);
    //     println!("round {}, {} options", iround, prev.len());
    // }

    // for input in (1..=9).rev() {
    //     for goal in 10..=18 {
    //         for prev_z in 0..100 {
    //             let regs = run(&chunks[13], [0, 0, 0, prev_z], &mut vec![input]);
    //             if regs[3] == goal {
    //                 println!("input digit {}, prev_z = {prev_z} -> {:?}", input, regs);
    //             }
    //         }
    //     }
    // }

    // let mut input = vec![9; 14];
    // let regs = run(&lines, [0; 4], &mut input);
    // dbg!(&regs);

    let sol_a = 0;
    let sol_b = 0;

    (sol_a, sol_b)
}

#[derive(Debug)]
struct St {
    digits: Vec<isize>,
    z: isize,
}

// Given a program and the list of acceptable z outputs, return a list of all
// predecessor z values and the inputs that would work with them.
fn one_round(program: &[&str], goal_sts: &[St]) -> Vec<St> {
    let mut r = Vec::new();
    'input: for input in (1..=9).rev() {
        for st in goal_sts {
            for prev_z in 0..26 {
                let regs = run(&program, [0, 0, 0, prev_z], &mut vec![input]);
                if regs[3] == st.z {
                    // println!("input digit {}, prev_z = {prev_z} -> {:?}", input, regs);
                    let mut digits = vec![input];
                    digits.extend_from_slice(&st.digits);
                    r.push(St { digits, z: prev_z });
                }
            }
        }
    }
    r
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 0);
        assert_eq!(b, 0);
    }
}
