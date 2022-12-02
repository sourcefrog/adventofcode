//! https://adventofcode.com/2022/day/1

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/01.txt").unwrap()
}

fn load(input: &str) -> Vec<usize> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| line.is_empty())
        .map(|group| group.iter().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect()
}

fn solve_a(input: &str) -> usize {
    *load(input).iter().max().unwrap()
}

fn solve_b(input: &str) -> usize {
    let mut sums = load(input);
    sums.sort();
    sums.iter().rev().take(3).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 65912);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 195625);
    }
}
