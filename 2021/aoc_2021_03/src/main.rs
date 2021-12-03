//! https://adventofcode.com/2021/day/3

use aoclib::Matrix;

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
    let mut gamma = 0;
    let mut epsilon = 0;
    for ones_in_col in m.columns().map(|col| col.filter(|&&bit| bit).count()) {
        gamma <<= 1;
        epsilon <<= 1;
        if ones_in_col > (m.height() - ones_in_col) {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }
    gamma * epsilon
}

/// Count the number of 1s and 0s in column `col` of the selected rows of the given matrix.
fn count(m: &Matrix<bool>, rows: &[usize], col: usize) -> (usize, usize) {
    let ones = rows.iter().filter(|row| m[(col, **row)]).count();
    (ones, rows.len() - ones)
}

fn from_base2(matrix: &Matrix<bool>, row: usize) -> usize {
    matrix
        .row(row)
        .fold(0, |acc, &bit| (acc << 1) | (bit as usize))
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
        oxy_cands.retain(|&row| m[(col, row)] == crit);
        if oxy_cands.len() == 1 {
            break;
        }
    }
    assert_eq!(oxy_cands.len(), 1);
    let oxy_rating = from_base2(&m, oxy_cands[0]);

    for col in 0..n {
        let (ones, zeroes) = count(&m, &co2_cands, col);
        let crit = ones < zeroes;
        co2_cands.retain(|&row| m[(col, row)] == crit);
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
