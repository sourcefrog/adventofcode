use std::fs::read_to_string;

fn main() {
    println!("2023_01 a {}", solve_a(&input()));
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
    fn example_1() {
        let input = indoc! { "\
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};
        assert_eq!(solve_a(input), 142);
    }
}
