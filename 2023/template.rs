use std::fs::read_to_string;

use itertools::Itertools;

static YEAR: &str = "2023";
static DAY: &str = "";

fn main() {
    let input = &input();
    println!("{YEAR}_{DAY} a {}", solve_a(input));
    println!("{YEAR}_{DAY} b {}", solve_b(input));
}

fn input() -> String {
    read_to_string(format!("{YEAR}/input/{DAY}.txt"))
        .or_else(|_| read_to_string(format!("input/{DAY}.txt")))
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
