//! https://adventofcode.com/2016/day/3

const DAY: &str = "1603";

fn solve_type_a(s: &str) -> usize {
    s.lines()
        .filter(|l| {
            let s: Vec<usize> = l
                .trim()
                .split_ascii_whitespace()
                .map(|w| w.parse().expect("parse integer length"))
                .collect();
            is_possible_triangle(&s)
        })
        .count()
}

fn is_possible_triangle(s: &[usize]) -> bool {
    debug_assert_eq!(s.len(), 3);
    (s[0] + s[1] > s[2]) && (s[1] + s[2] > s[0]) && (s[0] + s[2] > s[1])
}

fn solve_type_b(input: &str) -> usize {
    let s: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|word| word.parse().expect("parse integer length"))
                .collect()
        })
        .collect();
    s.as_slice()
        .chunks(3)
        .flat_map(|ch| {
            (0..3).filter(move |&col| is_possible_triangle(&[ch[0][col], ch[1][col], ch[2][col]]))
        })
        .count()
}

fn input() -> String {
    std::fs::read_to_string(format!("input/{}.txt", DAY)).unwrap()
}

fn solve_a() -> usize {
    solve_type_a(&input())
}

fn solve_b() -> usize {
    solve_type_b(&input())
}

fn main() {
    println!("1603a: {}", solve_a());
    println!("1603b: {}", solve_b());
}

#[cfg(test)]
mod test1603 {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 1050);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 1921);
    }
}
