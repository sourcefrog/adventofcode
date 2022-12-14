//! https://adventofcode.com/2022/day/14

use std::cmp::{max, min};

use aoclib::{point, Matrix, Point};

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/14.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let mut mat = Matrix::new(600, 600, '.');
    for l in input.lines() {
        let mut last: Option<Point> = None;
        for ps in l.split(" -> ") {
            let (x, y) = ps.split_once(',').unwrap();
            let x: isize = x.parse().unwrap();
            let y: isize = y.parse().unwrap();
            let p = point(x, y);
            if let Some(last) = last {
                if last.x == x {
                    for iy in min(last.y, y)..=max(last.y, y) {
                        mat[point(x, iy)] = '#';
                    }
                } else if last.y == y {
                    for ix in min(last.x, x)..=max(last.x, x) {
                        mat[point(ix, y)] = '#';
                    }
                } else {
                    panic!()
                }
            }
            last = Some(p);
        }
    }
    let tap = point(500, 0);
    's: while mat[tap] == '.' {
        let mut sp = tap;
        'q: loop {
            if (sp.y + 1) >= mat.height() as isize {
                // fell off the bottom
                break 's;
            } else if mat[sp.down()] == '.' {
                sp = sp.down();
            } else if mat[sp.down().left()] == '.' {
                sp = sp.down().left();
            } else if mat[sp.down().right()] == '.' {
                sp = sp.down().right();
            } else {
                // can't move
                assert_eq!(mat[sp], '.');
                println!("fill {sp:?}");
                mat[sp] = 'o';
                continue 's;
            }
        }
    }

    mat.values().filter(|c| **c == 'o').count()
}

fn solve_b(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 9900);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 9900);
    }
}
