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
    game_matches(input)
        .map(|hits| if hits > 0 { 1 << (hits - 1) } else { 0 })
        .sum()
}

fn solve_b(input: &str) -> usize {
    let ncards = input.lines().count();
    let mut copies = vec![1; ncards];
    for (i, n) in game_matches(input).enumerate() {
        for j in 0..n {
            if (i + j + 1) < ncards {
                copies[i + j + 1] += copies[i];
            }
        }
    }
    copies.iter().sum()
}

/// Given text input describing the games, return a list of the number of matches in
/// each game.
fn game_matches(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|l| {
        let (win, have) = l
            .split_once(": ")
            .expect("Remove card prefix")
            .1
            .split(" | ")
            .map(|a| {
                a.split_ascii_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect_tuple()
            .expect("Collect 2 parts");
        have.iter().filter(|x| win.contains(x)).count()
    })
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
