//! https://adventofcode.com/2022/day/6

use std::collections::HashSet;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/06.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    find_nonrepeating(input, 4)
}

fn solve_b(input: &str) -> usize {
    find_nonrepeating(input, 14)
}

fn find_nonrepeating(input: &str, ln: usize) -> usize {
    // We could potentially here keep a vec of counts of each character
    // and just increment and decrement counts as letters move in and
    // out of this window.
    //
    // It would in theory be faster and avoid building a set every time, but
    // it would definitely be more code, and this is already very fast on
    // this input.
    for (i, w) in input.chars().collect::<Vec<char>>().windows(ln).enumerate() {
        let s: HashSet<char> = w.iter().cloned().collect();
        if s.len() == ln {
            return i + ln;
        }
    }
    unreachable!("no nonrepeating sequence found");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7)
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 1034);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 2472);
    }
}
