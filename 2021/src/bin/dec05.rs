// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/5

use aoclib::{point, Matrix, Point};

fn main() {
    let input = input();
    println!("{:?}", solve_a(&input));
    println!("{:?}", solve_b(&input));
}

fn input() -> String {
    std::fs::read_to_string("input/05.txt").unwrap()
}

fn parsept(s: &str) -> Point {
    let (x, y) = s.split_once(',').unwrap();
    point(x.parse().unwrap(), y.parse().unwrap())
}

fn solve_a(input: &str) -> usize {
    solve(input, false)
}

fn solve_b(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, include_diagonals: bool) -> usize {
    let mut lines: Vec<(Point, Point)> = Vec::new();
    for l in input.lines() {
        let (a, b) = l.split_once(" -> ").unwrap();
        lines.push((parsept(a), parsept(b)));
    }
    let max_x = lines.iter().flat_map(|(a, b)| [a.x, b.x]).max().unwrap();
    let max_y = lines.iter().flat_map(|(a, b)| [a.y, b.y]).max().unwrap();
    let mut mat = Matrix::new(max_x as usize + 1, max_y as usize + 1, 0usize);
    for (a, b) in lines {
        let xs = (b.x - a.x).signum();
        let ys = (b.y - a.y).signum();
        if !include_diagonals && (xs != 0) && (ys != 0) {
            continue;
        }
        let mut x = a.x;
        let mut y = a.y;
        loop {
            mat[(x, y)] += 1;
            if x == b.x && y == b.y {
                break;
            }
            x += xs;
            y += ys;
        }
    }
    mat.values().filter(|c| **c > 1).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(solve_a(&input()), 7085);
        assert_eq!(solve_b(&input()), 20271);
    }
}
