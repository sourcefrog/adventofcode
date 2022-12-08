//! https://adventofcode.com/2022/day/8

use aoclib::Matrix;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/08.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let mat = aoclib::Matrix::from_string_lines(input);
    let mat = mat.map(|c| c.to_digit(10));
    let mut vis = Matrix::same_size(&mat, false);
    for row in 0..mat.height() {
        let mut h = None;
        for col in 0..mat.width() {
            let c = mat[(col, row)];
            if h.map(|h| c > h).unwrap_or(true) {
                vis[(col, row)] = true;
                h = Some(c);
            }
        }
        let mut h = None;
        for col in (0..mat.width()).rev() {
            let c = mat[(col, row)];
            if h.map(|h| c > h).unwrap_or(true) {
                vis[(col, row)] = true;
                h = Some(c);
            }
        }
    }
    for col in 0..mat.width() {
        let mut h = None;
        for row in 0..mat.height() {
            let c = mat[(col, row)];
            if h.map(|h| c > h).unwrap_or(true) {
                vis[(col, row)] = true;
                h = Some(c);
            }
        }
        let mut h = None;
        for row in (0..mat.height()).rev() {
            let c = mat[(col, row)];
            if h.map(|h| c > h).unwrap_or(true) {
                vis[(col, row)] = true;
                h = Some(c);
            }
        }
    }
    println!("{}", vis.to_string_lines());
    vis.values().filter(|x| **x).count()
}

fn solve_b(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 10466);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 10466);
    }
}
