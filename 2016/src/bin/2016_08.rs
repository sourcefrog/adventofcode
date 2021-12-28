//! https://adventofcode.com/2016/day/8

use std::fmt;
use std::ops::Range;

const DAY: &str = "1608";

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

const XRANGE: Range<usize> = 0..WIDTH;
const YRANGE: Range<usize> = 0..HEIGHT;

#[derive()]
struct Matrix {
    // Y going top to bottom; X left to right
    bits: Vec<bool>,
}

impl Matrix {
    fn offset(x: usize, y: usize) -> usize {
        assert!((XRANGE).contains(&x));
        assert!((0..HEIGHT).contains(&y));
        x + y * WIDTH
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.bits[Matrix::offset(x, y)]
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        self.bits[Matrix::offset(x, y)] = val
    }

    fn set_top_left_rect(&mut self, w: usize, h: usize) {
        for x in 0..w {
            for y in 0..h {
                self.set(x, y, true)
            }
        }
    }

    fn rotate_row(&mut self, y: usize, b: usize) {
        let tmp: Vec<bool> = (XRANGE).map(|x| self.get(x, y)).collect();
        for x in XRANGE {
            self.set((x + b) % WIDTH, y, tmp[x])
        }
    }

    fn rotate_column(&mut self, x: usize, b: usize) {
        let tmp: Vec<bool> = (YRANGE).map(|y| self.get(x, y)).collect();
        for y in YRANGE {
            self.set(x, (y + b) % HEIGHT, tmp[y]);
        }
    }

    fn count_lit(&self) -> usize {
        self.bits.iter().filter(|b| **b).count()
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Matrix {
            bits: vec![false; WIDTH * HEIGHT],
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in YRANGE {
            for x in XRANGE {
                write!(f, "{}", if self.get(x, y) { 'X' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn run(input: &str) -> Matrix {
    let mut mat = Matrix::default();
    for l in input.lines().map(str::trim) {
        // println!("{}", l);
        if let Some(params) = l.strip_prefix("rect ") {
            let (xs, ys) = params.split_once('x').unwrap();
            mat.set_top_left_rect(xs.parse().unwrap(), ys.parse().unwrap());
        } else if let Some(params) = l.strip_prefix("rotate row y=") {
            let (ys, bs) = params.split_once(" by ").unwrap();
            mat.rotate_row(ys.parse().unwrap(), bs.parse().unwrap());
        } else if let Some(params) = l.strip_prefix("rotate column x=") {
            let (xs, bs) = params.split_once(" by ").unwrap();
            mat.rotate_column(xs.parse().unwrap(), bs.parse().unwrap());
        } else {
            panic!("can't parse {:#?}", l)
        }
        // println!("{}", mat);
    }
    mat
}

fn solve_type_a(input: &str) -> usize {
    run(input).count_lit()
}

fn solve_type_b(input: &str) -> String {
    run(input).to_string()
}

fn input() -> String {
    std::fs::read_to_string(&format!("input/{}.txt", DAY)).unwrap()
}

fn solve_a() -> usize {
    solve_type_a(&input())
}

fn solve_b() -> String {
    solve_type_b(&input())
}

fn main() {
    println!("{}a: {}", DAY, solve_a());
    println!("{}b:\n{}", DAY, solve_b());
}

#[cfg(test)]
mod test1608 {
    use super::*;

    #[test]
    fn example() {}

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 115);
        // not EFEYKRFIJ
    }

    #[test]
    fn solution_b() {
        assert_eq!(
            solve_b(),
            "\
XXXX.XXXX.XXXX.X...XX..X.XXXX.XXX..XXXX..XXX...XX.
X....X....X....X...XX.X..X....X..X.X......X.....X.
XXX..XXX..XXX...X.X.XX...XXX..X..X.XXX....X.....X.
X....X....X......X..X.X..X....XXX..X......X.....X.
X....X....X......X..X.X..X....X.X..X......X..X..X.
XXXX.X....XXXX...X..X..X.X....X..X.X.....XXX..XX..
"
        );
    }
}
