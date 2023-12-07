use std::cmp::Ordering;
use std::fs::read_to_string;

use itertools::Itertools;

static YEAR: &str = "2023";
static DAY: &str = "07";

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
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|(hand, bid)| {
            (
                hand.chars().map(card).collect_vec(),
                bid.parse::<usize>().unwrap(),
            )
        })
        .sorted_by(|a, b| compare(&a.0, &b.0))
        .enumerate()
        .map(|(i, play)| (i + 1) * play.1)
        .sum()
}

fn compare(a: &[usize], b: &[usize]) -> Ordering {
    categorize(a).cmp(&categorize(b)).then(a.cmp(b))
}

fn categorize(cards: &[usize]) -> usize {
    let mut counts = [0; 15];
    for c in cards {
        counts[*c] += 1;
    }
    debug_assert_eq!(counts.iter().sum::<usize>(), 5);
    let dist = counts
        .into_iter()
        .filter(|c| *c > 0)
        .sorted()
        .rev()
        .collect_vec();
    debug_assert_eq!(dist.iter().sum::<usize>(), 5);
    if dist.len() == 1 {
        6 // 5 of a kind
    } else if dist == [4, 1] {
        5
    } else if dist == [3, 2] {
        4
    } else if dist == [3, 1, 1] {
        3
    } else if dist == [2, 2, 1] {
        2
    } else if dist == [2, 1, 1, 1] {
        1
    } else if dist == [1, 1, 1, 1, 1] {
        0
    } else {
        unreachable!()
    }
}

fn card(c: char) -> usize {
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        '2'..='9' => c.to_digit(10).unwrap() as usize,
        _ => panic!(),
    }
}

fn solve_b(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|(hand, bid)| {
            (
                hand.chars().map(card_value_b).collect_vec(),
                bid.parse::<usize>().unwrap(),
            )
        })
        .sorted_by(|a, b| cmp_b(&a.0, &b.0))
        .enumerate()
        .map(|(idx, (_hand, bid))| (idx + 1) * bid)
        .sum()
}

fn cmp_b(a: &[usize], b: &[usize]) -> Ordering {
    categorize_b(a).cmp(&categorize_b(b)).then(a.cmp(b))
}

fn card_value_b(c: char) -> usize {
    match c {
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        '2'..='9' => c.to_digit(10).unwrap() as usize,
        'J' => 1,
        _ => panic!(),
    }
}

fn categorize_b(hand: &[usize]) -> usize {
    let mut counts = [0; 15];
    for c in hand {
        counts[*c] += 1;
    }
    let num_j = counts[1];
    debug_assert_eq!(counts.iter().sum::<usize>(), 5);
    // distribution of cards excluding Js
    let dist = counts
        .into_iter()
        .skip(2) // skip 0 and 1
        .filter(|c| *c > 0)
        .sorted()
        .rev()
        .collect_vec();
    debug_assert_eq!(dist.iter().sum::<usize>() + num_j, 5);
    match &dist[..] {
        [] | [_] => 6,                    // 5 of a kind, or can be made with jokers
        [_, 1] => 5,                      // can make 4 of a kind
        [_, 2] => 4,                      // can make 3 of a kind
        [_, 1, 1] => 3,                   // make full house
        [2, 2, 1] => 2,                   // two pair
        [2, 1, 1, 1] | [1, 1, 1, 1] => 1, // one pair
        [1, 1, 1, 1, 1] => 0,             // high card
        _ => unreachable!("Can't handle {hand:?} dist={dist:?}"),
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! { "\
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        "};

    #[test]
    fn example_1() {
        assert_eq!(solve_a(EXAMPLE), 6440);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_b(EXAMPLE), 5905);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 245794640);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 247899149);
    }
}
