// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/6

fn main() {
    let input = input();
    println!("{:?}", solve_a(&input));
    println!("{:?}", solve_b(&input));
}

fn input() -> String {
    std::fs::read_to_string("input/06.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    solve(input, 80)
}

fn solve_b(input: &str) -> usize {
    solve(input, 256)
}

fn solve(input: &str, days: usize) -> usize {
    let mut per_cycle = [0usize; 9]; // Number of fish per age.
    input
        .trim()
        .split(',')
        .map(|w| w.parse::<usize>().unwrap())
        .for_each(|c| per_cycle[c] += 1);
    for _day in 0..days {
        let mut next = [0usize; 9];
        next[0..=7].copy_from_slice(&per_cycle[1..=8]);
        next[6] += per_cycle[0];
        next[8] = per_cycle[0];
        per_cycle = next;
    }
    per_cycle.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(solve_a(&input()), 389726);
        assert_eq!(solve_b(&input()), 1743335992042);
    }
}
