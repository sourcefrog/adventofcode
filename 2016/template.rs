//! https://adventofcode.com/2016/day/5

const DAY: &str = "1605";

fn solve_type_a(input: &str) -> usize {
    0
}

fn solve_type_b(input: &str) -> usize {
    0
}

fn input() -> String {
    std::fs::read_to_string(&format!("input/{}.txt", DAY)).unwrap()
}

fn solve_a() -> usize {
    solve_type_a(&input())
}

fn solve_b() -> usize {
    solve_type_b(&input())
}

fn main() {
    println!("{}a: {}", DAY, solve_a());
    println!("{}b: {}", DAY, solve_b());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 0);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 0);
    }
}
