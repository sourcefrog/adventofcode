//! https://adventofcode.com/2021/day/3

use aoclib::{point, Matrix};

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn to_matrix(input: &str) -> Matrix<bool> {
    input.lines().map(|l| l.chars().map(|c| c == '1')).collect()
}

fn solve_a(input: &str) -> usize {
    let m = to_matrix(input);
    let n = m.height();
    let cols = m.width();
    let mut ones = vec![0; cols];
    for point in m.iter_points() {
        if m[point] {
            ones[point.x as usize] += 1;
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for a in ones {
        gamma <<= 1;
        epsilon <<= 1;
        if a > (n - a) {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }
    gamma * epsilon
}

fn count(m: &Matrix<bool>, rows: &[usize], col: usize) -> (usize, usize) {
    let ones = rows
        .iter()
        .filter(|row| m[point(col as isize, **row as isize)])
        .count();
    (ones, rows.len() - ones)
}

fn from_base2(matrix: &Matrix<bool>, row: usize) -> usize {
    let mut x = 0;
    for col in 0..matrix.width() {
        let c = matrix[point(col as isize, row as isize)];
        x = (x << 1) | (c as usize);
    }
    x
}

fn solve_b(input: &str) -> usize {
    let m = to_matrix(input);
    let n = m.height();

    // Row indexes that are stil in play.
    let mut oxy_cands: Vec<usize> = (0..n).collect();
    let mut co2_cands: Vec<usize> = (0..n).collect();
    for col in 0..n {
        let (ones, zeroes) = count(&m, &oxy_cands, col);
        let crit = ones >= zeroes;
        oxy_cands.retain(|&row| m[point(col as isize, row as isize)] == crit);
        if oxy_cands.len() == 1 {
            break;
        }
    }
    assert_eq!(oxy_cands.len(), 1);
    let oxy_rating = from_base2(&m, oxy_cands[0]);

    for col in 0..n {
        let (ones, zeroes) = count(&m, &co2_cands, col);
        let crit = ones < zeroes;
        co2_cands.retain(|&row| m[point(col as isize, row as isize)] == crit);
        if co2_cands.len() == 1 {
            break;
        }
    }
    assert_eq!(co2_cands.len(), 1);
    let co2_rating = from_base2(&m, co2_cands[0]);
    oxy_rating * co2_rating
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 4191876);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 3414905);
    }
}
