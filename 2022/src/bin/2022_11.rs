//! https://adventofcode.com/2022/day/

use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/11.txt").unwrap()
}

#[derive(Debug)]
enum Op {
    Add(isize),
    Mul(isize),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<isize>,
    op: Op,
    divisor: isize,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut ms = Vec::new();
    for (i, chk) in input.lines().collect_vec().chunks(7).enumerate() {
        assert_eq!(chk[0], &format!("Monkey {i}:"));
        let items = chk[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<VecDeque<isize>>();
        let op;
        if let Some(x) = chk[2].strip_prefix("  Operation: new = old + ") {
            op = Op::Add(x.parse().unwrap());
        } else if chk[2] == "  Operation: new = old * old" {
            op = Op::Square;
        } else if let Some(x) = chk[2].strip_prefix("  Operation: new = old * ") {
            op = Op::Mul(x.parse().unwrap());
        } else {
            panic!();
        }
        let divisor = chk[3]
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let if_true = chk[4]
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let if_false = chk[5]
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        assert!(chk.len() == 6 || chk[6].is_empty());

        ms.push(Monkey {
            items,
            op,
            divisor,
            if_true,
            if_false,
            inspected: 0,
        });
    }
    ms
}

fn solve_a(input: &str) -> usize {
    let mut ms = parse(input);
    for _round in 0..20 {
        for im in 0..(ms.len()) {
            // Reference by index, and keep using the index, because
            // we need to read and write different elements of the array
            // from inside the one iteration.
            while let Some(mut item) = ms[im].items.pop_front() {
                match ms[im].op {
                    Op::Add(a) => item += a,
                    Op::Mul(m) => item *= m,
                    Op::Square => item *= item,
                }
                item /= 3;
                let dest = if item % ms[im].divisor == 0 {
                    ms[im].if_true
                } else {
                    ms[im].if_false
                };
                ms[im].inspected += 1;
                ms[dest].items.push_back(item);
            }
        }
    }
    monkey_business(&ms)
}

fn solve_b(input: &str) -> usize {
    let mut ms = parse(input);
    let modul: isize = ms.iter().map(|m| m.divisor).product();
    for _round in 0..10000 {
        for im in 0..(ms.len()) {
            while let Some(mut item) = ms[im].items.pop_front() {
                match ms[im].op {
                    Op::Add(a) => item += a,
                    Op::Mul(m) => item *= m,
                    Op::Square => item *= item,
                }
                item %= modul;
                let dest = if item % ms[im].divisor == 0 {
                    ms[im].if_true
                } else {
                    ms[im].if_false
                };
                ms[im].inspected += 1;
                ms[dest].items.push_back(item);
            }
        }
    }
    monkey_business(&ms)
}

fn monkey_business(monkeys: &[Monkey]) -> usize {
    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        assert_eq!(
            solve_a(
                "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"
            ),
            10605
        );
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 56120);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 24389045529);
    }
}
