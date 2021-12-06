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
    let mut fish: Vec<usize> = input
        .trim()
        .split(',')
        .map(|w| w.parse().unwrap())
        .collect();
    for _day in 0..80 {
        for j in 0..fish.len() {
            let x = fish[j];
            if x == 0 {
                fish[j] = 6;
                fish.push(8);
            } else {
                fish[j] -= 1;
            }
        }
    }
    fish.len()
}

fn solve_b(input: &str) -> usize {
    let fish: Vec<usize> = input
        .trim()
        .split(',')
        .map(|w| w.parse().unwrap())
        .collect();
    let mut per_cycle = vec![0usize; 9];
    for c in fish {
        per_cycle[c] += 1;
    }
    for _day in 0..256 {
        dbg!(&per_cycle);
        let mut n = vec![0usize; 9];
        for i in 1..=8 {
            n[i - 1] = per_cycle[i];
        }
        n[6] += per_cycle[0];
        n[8] += per_cycle[0];
        per_cycle = n;
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
