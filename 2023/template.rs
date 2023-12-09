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

fn solve_a(input: &str) -> usize {
    0
}

fn solve_b(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_2() {
        let input = "\
";
        assert_eq!(solve_b(input), 0);
    }

    #[test]
    fn solution_a() {
        // assert_eq!(solve_a(&input()), 24706);
    }

    #[test]
    fn solution_b() {
        // assert_eq!(solve_b(&input()), 13114317);
    }
}
