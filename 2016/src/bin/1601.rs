//! https://adventofcode.com/2016/day/1

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Dir {
    Left,
    Right,
}
use std::collections::HashSet;

use Dir::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Heading(isize);

impl Heading {
    fn turn(&self, dir: Dir) -> Heading {
        let rh = match dir {
            Left => -1,
            Right => 1,
        };
        let nd = (self.0 + rh).rem_euclid(4);
        assert!((0..=3).contains(&nd));
        Heading(nd)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction(Dir, usize);

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Position(isize, isize);

impl Position {
    fn manhattan_distance(&self) -> usize {
        self.0.unsigned_abs() + self.1.unsigned_abs()
    }
}

fn parse(s: &str) -> Vec<Instruction> {
    let mut r = Vec::new();
    let mut chars = s.chars();
    loop {
        let dir: Dir = match chars.next() {
            None => break,
            Some('R') => Right,
            Some('L') => Left,
            Some(other) => panic!("Unexpected character {:#?}", other),
        };
        let mut len: Option<usize> = None;
        loop {
            let d = match chars.next() {
                Some(d) if d.is_ascii_digit() => d,
                None | Some(',') | Some('\n') => break,
                Some(other) => panic!("Unexpected non-digit {:#?}", other),
            };
            len = Some(len.unwrap_or_default() * 10 + d.to_digit(10).unwrap() as usize);
        }
        r.push(Instruction(dir, len.expect("Empty length field")));
        match chars.next() {
            Some(' ') | Some('\n') => (),
            None => break,
            Some(other) => panic!("Unexpected character {:#?}", other),
        }
    }
    r
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
    heading: Heading,
}

impl Walker {
    /// Follow the given direction; returns a vec of all positions visited.
    fn walk(&mut self, &Instruction(dir, len): &Instruction) -> Vec<Position> {
        let mut visited = Vec::new();
        let mut p = self.pos;
        self.heading = self.heading.turn(dir);
        for _ in 0..len {
            p = match self.heading.0 {
                0 => Position(p.0, p.1 + 1),
                1 => Position(p.0 + 1, p.1),
                2 => Position(p.0, p.1 - 1),
                3 => Position(p.0 - 1, p.1),
                _ => panic!("unexpected heading"),
            };
            visited.push(p);
        }
        self.pos = p;
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
        use super::{Dir::*, Instruction};

        assert_eq!(
            parse("R1, L123, R99\n"),
            vec![
                Instruction(Right, 1),
                Instruction(Left, 123),
                Instruction(Right, 99)
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
