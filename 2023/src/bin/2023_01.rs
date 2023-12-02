use std::fs::read_to_string;

#[allow(unused_imports)]
use itertools::Itertools;

static NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = &input();
    println!("2023_01 a {}", solve_a(input));
    println!("2023_01 b {}", solve_b(input));
    // let bugs = find_bugs(input).collect_vec();
    // if !bugs.is_empty() {
    //     println!("bugs in first attempt:");
    //     bugs.iter().for_each(|b| println!("{b:?}"))
    // }
}

fn solve_a(input: &str) -> u32 {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let digits = l.chars().filter_map(|c| c.to_digit(10));
            digits.clone().next().expect("Has a first digit") * 10
                + digits.last().expect("Has a last digit")
        })
        .sum()
}

fn solve_b(input: &str) -> u32 {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse)
        .map(|(first, last)| first * 10 + last)
        .sum()
}

/// Parse a line containing both digits and English words for numbers, ignoring unrecognized characters.
fn parse(s: &str) -> (u32, u32) {
    let mut first = None;
    let mut last = None;
    for i in 0..s.len() {
        if let Some(a) = match_number(&s[i..]) {
            first.get_or_insert(a);
            last = Some(a);
        }
    }
    let first = first.expect("Found a first");
    let last = last.expect("Found a last");
    (first, last)
}

/// Find what was wrong with my first attempt.
///
/// Returns lines that parse differently: the difference is that the words can
/// overlap and we should look at all of them.
#[allow(dead_code)]
fn find_bugs(s: &str) -> impl Iterator<Item = (&str, [u32; 4])> {
    s.lines().filter_map(|line| {
        let (first, last) = parse(line);
        let old_parse = parse_old(line);
        let old_first = old_parse[0];
        let old_last = *old_parse.last().expect("old_parse has a last");
        if first != old_first || last != old_last {
            Some((line, [first, last, old_first, old_last]))
        } else {
            None
        }
    })
}

/// Parse a line containing both digits and English words for numbers, ignoring unrecognized characters.
fn parse_old(mut s: &str) -> Vec<u32> {
    let mut v = Vec::new();
    's: while let Some(c) = s.chars().next() {
        if let Some(a) = c.to_digit(10) {
            v.push(a);
        }
        for (a, numstr) in NUMBERS.iter().enumerate().skip(1) {
            // Zero isn't valid!
            if let Some(news) = s.strip_prefix(numstr) {
                v.push(a as u32);
                s = news;
                continue 's;
            }
        }
        if s.len() > 1 {
            s = s.split_at(1).1;
        } else {
            break;
        }
    }
    v
}

fn match_number(s: &str) -> Option<u32> {
    if let Some(a) = s.chars().next().unwrap().to_digit(10) {
        return Some(a);
    }
    for (a, numstr) in NUMBERS.iter().enumerate().skip(1) {
        // Zero isn't valid!
        if s.starts_with(numstr) {
            return Some(a as u32);
        }
    }
    None
}

fn input() -> String {
    read_to_string("input/01.txt")
        .or_else(|_| read_to_string("2023/input/01.txt"))
        .unwrap()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 54573);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 54591);
    }

    #[test]
    fn example_1() {
        let input = indoc! { "\
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};
        assert_eq!(solve_a(input), 142);
    }

    #[test]
    fn example_2() {
        let input = indoc! { "\
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};
        assert_eq!(solve_b(input), 281);
    }

    #[test]
    fn parse_old_examples() {
        for (line, expected) in [
            ("veight37", &[8u32, 3, 7] as &[u32]),
            ("fiveeight2zxjpzffvdsevenjhjvjfiveone", &[5, 8, 2, 7, 5, 1]),
            ("7b", &[7]),
        ] {
            assert_eq!(parse_old(line), expected);
        }
    }
}
