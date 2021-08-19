//! https://adventofcode.com/2016/day/3

fn solve_type_a(s: &str) -> usize {
    s.lines()
        .filter(|l| {
            let s: Vec<usize> = l
                .trim()
                .split_ascii_whitespace()
                .map(|w| w.parse().expect("parse integer length"))
                .collect();
            debug_assert_eq!(s.len(), 3);
            (s[0] + s[1] > s[2]) && (s[1] + s[2] > s[0]) && (s[0] + s[2] > s[1])
        })
        .count()
}

fn solve_type_b(s: &str) -> String {
    todo!()
}

fn input() -> String {
    std::fs::read_to_string("input/1603.txt").unwrap()
}

fn solve_a() -> usize {
    solve_type_a(&input())
}

fn solve_b() -> String {
    solve_type_b(&input())
}

fn main() {
    println!("1602a: {}", solve_a());
    // println!("1602b: {}", solve_b());
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn example_a() {
    //     assert_eq!(solve_type_a(EXAMPLE), "1985");
    // }

    // #[test]
    // fn example_b() {
    //     assert_eq!(solve_type_b(EXAMPLE), "5DB3");
    // }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 1050);
    }

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(), "7423A");
    // }
}
