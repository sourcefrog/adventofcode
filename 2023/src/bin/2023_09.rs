use std::fs::read_to_string;

use itertools::Itertools;

static PUZZLE: &str = env!("CARGO_BIN_NAME");

fn main() {
    let input = &input();
    println!("{PUZZLE} a {}", solve_a(input));
    println!("{PUZZLE} b {}", solve_b(input));
}

fn input() -> String {
    let (year, day) = PUZZLE.split_once('_').unwrap();
    read_to_string(format!("{year}/input/{day}.txt"))
        .or_else(|_| read_to_string(format!("input/{day}.txt")))
        .unwrap()
}

/// Given a row of increasing numbers, find the differences repeatedly,
/// then sum up to find the next number on the top row.
fn solve_row_a(a: &[isize]) -> isize {
    // l is the sum of the last element of every row on the way down.
    let mut a = a.to_vec();
    let mut l = *a.last().unwrap();
    loop {
        dbg!(&a, l);
        if a.iter().all(|x| *x == a[0]) {
            return l;
        } else {
            let new_a = a
                .iter()
                .tuple_windows()
                .map(|(a, b)| b.checked_sub(*a).unwrap())
                .collect_vec();
            assert_eq!(new_a.len(), a.len() - 1);
            a = new_a;
            l += *a.last().unwrap();
        }
    }
}

fn solve_a(input: &str) -> isize {
    parse(input).into_iter().map(|l| solve_row_a(&l)).sum()
}

fn solve_b(input: &str) -> isize {
    0
}

fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            dbg!(l);
            l.split_ascii_whitespace()
                .map(|w| w.parse::<isize>().expect("parse number"))
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"\
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn example_1() {
        assert_eq!(
            solve_row_a(&parse(EXAMPLE.lines().take(1).next().unwrap())[0]),
            18
        );
        assert_eq!(solve_a(EXAMPLE), 114);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 1789635132);
    }

    #[test]
    fn solution_b() {
        // assert_eq!(solve_b(&input()), 13114317);
    }
}
