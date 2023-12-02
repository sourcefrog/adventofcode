use std::fs::read_to_string;

static NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    println!("2023_01 a {}", solve_a(&input()));
    println!("2023_01 b {}", solve_b(&input()));
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
    (first.expect("Found a first"), last.expect("Found a last"))
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
    read_to_string("input/01.txt").unwrap()
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
}
