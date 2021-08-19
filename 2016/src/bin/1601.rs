//! https://adventofcode.com/2016/day/1

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    turn: char,
    len: usize,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Position(isize, isize);

impl Position {
    fn manhattan_distance(&self) -> usize {
        self.0.unsigned_abs() + self.1.unsigned_abs()
    }
}

fn parse(s: &str) -> Vec<Instruction> {
    s.trim_end()
        .split(", ")
        .map(|p| {
            let turn: char = p.chars().next().expect("no direction character");
            assert!(turn == 'R' || turn == 'L');
            let len = p.split_at(1).1.parse().expect("parse length");
            Instruction { turn, len }
        })
        .collect()
}

fn solve_a() -> usize {
    solve_type_a(&load())
}

fn solve_b() -> usize {
    solve_type_b(&load())
}

fn load() -> String {
    std::fs::read_to_string("input/1601.txt").unwrap()
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Walker {
    pos: Position,
    heading: isize,
}

impl Walker {
    /// Follow the given direction; returns a vec of all positions visited.
    fn walk(&mut self, &Instruction { turn, len }: &Instruction) -> Vec<Position> {
        let mut visited = Vec::new();
        let rh = match turn {
            'L' => -1,
            'R' => 1,
            _ => panic!("unexpected turn"),
        };
        self.heading = (self.heading + rh).rem_euclid(4);
        assert!((0..=3).contains(&self.heading));
        for _ in 0..len {
            self.pos = match self.heading {
                0 => Position(self.pos.0, self.pos.1 + 1),
                1 => Position(self.pos.0 + 1, self.pos.1),
                2 => Position(self.pos.0, self.pos.1 - 1),
                3 => Position(self.pos.0 - 1, self.pos.1),
                _ => panic!("unexpected heading"),
            };
            visited.push(self.pos);
        }
        visited
    }
}

fn solve_type_a(input: &str) -> usize {
    let mut walker = Walker::default();
    for inst in parse(input) {
        walker.walk(&inst);
    }
    walker.pos.manhattan_distance()
}

fn solve_type_b(input: &str) -> usize {
    let mut walker = Walker::default();
    let mut seen = HashSet::new();
    seen.insert(walker.pos);
    for p in parse(input).iter().flat_map(|inst| walker.walk(inst)) {
        if !seen.insert(p) {
            return p.manhattan_distance();
        }
    }
    panic!("Reached no square twice");
}

pub fn main() {
    println!("1601a: {}", solve_a());
    println!("1601b: {}", solve_b());
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        use super::parse;
        use super::Instruction;

        assert_eq!(
            parse("R1, L123, R99\n"),
            vec![
                Instruction { turn: 'R', len: 1 },
                Instruction {
                    turn: 'L',
                    len: 123
                },
                Instruction { turn: 'R', len: 99 },
            ]
        )
    }

    #[test]
    fn examples_a() {
        use super::solve_type_a;
        assert_eq!(solve_type_a("R2, L3"), 5);
    }

    #[test]
    fn solution_a() {
        assert_eq!(super::solve_a(), 246);
    }

    #[test]
    fn solution_b() {
        assert_eq!(super::solve_b(), 124);
    }
}
