// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/16

#![allow(clippy::comparison_chain)] // bad warning; it's slower and no simpler

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/16.txt").unwrap()
}

fn solve(_input: &str) -> (i32, u64) {
    // target area: x=70..125, y=-159..-121
    use std::cmp::max;
    let mut besty = 0;
    for ovx in 0..1000 {
        for ovy in 0i32..1000 {
            let mut vx = ovx;
            let mut vy = ovy;
            let mut x = 0;
            let mut y = 0;
            let mut maxy = 0;
            for _step in 0.. {
                x += vx;
                y += vy;
                maxy = max(maxy, y);
                if vx > 0 {
                    vx -= 1;
                } else if vx < 0 {
                    vx += 1;
                }
                vy -= 1;
                if x > 125 || y < -159 {
                    break;
                } else if x >= 70 && y <= -121 {
                    besty = max(besty, maxy);
                    break;
                }
            }
        }
    }
    let sol_a = besty;

    let mut sol_b = 0;
    for ovx in -1000..1000 {
        for ovy in -1000i32..1000 {
            let mut vx = ovx;
            let mut vy = ovy;
            let mut x = 0;
            let mut y = 0;
            for _step in 0.. {
                x += vx;
                y += vy;
                if vx > 0 {
                    vx -= 1;
                } else if vx < 0 {
                    vx += 1;
                }
                vy -= 1;
                if x > 125 || y < -159 {
                    break;
                } else if x >= 70 && y <= -121 {
                    sol_b += 1;
                    break;
                }
            }
        }
    }

    (sol_a, sol_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 12561);
        assert_eq!(b, 3785);
    }
}
