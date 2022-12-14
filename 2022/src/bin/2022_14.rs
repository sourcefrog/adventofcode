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

fn load(input: &str) -> Matrix<char> {
    let mut draw: Vec<Vec<Point>> = Vec::new();
    for l in input.lines() {
        draw.push(Vec::new());
        for ps in l.split(" -> ") {
            let (x, y) = ps.split_once(',').unwrap();
            let x: isize = x.parse().unwrap();
            let y: isize = y.parse().unwrap();
            draw.last_mut().unwrap().push(point(x, y));
        }
    }
    let mut mat = Matrix::new(1200, 600, '.');
    for shape in draw {
        for pair in shape.windows(2) {
            if let [p1, p2] = pair {
                if p1.x == p2.x {
                    for iy in min(p1.y, p2.y)..=max(p1.y, p2.y) {
                        mat[point(p1.x, iy)] = '#';
                    }
                } else if p1.y == p2.y {
                    for ix in min(p1.x, p2.x)..=max(p1.x, p2.x) {
                        mat[point(ix, p1.y)] = '#';
                    }
                } else {
                    panic!()
                }
            } else {
                unreachable!()
            }
        }
    }
    mat
}

fn solve_a(input: &str) -> usize {
    let mut mat = load(input);
    let tap = point(500, 0);
    's: while mat[tap] == '.' {
        let mut sp = tap;
        loop {
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
    let mut mat = load(input);
    let mut bottom = mat.find_values(&'#').map(|p| p.y).max().unwrap();
    bottom += 1;
    let tap = point(500, 0);
    's: while mat[tap] == '.' {
        let mut sp = tap;
        loop {
            if sp.y == bottom {
                assert_eq!(mat[sp], '.');
                // println!("bottom at {sp:?}");
                mat[sp] = 'o';
                continue 's;
            } else if mat[sp.down()] == '.' {
                sp = sp.down();
            } else if mat[sp.down().left()] == '.' {
                sp = sp.down().left();
            } else if mat[sp.down().right()] == '.' {
                sp = sp.down().right();
            } else {
                // can't move
                assert_eq!(mat[sp], '.');
                // println!("fill {sp:?}");
                mat[sp] = 'o';
                continue 's;
            }
        }
    }
    mat.values().filter(|c| **c == 'o').count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 1133);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 27566);
    }
}
