use std::collections::HashMap;
use std::fs::read_to_string;

use itertools::Itertools;

static YEAR: &str = "2023";
static DAY: &str = "08";

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
    let mut lines = input.lines();
    let mut turns = lines
        .next()
        .expect("List of turns")
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("{c:?}"),
        })
        .cycle();
    lines.next();
    let mut nodes: HashMap<&str, [&str; 2]> = HashMap::new();
    for l in lines {
        let here = &l[0..3];
        check_name(here);
        let left = &l[7..10];
        check_name(left);
        let right = &l[12..15];
        check_name(right);
        assert!(
            nodes.insert(here, [left, right]).is_none(),
            "node {here:?} occurred twice?"
        )
    }
    let mut pos = "AAA";
    let mut steps = 0;
    while pos != "ZZZ" {
        pos = nodes[pos][turns.next().unwrap()];
        steps += 1;
    }
    steps
}

fn check_name(s: &str) {
    debug_assert!(s.len() == 3, "{s:?}");
    debug_assert!(s.chars().all(|c| c.is_ascii_uppercase()), "{s:?}");
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
        assert_eq!(solve_a(&input()), 14893);
    }

    #[test]
    fn solution_b() {
        // assert_eq!(solve_b(&input()), 13114317);
    }
}
