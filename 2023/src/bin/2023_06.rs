use std::fs::read_to_string;

use itertools::Itertools;

static YEAR: &str = "2023";
static DAY: &str = "06";

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

/// Parse into a list of (time, distance) pairs, for each race.
fn parse(input: &str) -> Vec<(usize, usize)> {
    let (times, distances) = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .map(|w| w.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();
    times.into_iter().zip_eq(distances).collect_vec()
}

fn solve_a(input: &str) -> usize {
    let mut tot = 1;
    for (time, distance) in parse(input) {
        let mut wins = 0;
        for speed in 1..time {
            let travelled = (time - speed) * speed;
            if travelled > distance {
                wins += 1
            }
        }
        tot *= wins;
    }
    tot
}

fn solve_b(input: &str) -> usize {
    let (time, distance) = parse_b(input);
    /* Distance travelled for any given speed is (time-speed) * speed, which is a
     * 2nd order polynomial in speed, time*speed - speed, an inverted parabola
     * above the goal distance.
     *
     * This is already tolerably fast but we could do better:
     *
     * 1. closed-form solution for the roots of a quadratic equation, with some
     *    adjustments for the roots being irrational but the solution working in
     *    integers
     *
     * 2. binary search for the lowest and highest values
     */
    let mut wins = 0;
    for speed in 1..time {
        if (time - speed) * speed > distance {
            wins += 1;
        }
    }
    wins
}

fn parse_b(input: &str) -> (usize, usize) {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"\
            Time:      7  15   30
            Distance:  9  40  200
            "};

    #[test]
    fn example_1() {
        assert_eq!(solve_a(EXAMPLE), 288);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 252_000);
    }

    #[test]
    fn test_parse_b() {
        assert_eq!(parse_b(EXAMPLE), (71530, 940200));
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 36992486);
    }
}
