#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}
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

#[derive(Debug, PartialEq, Eq, Default)]
struct Position(isize, isize);

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
    solve_type_a(&std::fs::read_to_string("input/1601.txt").unwrap())
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Walker {
    pos: Position,
    heading: Heading,
}

impl Walker {
    fn walk(&mut self, Instruction(dir, l): Instruction)  {
        let Position(x, y) = self.pos;
        let l = l as isize;
        self.heading = self.heading.turn(dir);
        self.pos = match self.heading.0 {
            0 => Position(x, y + l),
            1 => Position(x + l, y),
            2 => Position(x, y - l),
            3 => Position(x - l, y),
            _ => panic!("unexpected heading"),
        };
    }
}

fn solve_type_a(input: &str) -> usize {
    let mut walker = Walker::default();
    for inst in parse(input) {
        walker.walk(inst);
    }
    let pos = walker.pos;
    pos.0.unsigned_abs() + pos.1.unsigned_abs()
}

// fn solve_type_b(input: &str) -> usize {
//     todo!();
// }

pub fn main() {
    println!("{}", solve_a());
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
}
