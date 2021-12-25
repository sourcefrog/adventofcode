// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/25

use aoclib::{point, Matrix, Point};

fn main() {
    let input = input();
    let (a, ()) = solve(&input);
    println!("{}", a);
}

fn input() -> String {
    std::fs::read_to_string("input/25.txt").unwrap()
}

fn solve(input: &str) -> (usize, ()) {
    let mut mat: Matrix<char> = Matrix::from_string_lines(input);

    let mut step = 1;
    loop {
        let mut next = mat.clone();
        let mut any_moved = false;
        for (pos, &c) in mat.point_values() {
            let r = right(&pos, &mat);
            if c == '>' && mat[r] == '.' {
                next[pos] = '.';
                next[r] = '>';
                any_moved = true;
            }
        }
        mat = next;
        let mut next = mat.clone();
        for (pos, &c) in mat.point_values() {
            let r = down(&pos, &mat);
            if c == 'v' && mat[r] == '.' {
                next[pos] = '.';
                next[r] = c;
                any_moved = true;
            }
        }
        if !any_moved {
            break;
        }
        mat = next;
        step += 1;
    }

    (step, ())
}

fn down(p: &Point, mat: &Matrix<char>) -> Point {
    let q = p.down();
    if q.y >= mat.height() as isize {
        point(q.x, 0)
    } else {
        q
    }
}

fn right(p: &Point, mat: &Matrix<char>) -> Point {
    let q = p.right();
    if q.x >= mat.width() as isize {
        point(0, q.y)
    } else {
        q
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 530);
        assert_eq!(b, ());
    }
}
