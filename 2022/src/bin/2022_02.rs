//! https://adventofcode.com/2022/day/2

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/02.txt").unwrap()
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
    fn solution_a() {
        assert_eq!(solve_a(&input()), 0);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 0);
    }
}
