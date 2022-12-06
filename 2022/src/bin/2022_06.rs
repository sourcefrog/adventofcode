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
    let mut i = 0;
    for w in input.chars().collect::<Vec<char>>().windows(4) {
        let s: HashSet<char> = HashSet::from_iter(w.iter().cloned());
        i += 1;
        if s.len() == 4 {
            break;
        }
    }
    i + 3
}

fn solve_b(input: &str) -> usize {
    let mut i = 0;
    let ln = 14;
    for w in input.chars().collect::<Vec<char>>().windows(ln) {
        let s: HashSet<char> = HashSet::from_iter(w.iter().cloned());
        if s.len() == ln {
            return i + ln;
        }
        i += 1;
    }
    // not 4106
    unreachable!();
    // i + 14 - 1
}

// fn find_nonrepeating(input:&str, ln: usize) -> usize {

// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exZ() {
        assert_eq!(solve_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7)
    }

    // #[test]
    // fn solution_a() {
    //     assert_eq!(solve_a(&input()), 0);
    // }

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(&input()), 0);
    // }
}
