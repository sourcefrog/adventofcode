fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    input.len()
}

fn solve_b(input: &str) -> usize {
    input.len()
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
