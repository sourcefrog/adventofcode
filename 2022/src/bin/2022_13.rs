//! https://adventofcode.com/2022/day/13

use std::cmp::Ordering;

use itertools::Itertools;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/13.txt").unwrap()
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Item {
    N(usize),
    L(Vec<Item>),
}

impl Item {
    fn parse(s: &str) -> (Item, &str) {
        if let Some(mut rest) = s.strip_prefix('[') {
            let mut l = Vec::new();
            loop {
                if let Some(r3) = rest.strip_prefix(']') {
                    break (Item::L(l), r3);
                }
                let (li, r2) = Item::parse(rest);
                l.push(li);
                if let Some(r3) = r2.strip_prefix(']') {
                    break (Item::L(l), r3);
                } else if let Some(r3) = r2.strip_prefix(',') {
                    rest = r3;
                    continue;
                } else {
                    unreachable!("unexpected {r2:?}")
                }
            }
        } else {
            let end = s.find(|c| matches!(c, ',' | '[' | ']')).unwrap_or(s.len());
            let (n, r2) = s.split_at(end);
            (Item::N(n.parse().unwrap()), r2)
        }
    }

    fn parse_all(s: &str) -> Item {
        let (i, rest) = Item::parse(s);
        assert!(rest.is_empty());
        i
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::N(a), Item::N(b)) => a.cmp(b),
            (Item::L(a), Item::L(b)) => {
                for (x, y) in a.iter().zip(b) {
                    let c = x.cmp(y);
                    if c != Ordering::Equal {
                        return c;
                    }
                }
                a.len().cmp(&b.len())
            }
            (Item::L(_), Item::N(_)) => self.cmp(&Item::L(vec![other.clone()])),
            (Item::N(_), Item::L(_)) => Item::L(vec![self.clone()]).cmp(other),
        }
    }
}

fn solve_a(input: &str) -> usize {
    let mut sum = 0;
    for (i, group) in (1..).zip(input.lines().collect::<Vec<&str>>().chunks(3)) {
        let a = group[0];
        let b = group[1];
        let ia = Item::parse(a);
        let ib = Item::parse(b);
        if ia < ib {
            // println!("{i} in right order");
            sum += i
        }
    }
    sum
}

fn solve_b(input: &str) -> usize {
    let dividers = ["[[2]]", "[[6]]"]
        .into_iter()
        .map(Item::parse_all)
        .collect_vec();
    let ps = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Item::parse_all)
        .chain(dividers.clone())
        .sorted()
        .collect_vec();
    dividers
        .iter()
        .map(|d| ps.iter().position(|q| *q == *d).unwrap() + 1)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn examples_a() {
        // 1
        assert!(Item::parse("[1,1,3,1,1]") < Item::parse("[1,1,5,1,1]"));
        // 8
        assert!(
            Item::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]") > Item::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]")
        );
    }

    #[test]
    fn big_example_a() {
        assert_eq!(solve_a(EXAMPLE), 13);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 140);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 5659);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 22110);
    }
}
