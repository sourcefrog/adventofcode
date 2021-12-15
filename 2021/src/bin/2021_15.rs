// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/15

#![allow(unused_imports)]
use std::collections::BTreeMap;

use itertools::Itertools;

use aoclib::{Matrix, Point,point};

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/15.txt").unwrap()
}

fn solve(input: &str) -> (u32,u32) {
    let m = Matrix::from_string_lines(input).map(
        |c| c.to_digit(10).unwrap());
    // The lowest known total risk to get to each cell.
    let mut best = Matrix::same_size(&m, u32::MAX);
    best[(0usize,0usize)] = 0;
    let mut active: Vec<Point> = vec![point(0,0)];
    // while let Some(p) = active.pop() {
    //     let prisk = best[p];
    //     for (q, &qrisk) in m.neighbors4(p) {
    //         // dbg!(q, qrisk);
    //         if prisk + qrisk < best[q] {
    //             best[q] = prisk + qrisk;
    //             active.push(q);
    //         }
    //     }
    // }
    let sol_a = best[(best.width()-1, best.height()-1)];

    let mut m2 = Matrix::new(m.width() * 5, m.height() * 5, u32::MAX);
    for (p, &v) in m.point_values() {
        for mx in 0usize..5 {
            for my in 0usize..5 {
                let mut u = v + mx as u32 + my as u32;
                while u > 9 { 
                    u-=9;
                }
                m2[(mx * m.width() + p.x as usize, my*m.height() + p.y as usize)] = u;
            }
        }
    }
    let mut best = Matrix::same_size(&m2, u32::MAX);
    best[(0usize,0usize)] = 0;
    let mut active = std::collections::BinaryHeap::new();
    active.push((0i32, point(0,0)));
    while let Some((neg_prisk,p)) = active.pop() {
        let prisk = (-neg_prisk) as u32;
        for (q, &qrisk) in m2.neighbors4(p) {
            // dbg!(q, qrisk);
            let tot = prisk + qrisk;
            if tot < best[q] {
                best[q] = tot;
                dbg!(best[q]);
                active.push((-(tot as i32), q));
            }
        }
    }
    dbg!(best.width(),best.height());
    let sol_b = best[(best.width()-1, best.height()-1)];
    // 4294967295 is wrong

    (sol_a,sol_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        // assert_eq!(solve(EX).0, 1588);
    }

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        // assert_eq!(a, 2194);
        // assert_eq!(b, 2360298895777);
    }
}
