use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    let input = &input();
    println!("2023_04 a {}", solve_a(input));
    println!("2023_04 b {}", solve_b(input));
}

fn input() -> String {
    read_to_string("2023/input/04.txt")
        .or_else(|_| read_to_string("input/04.txt"))
        .unwrap()
}

fn solve_a(input: &str) -> usize {
    let mut tot = 0;
    for l in input.lines() {
        let l = l.split_once(": ").unwrap().1;
        let (a, b) = l.split_once(" | ").unwrap();
        let a = a
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        let b = b
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        let hits = b.iter().filter(|x| a.contains(x)).count();
        if hits > 0 {
            tot += 1 << (hits - 1);
        }
    }
    tot
}

fn solve_b(input: &str) -> usize {
    let mut line_matches = vec![];
    let ncards = input.lines().count();
    for l in input.lines() {
        let l = l.split_once(": ").unwrap().1;
        let (a, b) = l.split_once(" | ").unwrap();
        let a = a
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        let b = b
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        let hits = b.iter().filter(|x| a.contains(x)).count();
        line_matches.push(hits);
    }
    let mut copies = vec![1; ncards];
    for (i, n) in line_matches.iter().enumerate() {
        for j in 1..(*n + 1) {
            if (i + j) < ncards {
                copies[i + j] += copies[i];
            }
        }
    }
    copies.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_2() {
        let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!(solve_b(input), 30);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 24706);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 13114317);
    }
}
