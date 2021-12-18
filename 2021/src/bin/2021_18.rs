// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/16

#![allow(clippy::comparison_chain)] // bad warning; it's slower and no simpler
#![allow(unused_imports)]
use std::cmp::max;
use std::collections::{BTreeMap, BTreeSet};
use std::os::unix::prelude::OsStrExt;

use aoclib::{point, Matrix, Point};

#[derive(Debug, PartialEq, Clone, Eq)]
enum E {
    Open,
    Close,
    N(u32),
}

impl E {
    fn addeq(&mut self, v: u32) {
        match self {
            E::N(a) => *a += v,
            _ => panic!(),
        }
    }

    fn val(&self) -> u32 {
        match self {
            E::N(a) => *a,
            _ => panic!(),
        }
    }
}

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/18.txt").unwrap()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Num {
    v: Vec<E>,
}

fn parse(s: &str) -> Num {
    let mut v = Vec::new();
    for c in s.chars() {
        match c {
            '[' => v.push(E::Open),
            ']' => v.push(E::Close),
            ',' => (),
            '0'..='9' => v.push(E::N(c.to_digit(10).unwrap())),
            _ => panic!("unexpected {:?}", c),
        }
    }
    Num { v }
}

impl Num {
    #[cfg(test)]
    fn to_str(&self) -> String {
        let mut s = String::new();
        for e in &self.v {
            match e {
                E::Open => {
                    if !s.is_empty() && !s.ends_with('[') {
                        s.push(',');
                    }
                    s.push('[')
                }
                E::Close => s.push(']'),
                E::N(v) => {
                    if !s.ends_with('[') {
                        s.push(',');
                    }
                    s.push_str(&v.to_string());
                }
            }
        }
        s
    }

    fn reduce(mut self) -> Num {
        while self.explode1() || self.split1() {}
        self
    }

    fn explode1(&mut self) -> bool {
        let mut depth = 0;
        let v = &mut self.v;
        for (i, e) in v.iter().enumerate() {
            match e {
                E::Open => depth += 1,
                E::Close => depth -= 1,
                _ => (),
            }
            if depth == 5 {
                println!("explode {:?}", &v[i..(i + 4)]);
                assert!(matches!(
                    &v[i..(i + 4)],
                    [E::Open, E::N(_), E::N(_), E::Close]
                ));
                let a = v[i + 1].val();
                let b = v[i + 2].val();

                if let Some(left) = v[..i]
                    .iter()
                    .enumerate()
                    .rev()
                    .filter(|(_i, e)| matches!(e, E::N(_)))
                    .map(|(i, _)| i)
                    .next()
                {
                    v[left].addeq(a);
                }
                if let Some(right) = v
                    .iter()
                    .enumerate()
                    .skip(i + 4)
                    .filter(|(_, e)| matches!(e, E::N(_)))
                    .map(|(i, _)| i)
                    .next()
                {
                    v[right].addeq(b);
                }
                for _ in 0..3 {
                    v.remove(i);
                }
                v[i] = E::N(0);
                return true;
            }
        }
        false
    }

    fn split1(&mut self) -> bool {
        for (i, e) in self.v.iter().enumerate() {
            match e {
                &E::N(a) if a >= 10 => {
                    self.v[i] = E::Close;
                    self.v.insert(i, E::N((a + 1) / 2));
                    self.v.insert(i, E::N((a) / 2));
                    self.v.insert(i, E::Open);
                    return true;
                }
                _ => (),
            }
        }
        false
    }

    fn add(&self, other: &Num) -> Num {
        let mut v = vec![E::Open];
        v.extend(self.v.iter().cloned());
        v.extend(other.v.iter().cloned());
        v.push(E::Close);
        Num { v }.reduce()
    }

    fn magnitude(&self) -> u32 {
        let mut st: Vec<Vec<u32>> = vec![vec![]];
        for e in &self.v {
            match e {
                E::Open => st.push(vec![]),
                E::N(a) => st.last_mut().unwrap().push(*a),
                E::Close => {
                    let l = st.pop().unwrap();
                    assert_eq!(l.len(), 2);
                    let m = l[0] * 3 + l[1] * 2;
                    st.last_mut().unwrap().push(m);
                }
            }
        }
        assert_eq!(st.len(), 1);
        let l = st.pop().unwrap();
        assert_eq!(l.len(), 1);
        l[0]
    }
}

fn solve(input: &str) -> (u32, u32) {
    let nums: Vec<Num> = input.lines().map(parse).collect();
    let sol_a = nums
        .clone()
        .into_iter()
        .reduce(|a, b| a.add(&b))
        .unwrap()
        .magnitude();

    let mut sol_b = 0;
    for na in &nums {
        for nb in &nums {
            if na != nb {
                sol_b = max(sol_b, na.add(nb).magnitude());
            }
        }
    }

    (sol_a, sol_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn syntax() {
        let ex = "[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        for l in ex.lines() {
            let _num = parse(l);
        }
    }

    #[test]
    fn explode() {
        let mut num = parse("[[[[[9,8],1],2],3],4]");
        assert!(num.explode1());
        assert_eq!(num.to_str(), "[[[[0,9],2],3],4]");

        for (before, after) in [
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ] {
            let mut num = parse(before);
            assert!(num.explode1());
            assert_eq!(num.to_str(), after);
        }
    }

    #[test]
    fn ex_add() {
        assert_eq!(
            parse("[[[[4,3],4],4],[7,[[8,4],9]]]")
                .add(&parse("[1,1]"))
                .to_str(),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }

    #[test]
    fn ex_magn() {
        for (s, m) in [("[[9,1],[1,9]]", 129)] {
            assert_eq!(parse(s).magnitude(), m);
        }
    }

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 3494);
        assert_eq!(b, 4712);
    }
}
