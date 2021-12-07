// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/7

fn main() {
    let input = input();
    println!("{:?}", solve_a(&input));
    println!("{:?}", solve_b(&input));
}

fn input() -> String {
    std::fs::read_to_string("input/07.txt").unwrap()
}

fn solve_a(input: &str) -> i64 {
    let ps = input
        .trim()
        .split(',')
        .map(|w| w.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut best = i64::MAX;
    for i in 1..(*ps.iter().max().unwrap()) {
        let guess = ps.iter().map(|x| (x - i).abs()).sum();
        best = std::cmp::min(best, guess);
    }
    best
}

fn solve_b(input: &str) -> i64 {
    let ps = input
        .trim()
        .split(',')
        .map(|w| w.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut best = i64::MAX;
    for i in 1..(*ps.iter().max().unwrap()) {
        let guess = ps
            .iter()
            .map(|x| {
                let d = (x - i).abs();
                (d * (d + 1)) / 2
            })
            .sum();
        best = std::cmp::min(best, guess);
    }
    best
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(solve_a(&input()), 340056);
        assert_eq!(solve_b(&input()), 96592275);
    }
}
