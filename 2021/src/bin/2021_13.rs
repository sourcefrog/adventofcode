// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/13

#[allow(unused_imports)]
use std::collections::BTreeMap;

use aoclib::{point, Matrix, Point};

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{a}");
    println!("{b}");
}

fn input() -> String {
    std::fs::read_to_string("input/13.txt").unwrap()
}

fn solve(input: &str) -> (usize, String) {
    let ps: Vec<Point> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| l.parse::<Point>().unwrap())
        .collect();
    let mut m = Matrix::bounding_box(ps.iter(), false);
    for p in &ps {
        m[*p] = true;
    }
    let mut n = Matrix::new(655, m.height(), false);
    for (p, &v) in m.point_values() {
        if v {
            if p.x < 655 {
                n[p] = true;
            } else {
                n[point(2 * 655 - p.x, p.y)] = true;
            }
        }
    }
    let sol_a = n.values().filter(|&&v| v).count();

    for l in input.lines() {
        if let Some(l) = l.strip_prefix("fold along ") {
            if let Some(sx) = l.strip_prefix("x=").map(|x| x.parse::<isize>().unwrap()) {
                let mut n = Matrix::new(sx as usize, m.height(), false);
                for (p, &v) in m.point_values() {
                    if v {
                        if p.x < sx {
                            n[p] = true;
                        } else {
                            n[point(2 * sx - p.x, p.y)] = true;
                        }
                    }
                }
                m = n;
            } else if let Some(sy) = l.strip_prefix("y=").map(|y| y.parse::<isize>().unwrap()) {
                let mut n = Matrix::new(m.width(), sy as usize, false);
                for (p, &v) in m.point_values() {
                    if v {
                        if p.y < sy {
                            n[p] = true;
                        } else {
                            n[point(p.x, 2 * sy - p.y)] = true;
                        }
                    }
                }
                m = n;
            }
        }
    }
    let sol_b = m.map(|&v| if v { '#' } else { '.' }).to_string();

    (sol_a, sol_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let input = input();
        let expect_b = "\
####.####.#....####...##..##..###..####.
#....#....#....#.......#.#..#.#..#.#....
###..###..#....###.....#.#....#..#.###..
#....#....#....#.......#.#.##.###..#....
#....#....#....#....#..#.#..#.#.#..#....
####.#....####.#.....##...###.#..#.#....
";
        let (a, b) = solve(&input);
        assert_eq!(a, 631);
        assert_eq!(b, expect_b);
    }
}
