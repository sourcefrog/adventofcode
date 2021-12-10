// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/10 "Syntax scoring"
//!
//! The constants from part 1 are interesting because they're all
//! related:
//!
//!     3 * 19 = 57
//!     57 * 21 = 1197
//!     1197 * 21 = 25137

fn main() {
    let input = input();
    println!("{:?}", solve(&input));
}

fn input() -> String {
    std::fs::read_to_string("input/10.txt").unwrap()
}

fn solve(input: &str) -> (u32, usize) {
    let mut sol_a = 0;
    let mut bs: Vec<usize> = Vec::new();
    for l in input.lines() {
        let mut st: Vec<char> = Vec::new();
        let mut bad = 0;
        for c in l.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    st.push(c);
                    continue;
                }
                _ => (),
            }
            let b = st.pop().unwrap();
            bad = match c {
                ')' if b == '(' => continue,
                '}' if b == '{' => continue,
                ']' if b == '[' => continue,
                '>' if b == '<' => continue,
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!(),
            };
        }
        if bad != 0 {
            sol_a += bad;
            continue;
        }
        let mut z = 0;
        while let Some(c) = st.pop() {
            let x = match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            };
            z = z * 5 + x;
        }
        bs.push(z);
    }
    bs.sort_unstable();
    assert_eq!(bs.len() % 2, 1);
    let sol_b = bs[bs.len() / 2];
    (sol_a, sol_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let input = input();
        assert_eq!(solve(&input), (345441, 3235371166));
    }
}
